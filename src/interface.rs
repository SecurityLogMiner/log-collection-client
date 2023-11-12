use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::Parser;

#[derive(Parser,Debug)]
struct Args {
    config: String,
}

#[derive(Debug)]
pub struct Config {
    server_address: String,
    server_port: u16,
    log_file_path: String,
    credentials: String, // TLS needed
}

// the user could provide a path to a different config or simply get help with using the command
pub fn 
command_line() -> Result<Config, Box<dyn std::error::Error>>{
    let args = Args::try_parse();
    println!("{:?}",args);
    Ok(Config {
        server_address: String::from("server address"),
        server_port: 123,
        log_file_path: String::from("path to log file"),
        credentials: String::from("credentials"),
    })
}

pub fn
read_config() -> Result<u8, Box<dyn std::error::Error>> {
    let file = File::open("test.config")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(0)
}
