use interpreter::interpreter::Interpreter;

use std::fs::File;
use std::io::{BufReader, Read};

mod interpreter;
mod stack;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let mut interpreter = Interpreter::from_file("test.bf")?;
    interpreter.run();

    Ok(())
}
