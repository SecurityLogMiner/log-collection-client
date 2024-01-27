use aws_config::meta::region::RegionProviderChain;
use aws_sdk_firehose::error::SdkError;
use aws_sdk_firehose::operation::put_record_batch::{PutRecordBatchError, PutRecordBatchOutput};
use aws_sdk_firehose::primitives::Blob;
use aws_sdk_firehose::types::Record;
use aws_sdk_firehose::{config::Region, meta::PKG_VERSION, Client, Error};

use aws_sdk_dynamodb as dynamodb;

/*
 * A buffer file will be used to accumulate the source data and, upon threshhold 
 * or time limit, will send the processed data (the file) as a batch using 
 * aws firehose. The test against this that I can think of is having an attacker
 * try to read and/or alter either the data source or the processed data. 
 */
pub fn send_batch() {
    println!("send the processed data using firehose.");
}
pub async fn create_firehose() {
    println!("create firehose client ");
}

pub async fn
start_elastic() -> Result<(), Error> {
    Ok(())
}

pub async fn
start_firehose() -> Result<Client, Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_firehose::Client::new(&config);
    Ok(client)
}

pub async fn
start_dynamo() -> Result<Client, Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_dynamodb::Client::new(&config);
    Ok(client)
}

pub async fn 
show_streams(client: &Client) -> Result<(), Error> {
    todo!();
    Ok(())
}

pub async fn 
add_record(client: &Client, stream: &str, key: &str, data: &str
           ) -> Result<(), Error> {
    println!("Put data into stream.");

    Ok(())
}
pub async fn put_record_batch(
    client: &Client,
    stream: &str,
    data: Vec<Record>,
) -> Result<PutRecordBatchOutput, SdkError<PutRecordBatchError>> {
    client
        .put_record_batch()
        .delivery_stream_name(stream)
        .set_records(Some(data))
        .send()
        .await
}
