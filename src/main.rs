mod awss3;
mod config;
mod producer;

use clap::Parser;
use producer::{start_log_stream};
use config::{read_config};
use awss3::{start_s3};
use aws_sdk_s3::{meta::PKG_VERSION};

#[derive(Debug, Parser)]
struct Opt {
    /// The name of the bucket.
    #[structopt(short, long)]
    bucket: String,

    /// The name of the log data object in the bucket.
    #[structopt(short, long)]
    key: String,

    /// Whether to display additional information.
    #[structopt(short, long)]
    verbose: bool,
}

// still need this to create the event and send to sink. The sink is the s3 bucket
// see producer:handle_log_data
#[tokio::main]
async fn 
main() {

    // these should be optional. for now, hard code them and test
    /*
    let Opt {
        bucket,
        key,
        verbose,
    } = Opt::parse();

    println!();

    if verbose {
        println!("S3 client version: {}", PKG_VERSION);
        println!("Region: us-west-2");// set in ~/.aws/credentials
        println!("Bucket:            {}", &bucket);
        println!("Log Data Key:               {}", &key);
        println!();
    }
    */
    //let result = start_s3();

    if let Ok(client) = start_s3().await {
        // pass client into configuration 
        let config_data = read_config(&client);
        match config_data {
            Some(config) => {
                let _ = start_log_stream(config).await;
            }
            None => panic!("error reading configuration. fix it.")
        }
    }
}
