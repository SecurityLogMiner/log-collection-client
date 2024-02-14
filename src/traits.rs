use async_trait::async_trait;
use std::sync::mpsc::{Receiver};
use aws_sdk_dynamodb::Client as DynamodbClient;
use aws_sdk_firehose::Client as OpenSearchClient;

#[async_trait]
pub trait DataHandler {
    fn show(&self) -> String;
    fn clone_self(&self) -> Self;
    async fn handle_log_data(&self,log_channel: Receiver<String>);
}
