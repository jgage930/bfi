use crate::interpreter::commands::Commands;

pub struct Interpreter {
    pointer: usize,
    memory: [u8; 3000],
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            pointer: 0,
            memory: [0u8; 3000],
        }
    }

    pub fn interpret(&mut self, commands: Vec<Commands>) {
        todo!();
    }
}
