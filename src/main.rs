mod config;
mod producer;
mod awssdk;
//mod dynamo;

use producer::{start_log_stream};
use config::{read_config};
use aws_sdk_firehose::{Error};
use std::env;

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
    }

    Ok(())
}
