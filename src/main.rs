use std::env;
mod producer;
//use crate::producer::{run_tail_f};

mod config;
use config::{read_config};

fn main() {
    match env::args().len() {
        1 => {let _ = read_config();}
        _ => {println!("handle command line usage");}
    }

    match establish_connection(&config.db_host, config.db_port) {           //This should work for the database connection
        Ok(_) => println!("Connection established successfully"),
        Err(err) => eprintln!("Error establishing connection: {}", err),
    }

    match listen_for_connections(config.server_port) {
        Ok(_) => println!("Listening for incoming connections"),
        Err(err) => eprintln!("Error listening for connections: {}", err),
    }
}

