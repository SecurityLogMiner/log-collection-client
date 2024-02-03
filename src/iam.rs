use aws_sdk_iam::{Client as Client, ListAttachedUserPoliciesRequest, GetUserRequest};


pub async fn
start_iam() -> Result<Client, Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_firehose::Client::new(&config);
    Ok(client)
}

async fn get_user_policies(client: &Client, username: &str) {
    let request = ListAttachedUserPoliciesRequest {
        user_name: Some(username.to_string()),
        ..Default::default()
    };

    match client.list_attached_user_policies(request).await {
        Ok(response) => {
            // Process the list of attached policies
            for policy in response.attached_policies.unwrap_or_default() {
                println!("Attached Policy ARN: {}", policy.policy_arn.unwrap());
            }
        }
        Err(err) => eprintln!("Error listing attached user policies: {}", err),
    }
}


async fn get_user_name(client: &Client, access_key_id: &str) {
    let request = GetUserRequest {
        user_name: Some(access_key_id.to_string()),
        ..Default::default()
    };

    match client.get_user(request).await {
        Ok(response) => {
            if let Some(user) = response.user {
                if let Some(user_name) = user.user_name {
                    println!("IAM User Name: {}", user_name);
                }
            }
        }
        Err(err) => eprintln!("Error getting IAM user: {}", err),
    }
}
