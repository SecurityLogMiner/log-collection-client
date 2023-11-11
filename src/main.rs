mod producer;
use crate::producer::{run_tail_f};

mod interface;
use interface::{command_line};

fn main() {
    command_line();
    run_tail_f();
}
