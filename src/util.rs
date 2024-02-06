use aws_sdk_dynamodb::client;

// util.rs serves as housing various utility functions that are used in main.rs
use crate::config::Config;
use crate::producer::start_log_stream;
use crate::dynamosdk; // Import other modules as needed
use std::{env, process};
use crate::iam;

pub async fn send_logs_to_all_destinations(config: Config) {
    // Call the functions to send logs to all destinations
    // let _ = start_log_stream(config).await;
    // Call other functions for other destinations
    // ...
}

pub async fn print_help() {
    println!("Usage: cargo run -- <destination>");
    println!("Available Destinations:");
    println!("  all            Send logs to all locations");
    println!("  dynamodb       Create DynamoDB table");
    println!("  kdf            Send logs to Kinesis Firehose");
    println!("  s3             Send logs to S3 bucket");
    println!("  iam            Test iam features");
    println!("  elastic        Send logs to Elastic");
    process::exit(0);
}

pub async fn initialize_iam(config:Config){

    let iam_client = iam::start_iam().await;
    println!("{:?}",&config);

    match iam_client{
        Ok(client) => {
            if let Ok(users) = iam::list_users(&client,
                None,
                None,
                None).await {
                    print!("Printing users: ");
                    for user in users.users {
                        println!("{}", user.user_name);
                }
            
        }
    },
    Err(_) => todo!(),
}

}
pub async fn send_dynamodb(config: Config) {
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