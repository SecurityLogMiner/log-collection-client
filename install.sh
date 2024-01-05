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

