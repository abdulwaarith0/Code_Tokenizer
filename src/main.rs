use std::io::{stdin, stdout};
use crate::repl::start;

pub mod token;
pub mod lexer;
pub mod repl;
pub mod ast;

fn main() {
    println!("Hello, world!");
    println!("Type in commands");
    start(stdin(), stdout());

}
