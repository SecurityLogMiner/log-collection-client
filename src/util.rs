// util.rs serves as housing various utility functions that are used in main.rs
use crate::config::Config;
use crate::producer::start_log_stream;
use crate::dynamosdk; // Import other modules as needed

pub async fn send_logs_to_all_destinations(config: Config) {
    // Call the functions to send logs to all destinations
    let _ = start_log_stream(config).await;
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
    println!("  elastic        Send logs to Elastic");
}