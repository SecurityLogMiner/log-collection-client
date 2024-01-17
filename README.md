# Log Collection Client 

A service that collects and sends system event data to a server.

## Table of Contents

- [Getting Started](#getting-started)
- [License](#license)
- [Acknowledgments](#acknowledgments)
- [Contact](#contact)

### Creating Issues
TODO

## Getting Started
Install Rust on your local machine. Use the following link to get setup quickly:
[rust setup](https://www.rust-lang.org/tools/install)

Create an AWS account, setup IAM and bucket policies.
1. [Create an AWS Account](https://portal.aws.amazon.com/billing/signup#/start/email)

2. Set up Identity and Access Management account (IAM).
    - Note: Be sure to copy down your access and secret access key and save them locally.

3. Set up bucket policies to receive data using Amazon S3.
    - AmazonKinesisFirehoseFullAccess
    - AmazonKinesisFullAccess
    - AmazonS3FullAccess

Clone the client repositories to start.
- [Client](https://github.com/SecurityLogMiner/log-collection-client)

The client will read the configuration file and begin processing and sending 
log data from the given PATH to the server.

When running the client for the first time on a linux system, a directory will 
be created at:
- /var/log/logminer/logs/

If you do not have a system service that you are able to read log data from, you
can create one with the install script.

install.sh:
```
#!/bin/bash

# Define the log directory path
LOG_DIR="/var/log/logminer/logs"
# TODO - add the setup to add user permissions to interact with the log directory
# setfacl -m u:username:perms /path/to/log/file

# Create the directory if it doesn't exist
if [ ! -d "$LOG_DIR" ]; then
    mkdir -p "$LOG_DIR"
    chmod 700 "$LOG_DIR"  # Adjust permissions as needed
    echo "Log directory created at: $LOG_DIR"
else
    echo "Log directory already exists at: $LOG_DIR"
fi
```

Ensure that your IAM credentials are provided in ~/.aws/credentials:
```
[default] 
  aws_access_key_id=YOUR-ACCESS-KEY
  aws_secret_access_key=YOUR-SECRET-KEY
```
Enter the command "aws config" to configure these credentials as well as the output type(text) and region "us-west-2".

The client will look for these credentials when executed.

Running the Client:
```
cd <client_repo_dir>
cargo install
cargo run
```

## License
Apache 2.0

## Acknowledgments

## Contact

[Back to top](#table-of-contents)

https://docs.aws.amazon.com/sdk-for-rust/latest/dg/using.html
https://docs.aws.amazon.com/AmazonS3/latest/userguide/Welcome.html
https://github.com/awslabs/aws-sdk-rust/tree/main/examples/examples/s3
https://docs.rs/aws-sdk-s3/latest/aws_sdk_s3/index.html


https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Introduction.html

https://aws.amazon.com/kinesis/data-streams/pricing/
