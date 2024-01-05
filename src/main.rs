use std::env;
mod producer;
use crate::producer::{create_log_stream};
mod config;
use config::{read_config, Config};

fn main() {
    let mut config_data;

    match env::args().len() {
        1 => {
            // for now, config file is hard coded in read_config.
            config_data = read_config();
            match config_data {
                Some(config) => {
                    println!("{:?}", config);
                    let _ = create_log_stream(config);
                }
                None => panic!("error reading configuration. fix it.")
            }
            /*
            if let Err(err) = create_log_stream(
                    config_data.unwrap()[0].to_string()) {
                    eprintln!("Error: {}", err);
                }
            */
        }
        _ => {println!("handle command line usage");}
    }
}
