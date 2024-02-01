mod config;
mod producer;
mod firehosesdk;
mod dynamosdk;
mod util;

use producer::start_log_stream;
use config::read_config;
use std::{env, process};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    let args: Vec<String> = env::args().collect();

    if args.len() <= 2 {
        let config_data = read_config();
        match config_data {
            Some(config) => {
                if args.len() == 1 {
                    let _ = start_log_stream(config.log_paths.clone(),"s3").await;
                }
                if args.len() == 2 {
                    if args[1] == "--help" || args[1] == "-h" {
                        util::print_help().await;
                    }
                    let destination = args[1].as_str();
                    println!("Destination: {}", destination);
                    match destination {
                        "dynamodb" => {
                            util::send_dynamodb(config).await;
                        }
                        "elastic" => {
                            todo!();
                        }
                        _ => {
                            util::print_help().await;
                        }
                    }
                }
            }
            None => panic!("Error reading configuration. Fix it."),
        }
    } 

    Ok(())
}

