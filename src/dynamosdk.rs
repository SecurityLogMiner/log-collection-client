/*
use tokio::time;
use async_trait::async_trait;
use std::thread;
use std::sync::mpsc::{Receiver};
*/


use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::error::SdkError;
use aws_sdk_dynamodb::primitives::Blob;
use aws_sdk_dynamodb::{config::Region, meta::PKG_VERSION, Client, Error};
use aws_sdk_dynamodb::operation::create_table::{CreateTableOutput,CreateTableError};
use aws_sdk_dynamodb::error::{BuildError};
use aws_sdk_dynamodb::types::{
    AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType,
};

// The trait and impl will need to move to the dynamosdk module. 
// Not yet ready to do so.
/*
#[async_trait]
pub trait TestDynamo {
    fn show(&self) -> String;
    async fn handle_log_data(&self,log_channel: Receiver<String>);
}
#[async_trait]
pub impl TestDynamo for DynamodbClient {
    fn show(&self) -> String {
        format!("{self:?}")
    }

    pub async fn handle_log_data(&self, log_channel: Receiver<String>) {
        for log_line in log_channel {
            println!("{log_line}");
        }
    }
}
*/

pub async fn
start_dynamo() -> Result<Client, Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_dynamodb::Client::new(&config);
    Ok(client)
}
pub fn show_dynamodb_tables() -> Result<(), Error> {
    todo!();
    Ok(())
}

pub async fn
start_dynamodb() -> Result<Client, Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_dynamodb::Client::new(&config);
    Ok(client)
}

pub async fn create_table(
    client: &Client,
    table: &str,
    key: &str,
) -> Result<CreateTableOutput, String> {
    let a_name: String = key.into();
    let table_name: String = table.into();

    let ad = AttributeDefinition::builder()
        .attribute_name(&a_name)
        .attribute_type(ScalarAttributeType::S)
        .build().unwrap();

    let ks = KeySchemaElement::builder()
        .attribute_name(&a_name)
        .key_type(KeyType::Hash)
        .build().unwrap();

    let pt = ProvisionedThroughput::builder()
        .read_capacity_units(10)
        .write_capacity_units(5)
        .build().unwrap();

    let create_table_response = client
        .create_table()
        .table_name(table_name)
        .key_schema(ks)
        .attribute_definitions(ad)
        .provisioned_throughput(pt)
        .send()
        .await;

    match create_table_response {
        Ok(out) => {
            println!("Added table {} with key {}", table, key);
            Ok(out)
        }
        Err(e) => {
            eprintln!("Got an error creating table:{e:?}");
            Err("error".to_string())
        }
    }
}

