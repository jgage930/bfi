#[derive(Debug, PartialEq, Eq)]
pub enum Commands {
    Right,
    Left,
    Inc,
    Dec,
    Output,
    Input,
    StartLoop,
    EndLoop,
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
            '[' => Self::StartLoop,
            ']' => Self::EndLoop,
            _ => Self::NA,
        }
    }
}
