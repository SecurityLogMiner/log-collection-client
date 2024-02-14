use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::error::SdkError;
use aws_sdk_dynamodb::primitives::Blob;
use aws_sdk_dynamodb::{config::Region, meta::PKG_VERSION, Error};
use aws_sdk_dynamodb::operation::create_table::{CreateTableOutput,CreateTableError};
use aws_sdk_dynamodb::operation::put_item::{PutItemOutput, PutItemError};
use aws_sdk_dynamodb::error::{BuildError};

use aws_sdk_dynamodb::types::{
    AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType,
};
use async_trait::async_trait;
use std::sync::mpsc::{Receiver};
use crate::dynamosdk;
use crate::traits::DataHandler;
use crate::config::Config;
use crate::producer::start_log_stream;
use aws_sdk_dynamodb::Client as DynamodbClient;
use aws_sdk_dynamodb::types::AttributeValue;

#[async_trait]
impl DataHandler for DynamodbClient {
    fn show(&self) -> String {
        format!("DynamodbClient: {:?}", &self)
    }
    fn clone_self(&self) -> Self {
        self.clone()
    }
    async fn handle_log_data(&self, log_channel: Receiver<String>) {
        if let Ok(table) = self.describe_table().table_name("eptesttable").send().await {
            for log_line in log_channel {
                println!("{log_line}");
                let res = self.put_item()
                    .table_name("eptesttable")
                    .item("epkeyitem",AttributeValue::S(log_line))
                    .send().await;
                // dumb error checking for now. eventually, this will need to be 
                // sent to the status api for the user.
                //println!("{res:?}");
            }
        }
    }
}

pub fn show_dynamodb_tables() -> Result<(), Error> {
    todo!();
    Ok(())
}

pub async fn
create_client() -> Result<DynamodbClient, Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_dynamodb::Client::new(&config);
    Ok(client)
}

pub async fn create_table(
    client: &DynamodbClient,
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

pub async fn send_dynamodb(config: Config) {
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
                if tbl == config.dynamodb.table {
                    println!("found {tbl:?}");
                    // use the table
                    let _ = start_log_stream(config.sources.logs.clone(),
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
