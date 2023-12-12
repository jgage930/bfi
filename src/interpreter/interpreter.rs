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

    pub fn pointer(&self) -> usize {
        self.pointer
    }

    pub fn memory(&self) -> [u8; 3000] {
        self.memory
    }

    pub fn interpret(&mut self, commands: Vec<Commands>) {
        for command in commands.iter() {
            match command {
                Commands::Right => self.move_pointer_right(),
                Commands::Left => self.move_pointer_left(),
                Commands::Inc => self.increment(),
                Commands::Dec => self.decrement(),
                _ => {}
            }
        }
    }

    fn move_pointer_right(&mut self) {
        self.pointer += 1;
    }

    fn move_pointer_left(&mut self) {
        self.pointer -= 1;
    }

    fn increment(&mut self) {
        self.memory[self.pointer] += 1;
    }

    fn decrement(&mut self) {
        self.memory[self.pointer] -= 1;
    }
}
