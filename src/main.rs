mod config;
mod producer;
mod firehosesdk;
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

        let iam_client = iam::start_iam().await;
        

        match config_data {
            Some(config) => {
                if args.len() == 1 {

                //Create a setup functoin 
                // User gives IAM credentials; as long as they have correct policies; based on the policies set up on whatever they have available.
                // Attach policies to IAM user based on the set up function
                    let _ = start_log_stream(config.log_paths.clone(),"s3").await;
                }
                if args.len() == 2 {
                    if args[1] == "--help" || args[1] == "-h" {
                        util::print_help().await;
                        util::print_help().await;
                    }

                    let destination = args[1].as_str();
                    println!("Destination: {}", destination);
                    match destination {
                        "dynamodb" => {
                            util::send_dynamodb(config).await;
<<<<<<< HEAD
                            util::send_dynamodb(config).await;
=======
                            // Call the function to create DynamoDB table
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
                                            let _ = start_log_stream(config.log_paths.clone(),"dynamodb").await;
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
>>>>>>> 2ba48e2 (update)
                        }
                        "elastic" => {
                            todo!();
                        }

                        _ => {
                            util::print_help().await;
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

