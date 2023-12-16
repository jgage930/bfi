use super::{instruction::Instruction, opcode::OpCode};

fn lex(input: String) -> Vec<OpCode> {
    // Turn the code from the file into a list of codes.
    input
        .chars()
        .into_iter()
        .map(|c| OpCode::from_char(c))
        .collect::<Vec<OpCode>>()
}

fn parse_loop_indicies(codes: Vec<OpCode>) -> Vec<(usize, usize)> {
    let mut indicies = Vec::new();

    let mut start_loop = None;
    for (i, code) in codes.iter().enumerate() {
        match code {
            OpCode::StartLoop => {
                start_loop = Some(i);
            }
            OpCode::EndLoop => {
                if let Some(start_index) = start_loop {
                    indicies.push((start_index, i));
                }
            }
            _ => {}
        }
    }

    indicies
}

fn parse(codes: Vec<OpCode>) -> Vec<Instruction> {
    // Turn a vec of op codes into a list of instructions.
    let in_loop = false;
    let instructions = Vec::new();

    instructions
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing_input() -> String {
        ",+[-.,+]".to_string()
    }

    fn testing_codes() -> Vec<OpCode> {
        vec![
            OpCode::Input,
            OpCode::Increment,
            OpCode::StartLoop,
            OpCode::Decrement,
            OpCode::Output,
            OpCode::Input,
            OpCode::Increment,
            OpCode::EndLoop,
        ]
    }

    #[test]
    fn test_lex() {
        assert_eq!(lex(testing_input()), testing_codes(),)
    }

    #[test]
    fn test_loop_indicies() {
        assert_eq!(parse_loop_indicies(testing_codes()), vec![(2, 7)])
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(testing_codes()),
            vec![
                Instruction::Input,
                Instruction::Increment,
                Instruction::Loop(vec![
                    Instruction::Decrement,
                    Instruction::Output,
                    Instruction::Input,
                    Instruction::Increment,
                ])
            ]
        );
    }
}
