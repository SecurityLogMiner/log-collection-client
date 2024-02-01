mod config;
mod producer;
mod firehosesdk;
mod dynamosdk;
mod util;

use producer::start_log_stream;
// util::{print_help, send_logs_to_all_destinations};
use config::read_config;
use std::{env, process};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {

        if args[1] == "--help" || args[1] == "-h"{
            util::print_help().await;
            process::exit(0);
        }

        if args[1] == "all" {
            // Handle sending logs to all destinations
            let config_data = config::read_config();
            match config_data {
                Some(config) => {
                    util::send_logs_to_all_destinations(config).await;
                }
                None => panic!("Error reading configuration. Fix it."),
            }
            return Ok(());
        }

        let config_data = read_config();
        match config_data {
            Some(config) => {
                let destination = args[1].as_str();
                println!("Destination: {}", destination);
                    match destination {
                        "kdf" => {
                            // Call the function to start log stream to 
                            let _ = start_log_stream(config).await;
                        }
                        "dynamodb" => {
                            // Call the function to create DynamoDB table
                            println!("Creating DynamoDB table...");
                            if let Ok(client) = dynamosdk::start_dynamodb().await {
                                if let Ok(res) = dynamosdk::create_table(&client, "eptesttable", "epkeyitem").await {
                                    println!("{:?}", res);
                                }
                            }
                        }
                        "s3" =>{
                            // todo
                            println!("Inserting data into S3 bucket");
                        }
                        "elastic" => {
                            //todo
                            println!("Sending data to Elastic Stack");
                        }
                    _ => println!("Invalid destination: Use cargo run -- --help"),
                }
            }
            None => panic!("Error reading configuration. Fix it."),
        }
    } else {
        println!("Usage: cargo run -- <destination>");
        println!("For more information: cargo run -- --help");
    }

    Ok(())
}

