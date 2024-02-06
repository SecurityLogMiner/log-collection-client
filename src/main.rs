mod config;
mod producer;
mod dynamosdk;

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
                    let _ = start_log_stream(config.log_paths.clone()).await;
                }
                if args.len() == 2 {
                    if args[1] == "--help" || args[1] == "-h" {
                        print_help();
                    }
                    let destination = args[1].as_str();
                    println!("Destination: {}", destination);
                    match destination {
                        "dynamodb" => {
                            // get the client
                            let dynamoclient = dynamosdk::start_dynamodb().await;
                            match dynamoclient {
                                Ok(client) => {
                                    // check if the table listed in the configuration file
                                    // exists. If it does not, create it. 
                                    println!("{:?}",&config);
                                    let tables = client.list_tables()
                                                        .into_paginator()
                                                        .items()
                                                        .send(); 
                                    let table_names = tables.collect::<Result<Vec<_>,_>>().await.unwrap();
                                    for tbl in table_names {
                                        if tbl == config.dynamo_table_name {
                                            println!("found {tbl:?}");
                                            // use the table
                                            let _ = start_log_stream(config.log_paths.clone()).await;
                                        }
                                    } 
                                    if let Ok(table) = dynamosdk::create_table(&client,
                                                            "default_table",
                                                            "default_key").await {
                                        println!("{table:?}");
                                    }
                                },
                                Err(_) => todo!(),
                            }
                        }
                        "elastic" => {
                            todo!();
                        }
                        _ => {
                            print_help();
                        }
                    }
                }
            }
            None => panic!("Error reading configuration. Fix it."),
        }
    } 

    Ok(())
}

fn print_help() {
    println!("Usage: cargo run -- <destination>");
    println!("Available Destinations:");
    println!("  dynamodb        Create DynamoDB table");
    println!("  opensearch      Create DynamoDB table");
    println!("  elastic (todo)  Send logs to Elastic");
    process::exit(0);
}
