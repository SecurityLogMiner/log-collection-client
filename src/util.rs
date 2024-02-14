use aws_sdk_dynamodb::client;

// util.rs serves as housing various utility functions that are used in main.rs
use crate::config::Config;
use crate::producer::start_log_stream;
use crate::dynamosdk; // Import other modules as needed
use std::{env, process};
use crate::iam;
use aws_sdk_iam::types::User;

pub async fn send_logs_to_all_destinations(config: Config) {
    // Call the functions to send logs to all destinations
    // let _ = start_log_stream(config).await;
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
    println!("  iam            Test iam features");
    println!("  elastic        Send logs to Elastic");
    process::exit(0);
}


/*
    What I'm thinking the purpose of this function could be is to create a new user
    Assign a specific pre-created role that specifies customers with limited permissions such Read only
        These would require all the policies needed for using S3, DynamoDB, Firehose, etc..
        This could be an option where the user requires send to "all"

    Another approach would be if we don't want a user to gain permissions to resources they are not using is to
    assign each policy to use those resources individually.

    Regardless, an option would be to:
    1. Create a user for a customer
    2. Assign roles to user based on needs (or all)
    3. User uses resources
    4. Delete the user?

    We could also follow the "Apply least-privilege permissions" scenario listed on AWS:
        "When you set permissions with IAM policies, grant only the permissions required to perform a task. 
        You do this by defining the actions that can be taken on specific resources under specific conditions, 
            also known as least-privilege permissions. 
        You might start with broad permissions while you explore the permissions that are required for your workload or use case. 
        As your use case matures, you can work to reduce the permissions that you grant to work toward least privilege. 
        For more information about using IAM to apply permissions"

    In this case, users will simply receive the policies based on the resouces they need.
*/
pub async fn initialize_iam(config:Config){

    let iam_client = iam::start_iam().await;
    println!("{:?}",&config);
    match iam_client {
        Ok(client) => {
            
            // Create a test user named "testuser"
            let testuser = "testuser";
            println!("\nStarted the IAM client\nCreating user: testuser");
            let created_user = iam::create_user(&client, testuser).await.unwrap();

            // List all the current users; must require IAM policy
            println!("Listing all current users along with added testuser");
            let users = iam::list_users(&client, None, None, None)
            .await
            .unwrap();
            for user in users.users {
                println!("{}", user.user_name);
            }

            // Delete the test user
            println!("\nDeleting user: testuser");
            match iam::delete_user(&client, &created_user).await {
                Ok(_) => {
                    println!("\nUser 'testuser' deleted successfully\n");
                }
                Err(err) => {
                    eprintln!("Error deleting user 'testuser': {:?}", err);
                    // Handle the error as needed
                }
            }
            println!("Current users after testuser deletion:");
            let users = iam::list_users(&client, None, None, None)
            .await
            .unwrap();
            for user in users.users {
                println!("{}", user.user_name);
            }
        }
               
            Err(err) => {
                println!("Error occurred starting IAM client: {}", err);
            }
    }
}



