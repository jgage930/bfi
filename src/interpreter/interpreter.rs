use std::io;

use super::instruction::Instruction;

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

    pub fn run(&mut self) {
        for instruction in self.instructions.clone().iter() {
            self.interpret(instruction);
        }
    }

    fn interpret(&mut self, instruction: &Instruction) {
        match instruction {
            &Instruction::PointerRight => {
                self.pointer_right();
            }
            &Instruction::PointerLeft => {
                self.pointer_left();
            }
            &Instruction::Increment => {
                self.increment();
            }
            &Instruction::Decrement => {
                self.decrement();
            }
            &Instruction::Output => {
                self.output();
            }
            &Instruction::Input => {
                self.input();
            }
            Instruction::Loop(to_loop) => {
                self.loop_(&to_loop);
            }
            &Instruction::End => {}
        }
    }

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

    fn output(&mut self) {
        print!("{}", self.memory[self.pointer] as char);
    }

    fn input(&mut self) {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");

        if let Some(c) = input.chars().next() {
            self.memory[self.pointer] = c as u8;
        }
    }

    fn loop_(&mut self, to_loop: &Vec<Instruction>) {
        while self.memory[self.pointer] != 0 {
            for instruction in to_loop.iter() {
                self.interpret(instruction);
            }
        }
    }
}
