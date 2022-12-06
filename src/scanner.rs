use crate::ARGS;
use lazy_static::lazy_static;
use log::{info, warn};
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use reqwest::blocking::{Client, Response};
use std::fs::OpenOptions;
use std::io::Write;
use std::net::TcpStream;
use std::path::Path;
use std::string::String;

lazy_static! {
    pub static ref HTTP_CLIENT: Client = reqwest::blocking::Client::new();
}

pub fn scan() {
    // Set the threads
    if ARGS.threads > 0 {
        rayon::ThreadPoolBuilder::new()
            .num_threads(ARGS.threads as usize)
            .build_global()
            .expect("Unable to set number of threads");
    }

    // Grab an iterator
    let mut nums: Vec<u16> = Vec::new();
    for x in 0..256 {
        nums.push(x);
    }

    // Scan the internet, even if the loop is a little messy
    for first in 1..256 {
        // Second segment
        for second in 0..256 {
            // Third, but let's parallelize now
            nums.par_iter().for_each(|third| {
                check_ip_range(&first, &second, third);
            });
        }
    }
}

fn check_ip_range(first: &usize, second: &usize, third: &u16) {
    // Go through segments
    for fourth in 0..256 {
        let ip_address = format!("{}.{}.{}.{}", first, second, third, fourth);

        // Are we public?
        if !is_public(&ip_address) {
            continue;
        }

        // Check the ports
        check_ports(&ip_address);
    }
}

fn check_ports(ip_address: &String) {
    // Go through ports
    for port in &ARGS.ports {
        let hostname = format!("{}:{}", ip_address, port);

        // Are we open?
        match TcpStream::connect_timeout(&hostname.parse().unwrap(), ARGS.timeout) {
            Ok(r) => r,
            Err(_) => continue,
        };
        info!("Found open port at {}", hostname);

        // Check http
        let res = match check_http(&hostname) {
            Some(r) => r,
            None => return,
        };

        // Add results to output file
        for phrase in res.iter() {
            save_results(&hostname, &phrase.to_string());
        }
    }
}

fn is_public(ip: &String) -> bool {

    !(ip.starts_with("127.")
        || ip.starts_with("192.")
        || ip.starts_with("10.")
        || ip.starts_with("0.")
        || ip == "255.255.255.255")


}

fn check_http(hostname: &str) -> Option<Vec<String>> {
    // Are we on SSL?
    let mut prefix = "http://";
    if hostname.ends_with(":443") {
        prefix = "https://";
    }

    // Fire away
    let url = format!("{}{}/", prefix, hostname);
    let res: Response = match HTTP_CLIENT.get(&url)
        .header("Host", &ARGS.hostname)
        .header("Connection", "close")
        .send() {
        Ok(r) => r,
        Err(_) => {
            warn!("Didn't get a response from {}", hostname);
            return None;
        }
    };

    // Check for result
    let body: String = match res.text() {
        Ok(r) => r,
        Err(_) => {
            warn!("No body found from {}", hostname);
            return None;
        }
    };

    // Check the search criteria
    let results: Vec<String> = ARGS
        .search_text
        .iter()
        .map(|s| s.to_string())
        .filter(|c| body.contains(c))
        .collect();

    // Check for no results
    if results.is_empty() {
        return None;
    }

    // Return
    Some(results)
}

fn save_results(hostname: &str, criteria: &String) {
    // Open the file
    let path = Path::new(&ARGS.output_file);
    let mut fh = match OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&path)
    {
        Ok(r) => r,
        Err(e) => panic!(
            "Unable to open results file at {}, error: {}",
            ARGS.output_file, e
        ),
    };

    // Write line
    let line = format!("{} contains {}\n", hostname, criteria);
    info!("{}", line);
    fh.write_all(line.as_bytes())
        .expect("Unable to write to output file");

    // Close file
    drop(fh);
}
