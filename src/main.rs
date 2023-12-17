use std::{fs::File, io::Read};

use interpreter::{
    helpers::{lex, parse},
    interpreter::Interpreter,
};

mod interpreter;
mod stack;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    // read file into string
    let mut code = String::new();
    let mut file = File::open("examples/hello_world.bf")?;
    file.read_to_string(&mut code)?;

    let opcodes = lex(code);
    let instructions = parse(opcodes);

    let mut interpreter = Interpreter::new(instructions);
    interpreter.run();

    Ok(())
}
