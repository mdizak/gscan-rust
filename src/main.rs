#![allow(warnings)]

use crate::args::Args;
use env_logger::{Builder, Target};
use lazy_static::lazy_static;
use log::LevelFilter;
use std::io::Write;

mod args;
mod scanner;

lazy_static! {
    pub static ref ARGS: Args = Args::new();
}

fn main() {
    // Init logger
    init_logger();

    // Scan
    scanner::scan();
}

fn init_logger() {
    // Get log level
    let log_level = LevelFilter::Info;

    // Init logger
    Builder::new()
        .format(|buf, record| writeln!(buf, "{}: {}", record.level(), record.args()))
        .filter(None, log_level)
        .target(Target::Stdout)
        .init();

    // Send greeting
    println!("-------------------");
    println!("-    Gscan -- Find the IPs you need!");
    println!("-    Developed By: Matt Dizak <matt@apexpl.io>");
    println!("-    Released: Dec 2022");
    println!("-------------------");
    println!("\n");
}
