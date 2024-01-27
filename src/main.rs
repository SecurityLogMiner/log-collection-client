mod config;
mod producer;
mod firehosesdk;
mod dynamosdk;

use producer::{start_log_stream};
use config::{read_config};
use std::env;

/*transfer these into dynamo mod */
use aws_sdk_dynamodb::operation::list_tables::{ListTablesError};

#[tokio::main]
async fn 
main() -> Result<(), std::io::Error> {
    let config_data = read_config();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        match config_data {
            Some(config) => {
                let _ = start_log_stream(config).await;
            }
            None => panic!("error reading configuration. fix it.")
        }
    } else {
        println!("make dynamo");
        if let Ok(client) = dynamosdk::start_dynamodb().await {
            println!("client exists");
            if let Ok(res) = dynamosdk::create_table(&client,
                                                       "eptesttable",
                                                       "epkeyitem").await {
                println!("{res:?}");
            }

        }
    }

    Ok(())
}
