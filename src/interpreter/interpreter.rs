use crate::Result;
use crate::{interpreter::commands::Commands, stack::Stack};
use std::fs::File;
use std::io::{self, BufReader, Read};

pub struct Interpreter {
    memory: [u8; 3000],
    // Points to a position in memory
    mem_pointer: usize,

    commands: Vec<Commands>,
    // Points to a position in the command vec
    cmd_pointer: usize,
    stack: Stack<usize>,
}

impl Interpreter {
    pub fn from_file(path: &str) -> Result<Self> {
        let chars = Self::read_bf(path)?;
        let commands: Vec<Commands> = chars.iter().map(|c| Commands::from_char(*c)).collect();

        Ok(Self {
            memory: [0u8; 3000],
            mem_pointer: 0,
            commands,
            cmd_pointer: 0,
            stack: Stack::new(),
        })
    }

    fn read_bf(path: &str) -> Result<Vec<char>> {
        // Read a .bf file into a vec of chars
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;

        let chars: Vec<char> = buffer.iter().map(|&byte| byte as char).collect();
        Ok(chars)
    }

    pub fn interpret(&mut self) {
        // Execute the command at command pointer.
        match self.commands[self.cmd_pointer] {
            Commands::Right => self.move_pointer_right(),
            Commands::Left => self.move_pointer_left(),
            Commands::Inc => self.increment(),
            Commands::Dec => self.decrement(),
            Commands::Output => self.output(),
            Commands::Input => self.input().expect("Failed to read char."),
            Commands::StartLoop => self.start_loop(),
            _ => {}
        }
    }

    fn next_command(&mut self) {
        self.cmd_pointer += 1;
    }

    fn move_pointer_right(&mut self) {
        self.mem_pointer += 1;
    }

    fn move_pointer_left(&mut self) {
        self.mem_pointer -= 1;
    }

    fn increment(&mut self) {
        self.memory[self.mem_pointer] += 1;
    }

    fn decrement(&mut self) {
        self.memory[self.mem_pointer] -= 1;
    }

    fn output(&self) {
        print!("{}", self.memory[self.mem_pointer] as char);
    }

    fn input(&mut self) -> Result<()> {
        // prompt the user to input a single byte
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        // ! This only reads the first char from input.
        if let Some(c) = input.chars().next() {
            self.memory[self.mem_pointer] = c as u8;
        };

        Ok(())
    }

    fn start_loop(&mut self) {
        self.stack.push(self.mem_pointer);

        if self.memory[self.mem_pointer] != 0 {
            self.next_command();
        } else {
            let end_index = self.stack.pop();

            if let Some(i) = end_index {
                self.cmd_pointer = i;
            }
        }
    }

    fn end_loop(&mut self) {
        if let Some(i) = self.stack.pop() {
            self.cmd_pointer = i;
        }

        self.stack.push(self.cmd_pointer);
    }
}
