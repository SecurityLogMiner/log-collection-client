use aws_sdk_iam::{Client as Client, Error};
use aws_sdk_iam::error::SdkError;


// #[::tokio::main]
pub async fn
start_iam() -> Result<Client, Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_iam::Client::new(&config);
    Ok(client)
}

async fn get_user_policies(client: &Client, username: &str) {


    // Todo
}

