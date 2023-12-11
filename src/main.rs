
use std::env;
mod producer;
use crate::producer::{run_tail_f};
mod config;
use config::{read_config};
use std::net::TcpStream;
use std::io::{Read, Write};

fn main() {
    let mut config_data;
    /*
    match env::args().len() {
        1 => {
            config_data = read_config();
            let tail_output = run_tail_f(config_data.unwrap()[0].to_string());
            println!("{}", String::from(&tail_output));
        }
        _ => {println!("handle command line usage");}
    }
    */

    let mut stream = TcpStream::connect("127.0.0.1:44331").expect("failed to connect to server");
    let message = "Hello, server!";
    stream.write_all(message.as_bytes()).expect("Failed to send data to server");
}
