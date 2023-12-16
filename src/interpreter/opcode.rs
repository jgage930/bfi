#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    PointerRight,
    PointerLeft,
    Increment,
    Decrement,
    Input,
    Output,
    StartLoop,
    EndLoop,
    NA,
}

impl OpCode {
    pub fn from_char(c: char) -> Self {
        match c {
            '>' => Self::PointerRight,
            '<' => Self::PointerLeft,
            '+' => Self::Increment,
            '-' => Self::Decrement,
            ',' => Self::Input,
            '.' => Self::Output,
            '[' => Self::StartLoop,
            ']' => Self::EndLoop,
            _ => Self::NA,
        }
    }
}
