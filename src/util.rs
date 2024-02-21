use aws_sdk_dynamodb::client;
use aws_sdk_dynamodb::operation::query;

// util.rs serves as housing various utility functions that are used in main.rs
use crate::config::Config;
use crate::producer::start_log_stream;
use crate::dynamosdk; // Import other modules as needed
use std::process;
use std::fmt;
use crate::iam;
use aws_sdk_iam::types::User;
use std::process::Command;
use std::io::{self, Write};
pub struct UserDisplay<'a>(pub &'a aws_sdk_iam::types::User);


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

pub async fn initialize_iam(config: Config) {
    // Only make this accessible to users with admin privileges!!

    let iam_client = iam::start_iam().await;
    println!("{:?}", &config);
    match iam_client {
        Ok(client) => {
            let user = iam::get_user(&client).await.unwrap();
            if iam::is_admin_user(&user.user.as_ref().unwrap(), &client).await {

                println!("1. List all current users");
                println!("2. Get current user");
                println!("3. Exit");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let choice: u32 = input.trim().parse().expect("Please enter a number");

                match choice {
                    1 => {
                        println!("\nListing all current users");
                        if let Ok(users) = iam::list_users(&client, None, None, None).await {
                            for user in users.users {
                                println!("{}", user.user_name);
                            }
                        } else {
                            eprintln!("Failed to list users. Please check your network connection and IAM permissions, and try again.");
                        }
                    }
                    2 => {
                        if let Ok(user) = iam::get_user(&client).await {
                            println!("\nCurrent User:");
                            println!("{}", UserDisplay(user.user.as_ref().unwrap()));
                        }
                        else {
                            eprintln!("Failed to get the user. Please check your network connection and IAM permissions, and try again.");
                        }
                    }
                    3 => {
                        process::exit(0);
                    }
                    _ => {
                        println!("Invalid choice");
                    }
                }
            } else {
                println!("\n{} is not an administrator!", user.user.as_ref().unwrap().user_name);
                println!("Please contact the administrator for more information.");
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

