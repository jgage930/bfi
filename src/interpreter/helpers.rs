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
    let mut instructions: Vec<Instruction> = Vec::new();

    let mut loop_stack = 0;
    let mut loop_start = 0;

    for (i, code) in codes.iter().enumerate() {
        if loop_stack == 0 {
            let instruction = match code {
                OpCode::PointerRight => Some(Instruction::PointerRight),
                OpCode::PointerLeft => Some(Instruction::PointerLeft),
                OpCode::Increment => Some(Instruction::Increment),
                OpCode::Decrement => Some(Instruction::Decrement),
                OpCode::Output => Some(Instruction::Output),
                OpCode::Input => Some(Instruction::Input),
                OpCode::StartLoop => {
                    loop_start = i;
                    loop_stack += 1;
                    None
                }
                OpCode::EndLoop => {
                    panic!("Closing loop has no start!!");
                }
                OpCode::NA => Some(Instruction::End),
            };

            if let Some(ins) = instruction {
                instructions.push(ins);
            }
        } else {
            match code {
                OpCode::StartLoop => {
                    loop_stack += 1;
                }
                OpCode::EndLoop => {
                    loop_stack -= 1;

                    if loop_stack == 0 {
                        instructions
                            .push(Instruction::Loop(parse(codes[loop_start + 1..i].to_vec())));
                    }
                }
                _ => {}
            }
        }
    }

    if loop_stack != 0 {
        panic!("Unclosed loop!!")
    }

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
