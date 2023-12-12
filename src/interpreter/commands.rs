pub enum Commands {
    Right,
    Left,
    Inc,
    Dec,
    Output,
    Input,
    JumpForward,
    JumpBack,
    NA,
}

impl Commands {
    pub fn from_char(c: char) -> Self {
        match c {
            '>' => Self::Right,
            '<' => Self::Left,
            '+' => Self::Inc,
            '-' => Self::Dec,
            ',' => Self::Input,
            '.' => Self::Output,
            '[' => Self::JumpForward,
            ']' => Self::JumpBack,
            _ => Self::NA,
        }
    }
}
