mod config;
mod producer;
mod awssdk;

use producer::{start_log_stream};
use config::{read_config};
use aws_sdk_kinesis::{Error};

#[tokio::main]
async fn 
main() -> Result<(), std::io::Error> {
    let config_data = read_config();
    match config_data {
        Some(config) => {
            let _ = start_log_stream(config).await;
        }
        None => panic!("error reading configuration. fix it.")
    }
    Ok(())
}
