use std::io::Read;

use super::instruction::{self, Instruction};

pub struct Interpreter {
    instructions: Vec<Instruction>,
    pointer: usize,
    memory: [u8; 3000],
}

impl Interpreter {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            pointer: 0,
            memory: [0u8; 3000],
        }
    }

    pub fn run(&self) {
        for instruction in self.instructions.iter() {
            Self::interpret(&instruction);
        }
    }

    fn interpret(instruction: &Instruction) {}

    fn pointer_right(&mut self) {
        self.pointer += 1;
    }

    fn pointer_left(&mut self) {
        self.pointer -= 1;
    }

    fn increment(&mut self) {
        self.memory[self.pointer] += 1;
    }

    fn decrement(&mut self) {
        self.memory[self.pointer] -= 1;
    }

    fn output(&self) {
        print!("{}", self.memory[self.pointer]);
    }

    fn input(&mut self) {
        let mut input: [u8; 1] = [0; 1];
        std::io::stdin()
            .read_exact(&mut input)
            .expect("Failed to input");

        self.memory[self.pointer] = input[0];
    }

    fn loop_(&mut self) {
        while self.memory[self.pointer] != 0 {
            todo!();
        }
    }
}
