#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    PointerRight,
    PointerLeft,
    Increment,
    Decrement,
    Input,
    Output,
    Loop(Vec<Instruction>),
}
