mod producer;
use crate::producer::{run_tail_f};

mod interface;
use interface::{command_line};

fn main() {
    let name = String::from("Security Log Collector");
    println!("Hello, {}",name);
    command_line();
    // see Comment A
    run_tail_f();
}
