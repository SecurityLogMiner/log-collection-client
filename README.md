<a name="readme-top"></a>

# Log Collection Client 

A service that collects and sends system event data to a server.

  <p align="left">
    <br />
    <a href="https://securitylogminer-doc-repo.readthedocs.io/"><strong>Documentation »</strong></a>
    <br />
  
## Table of Contents

- [Getting Started](#getting-started)
    - [Users](##users)
    - [Administrators](##admins)
    - [Client Instructions](#client-instructions)
- [Resources](#resources)
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

## Users
Log Collection Client users are granted limited access to resources on AWS, adhering to the principle of [least-privilege permissions](https://docs.aws.amazon.com/IAM/latest/UserGuide/best-practices.html#grant-least-privilege). This approach ensures that users only receive the permissions necessary to fulfill their specific tasks. Least privilege is a core principle among Amazon Web Services (AWS) Well-Architected best practices, contributing to secure cloud architecture.

By default, users are configured with read-only access and specific write permissions essential for the Log Collection Client's interactions with AWS. This setup aims to strike a balance between functionality and security.
## Administrators
Adminstrators are responsible for managing user account and permissions accordingly.
They have default full access to various AWS resources critical for the client's operation, including:
- DynamoDB
- Kinesis Firehose
- S3 bucket management
- IAM Permissions
Administrators are tasked with overseeing the overall system configuration, ensuring that users have the appropriate levels of access to fulfill their responsibilities while upholding security best practices.

Additionally, administrators will have access to the AWS-CLI through the client. Running the command **cargo run -- run-admin** will allow administrators to input commands to the AWS through the client command line. However, it is advised to access and modify permissions on the actual [AWS website.](https://portal.aws.amazon.com).

## Client Instructions
Clone the client repositories to start.
- [Client](https://github.com/SecurityLogMiner/log-collection-client)

The client will read the configuration file and begin processing and sending 
log data from the given PATH to the server.

When running the client for the first time on a linux system, a directory will 
be created at:
- /var/log/logminer/logs/

If you do not have a system service that you are able to read log data from, you
can create one with a combination of a shell script and cronjob:

script.sh:
```
#!/bin/bash
for ((i = 1; i <= 60; i++)); do
    echo "test $(date)" >> /var/log/logminer/logs/test.log
    sleep 1
done
echo "" > /var/log/logminer/logs/test.log
```

cronjob:
```
* * * * * <path_to_your_script>
0 * * * * echo "" > /var/log/logminer/logs/test.log
```

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
cargo run -- -help
```
Running the client through the command line will require the user or administrator to specify a specific destination.
The available destinations include:
- DynamoDB
- Kinesis Firehose
- S3
- Identity and Access Management (IAM)
- Elastic

Run ```cargo run -- --help ``` to output all available destinations.

## Resources
- [SDK for Rust](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/using.html)
- [Amazon Simple Storage Service](https://docs.aws.amazon.com/AmazonS3/latest/userguide/Welcome.html)
- [AWS SDK and S3 Rust Documentation](https://docs.rs/aws-sdk-s3/latest/aws_sdk_s3/index.html)
- [Rust AWS SDK Examples](https://github.com/awslabs/aws-sdk-rust/tree/main/examples/examples/s3)
- [Amazon Dynamodb Docs](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/GettingStarted.html)
- [Amazon Dynamodb Examples](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/rustv1/examples/dynamodb#code-examples)

### IAM Resources
- [IAM Crate](https://docs.rs/aws-iam/latest/aws_iam/)
- [IAM SDK Code Examples](https://docs.aws.amazon.com/IAM/latest/UserGuide/service_code_examples_iam.html)
- [AWS-SDK-IAM Client docs](https://docs.rs/aws-sdk-iam/latest/aws_sdk_iam/client/struct.Client.html)
- [Creating Read only and read-write users IAM AWS SDK](https://docs.aws.amazon.com/IAM/latest/UserGuide/iam_example_iam_Scenario_UserPolicies_section.html)
<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Ratatui Resources
 - [Installation](https://ratatui.rs/installation/)
## License
Apache 2.0

## Acknowledgments

## Contact
