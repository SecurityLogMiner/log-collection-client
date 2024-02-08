mod traits;
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

    if args.len() <= 2 {
        let config_data = read_config();
        match config_data {
            Some(config) => {
                if args.len() == 1 {
                    // the default might end up being to a custom http endpoint.
                    todo!();
                }
                if args.len() == 2 {
                    if args[1] == "--help" || args[1] == "-h" {
                        print_help();
                    }
                    let destination = args[1].as_str();
                    match destination {
                        "dynamodb" => {
                            // get the client
                            let dynamoclient = dynamosdk::create_client().await;
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
                                            let _ = start_log_stream(config.log_paths.clone(),
                                                        &client).await;
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
                        "opensearch" => {
                            let client = firehosesdk::create_client().await;
                            match client {
                                Ok(c) => {
                                    let stream_list = c.list_delivery_streams().send().await;
                                    if let Ok(stream) = stream_list {
                                        for name in stream.delivery_stream_names {
                                            if name == config.delivery_stream {
                                                println!("Using OpenSearch delivery stream: {name}");
                                            }
                                        }
                                    }
                                },
                                Err(_) => eprintln!("handle opensearch client error")
                            }
                        }
                        "elastic" => {
                            todo!();
                            //KDS-OPS-UMQ2c
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

