use std::env;
mod producer;
use crate::producer::{run_tail_f};

mod interface;
use interface::{command_line,read_config};

fn main() {
    match env::args().len() {
        1 => {read_config();}
        _ => {println!("handle command line usage");}
    }
}
