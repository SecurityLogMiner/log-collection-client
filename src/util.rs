use aws_sdk_dynamodb::client;

// util.rs serves as housing various utility functions that are used in main.rs
use crate::config::Config;
use crate::producer::start_log_stream;
use crate::dynamosdk; // Import other modules as needed
use std::process;
use std::fmt;
use crate::iam;
use aws_sdk_iam::types::User;
use std::{env, process::Command};
use std::io::{self, Write};
pub struct UserDisplay<'a>(pub &'a aws_sdk_iam::types::User);


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
    println!("  run-admin      Connecto to the Administrator AWS CLI");
    println!("  elastic        Send logs to Elastic");
    process::exit(0);
}


impl<'a> fmt::Display for UserDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let user: &User = self.0;
        write!(f,
            "Username: {}\nUser ID: {}\nARN: {}\nPermission Boundaries: {:?}\n",
            user.user_name, user.user_id, user.arn, user.permissions_boundary)
        
    }    
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


/*
    Grant the user groups.
        Group policy or roles
    List the groups and assign least-privilege permissions to the user.

    If the user wants to send to all; check if they have the necessary policies and then throw an error
    They need to contact the admin to add that policy

        User supplies a list of source logs
            Create a table for each source logs
        Open search should be able to find this.
            opensearch and dynmo
    
    Dynamo priveleges:
        Dynamo needs to create a table;
        Write to a table
    
    Dashboard would be using admin privielges to read and display content.

    Check the credentials file and read IAM string and check if that user exists.
    Start the dynamoDB client;
        Check the privileges

*/
pub async fn initialize_iam(config:Config){

    let iam_client = iam::start_iam().await;
    println!("{:?}",&config);
    match iam_client {
        Ok(client) => {
            // Check if the user exists
            // List all the current users; must require IAM policy
            // Currently endpoint users are able to list this out along with admins but this is not advisable. 
            // I'm sure there is a policy on iam to have them list only thier own credentials and users
            println!("\nListing all current users");
            let users = iam::list_users(&client, None, None, None)
            .await
            .unwrap();
            for user in users.users {
                println!("{}", user.user_name);
            }
            
            if let Ok(user) = iam::get_user(&client).await {
                println!("\nCurrent User:");
                println!("{}", UserDisplay(user.user.as_ref().unwrap()));            
            } 
            else {
                eprintln!("Failed to get the user. Please check your network connection and IAM permissions, and try again.");
            }
        }
               
            Err(err) => {
                println!("Error occurred starting IAM client: {}", err);
            }
    }
    
}


/// Asynchronously runs the Administrator AWS CLI if the user is an admin
pub async fn run_admin_cli(){
    // Check if the user is an admin

    // Get the current user
    let iam_client = iam::start_iam().await;
    match iam_client {
        Ok(client) => {
            let user = iam::get_user(&client).await.unwrap();
            if iam::is_admin_user(&user.user.as_ref().unwrap(), &client).await {
                // Set up standard input and output
                let stdin = io::stdin();
                let mut stdout = io::stdout();
        
                // Print a message indicating the start of the Administrator AWS CLI
                println!("\nRunning Administrator AWS CLI:");
                // Start a loop to continuously receive and process user input
                loop {
                    // Prompt the user for input
                    print!("aws> ");
                    // Ensure the prompt is displayed immediately by flushing the output
                    stdout.flush().unwrap();
        
                    // Read user input from the standard input
                    let mut input = String::new();
                    stdin.read_line(&mut input).unwrap();
        
                    // Check if the user wants to exit the CLI
                    if input.trim().eq_ignore_ascii_case("exit") {
                        break;
                    }
        
                    // Split the input into individual arguments
                    let args: Vec<&str> = input.trim().split_whitespace().collect();
                    // If there are no arguments, continue to the next iteration of the loop
                    if args.is_empty() {
                        continue;
                    }
        
                    // Execute the AWS CLI command with the provided arguments
                    let output = Command::new("aws")
                        .args(&args)
                        .output()
                        .expect("Failed to execute command");
        
                    // Write the command output to the standard output and standard error
                    io::stdout().write_all(&output.stdout).unwrap();
                    io::stderr().write_all(&output.stderr).unwrap();
                } 
            }
            else{
                println!("\n{} is not an admininstrator!", user.user.as_ref().unwrap().user_name);
                println!("Please contact the admininistrator for more information.");
            }
        }
        Err(err) => {
            println!("Error occurred starting IAM client: {}", err);
        }
    }
}


pub async fn send_dynamodb(config: Config) {
// Call the function to create DynamoDB table
let dynamoclient = dynamosdk::start_dynamodb().await;
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
            if tbl == config.dynamo_table_name {
                println!("found {tbl:?}");
                // use the table
                let _ = start_log_stream(config.log_paths.clone()).await;
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