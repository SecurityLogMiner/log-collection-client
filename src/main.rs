
use std::env;
mod producer;
use crate::producer::{create_log_stream};
mod config;
use config::{read_config};
use std::net::TcpStream;
use std::io::{Read, Write};

fn main() {
    let mut config_data;
    match env::args().len() {
        1 => {
            config_data = read_config();
            println!("{:?}",config_data);
            if let Err(err) = create_log_stream(
                config_data.unwrap()[0].to_string()) {
                eprintln!("Error: {}", err);
            }
        }
        _ => {println!("handle command line usage");}
    }
}
