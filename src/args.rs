use clap::{App, Arg};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::string::String;
use std::time::Duration;

pub struct Args {
    pub threads: u8,
    pub ports: Vec<u32>,
    pub search_text: Vec<String>,
    pub output_file: String,
    pub timeout: Duration,
    pub hostname: String
}

impl Args {
    pub fn new() -> Self {
        let matches = App::new("gscan")
            .version("0.1")
            .author("Matt Dizak <matt@apexpl.io>")
            .about("Find the IPs you're looking for!.")
            .arg(Arg::with_name("threads")
                .short('t')
                .long("threads")
                .takes_value(true)
                .help("Number of threads to utilize.  Optional, and if not included will utilize all threads available on the machine."))
            .arg(Arg::with_name("port")
                .short('p')
                .long("port")
                .takes_value(true)
                .help("Comma delimited list of ports to check.  If 443 is defined, will connect over SSL."))
            .arg(Arg::with_name("file")
                .short('f')
                .long("file")
                .takes_value(true)
                .help("File containing lines of text to search sites for.  Defaults to search.txt."))
            .arg(Arg::with_name("output")
                .short('o')
                .long("output")
                .takes_value(true)
                .help("Output file to place any successful results.  Defaults to gscan.log."))
            .arg(Arg::with_name("seconds")
                .short('s')
                .long("seconds")
                .takes_value(true)
                .help("Seconds to timeout upon checking TCP connection.  Defaults to 3."))
            .arg(Arg::with_name("hostname")
                .short('h')
                .long("hostname")
                .takes_value(true)
                .help("Value of the Host: field to include in each HTTP request."))
        .get_matches();

        // Get args
        let threads = matches.value_of("threads").unwrap_or("0");
        let ports_line = matches.value_of("port").unwrap_or("80");
        let search_file = matches.value_of("file").unwrap_or("search.txt");
        let output_file = matches.value_of("file").unwrap_or("gscan.log");
        let seconds = matches.value_of("seconds").unwrap_or("10");
        let hostname = matches.value_of("hostname").unwrap_or("");

        // Get ports
        let ports: Vec<u32> = ports_line
            .split(',')
            .into_iter()
            .map(|v| v.parse::<u32>().unwrap())
            .collect();

        // Return
        Self {
            threads: threads.parse::<u8>().unwrap(),
            ports,
            search_text: Args::read_search_file(&search_file.to_string()),
            output_file: output_file.to_string(),
            timeout: Duration::from_secs(seconds.parse::<u64>().unwrap()),
        hostname: hostname.to_string()
        }
    }

    pub fn read_search_file(filename: &String) -> Vec<String> {
        // Ensure file exists
        if !Path::new(&filename).exists() {
            panic!("No search file exists at {}", filename);
        }

        // Set variables
        let path = Path::new(&filename);
        let mut search_text: Vec<String> = Vec::new();

        // Open file
        let file = File::open(path).expect("Unable to open search.txt file");
        let lines = BufReader::new(file).lines();
        for line in lines {
            let search_line = line.unwrap().trim().to_string();
            if search_line.is_empty() {
                continue;
            }
            search_text.push(search_line);
        }

        // Return
        search_text
    }
}


impl Default for Args {

    fn default() -> Self {
        Self::new()
    }

}

