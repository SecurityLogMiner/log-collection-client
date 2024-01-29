mod config;
mod producer;
mod firehosesdk;
mod dynamosdk;

use producer::start_log_stream;
use config::read_config;
use std::{env, process};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {

        if args[1] == "--help" || args[1] == "-h"{
            print_help();
            process::exit(0);
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
                    _ => println!("Invalid destination"),
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

fn print_help() {
    println!("Usage: cargo run -- <destination>");
    println!("Available Destinations:");
    println!("  dynamodb       Create DynamoDB table");
    println!("  kdf            Send logs to Kinesis Firehose");
    println!("  s3             Send logs to S3 bucket");
    println!("  elastic        Send logs to Elastic");
}