use std::{fs::File, io::Read};

use clap::Parser;
use cli::Cli;
use interpreter::{
    helpers::{lex, parse},
    interpreter::Interpreter,
};

mod cli;
mod interpreter;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    // read file into string
    let args = Cli::parse();

    let mut code = String::new();
    let mut file = File::open(args.path.as_str())?;
    file.read_to_string(&mut code)?;

    let opcodes = lex(code);
    let instructions = parse(opcodes);

    let mut interpreter = Interpreter::new(instructions);
    interpreter.run();

    Ok(())
}
