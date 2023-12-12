use interpreter::interpreter::Interpreter;

use crate::interpreter::commands::Commands;
use std::fs::File;
use std::io::{BufReader, Read};

mod interpreter;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let chars = read_file("test.bf")?;
    let commands: Vec<Commands> = chars.iter().map(|c| Commands::from_char(*c)).collect();

    let mut interpreter = Interpreter::new();
    interpreter.interpret(commands);

    println!("{:?}", interpreter.memory());

    Ok(())
}

fn read_file(path: &str) -> Result<Vec<char>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let chars: Vec<char> = buffer.iter().map(|&byte| byte as char).collect();
    Ok(chars)
}
