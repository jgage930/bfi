use super::opcode::OpCode;

#[derive(Debug, Clone)]
pub enum Instruction {
    PointerRight,
    PointerLeft,
    Increment,
    Decrement,
    Input,
    Output,
    Loop(Vec<Instruction>),
    End,
}
