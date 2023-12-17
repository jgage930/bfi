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

    pub fn run(&self) {
        todo!();
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

    fn output(&self) {
        print!("{}", self.memory[self.pointer]);
    }

    fn input(&mut self) {
        todo!()
    }

    fn loop_(&mut self) {
        todo!()
    }
}
