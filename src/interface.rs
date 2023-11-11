
#[allow(unused)]
use clap::Parser;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;


/* Creating a test struct CLI to test command line arguments */
#[derive(Parser)]
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
pub fn command_line() ->Result<()>{
    let args = CLi::parse();
    let path_name = &args.path;
    let content = match std::fs::read_to_string(path_name){
        Ok(content) => content,
        Err(error) => {
            println!("{}: {}", path_name.to_string_lossy(), error);
            return Err(error)
                .with_context(|| format!("could not read file `{}`", path_name.to_string_lossy()));
        }
    };
    
    println!("File Content: {}", content);
    let log_entries: Vec<Config> = serde_json::from_str(&content)
        .with_context(|| "failed to parse JSON data")?;

    println!("Log Entries:");
    for entry in &log_entries {
        println!("{:?}", entry);
    }
    Ok(())
}
