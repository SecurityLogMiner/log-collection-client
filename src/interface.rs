#[allow(unused)]
use clap::Parser;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;


/* Creating a test struct CLI to test command line arguments */
#[derive(Parser,Debug)]
struct CLi{
    pattern: String,
    path: std::path::PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    server_address: String,
    server_port: u16,
    log_file_path: String,
}

/* Testing CLI arguments into a textfile */
pub fn command_line() ->Result<u8>{
    let args = CLi::parse();
    println!("{:?}",args);
    Ok(0)
}
