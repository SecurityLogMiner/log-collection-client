use std::env;
mod producer;
//use crate::producer::{run_tail_f};

mod config;
use config::{read_config};

fn main() {
    match env::args().len() {
        1 => {let _ = read_config();}
        _ => {println!("handle command line usage");}
    }
}
