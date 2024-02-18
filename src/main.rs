mod traits;
mod config;
mod producer;
mod dynamosdk;
mod util;
mod iam;

use aws_config::imds::Client;
use producer::start_log_stream;
use config::read_config;
use std::{env, process};
use util::{print_help, send_logs_to_all_destinations};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 2 {
        let config_data = read_config();

        match config_data {
            Some(config) => {
                if args.len() == 1 {
                util::print_help().await;
            }
                if args.len() == 2 {
                    if args[1] == "--help" || args[1] == "-h" {
                        util::print_help().await;
                    }

                    let destination = args[1].as_str();
                    println!("Destination: {}", destination);
                    match destination {
                        "dynamodb" => {
                            dynamosdk::send_dynamodb(config).await;
                        }
                        "elastic" => {
                            todo!();
                        }
                        "iam" => {
                            util::initialize_iam(config).await;
                        }
                        "run-admin" => {
                            // util::initialize_iam(config).await;
                            util::run_admin_cli().await;
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

