#[derive(Debug)]
enum ParserState {
    Invalid,
    MulIns(char),
    DoIns(char),
    DontIns(char),
    ParenOpen,
    Num1Digit(u8),
    Comma,
    Num2Digit(u8),
    CondParenOpen,
    EndMulInstruction,
    EndCondInsInstruction,
}

fn next_state(last_state: ParserState, input: char) -> ParserState {
    match (last_state, input) {
        (_, 'm') => ParserState::MulIns('m'),
        (ParserState::MulIns('m'), 'u') => ParserState::MulIns('u'),
        (ParserState::MulIns('u'), 'l') => ParserState::MulIns('l'),
        (ParserState::MulIns('l'), '(') => ParserState::ParenOpen,
        (ParserState::ParenOpen, '0'..='9') => ParserState::Num1Digit(input as u8 - '0' as u8),
        (ParserState::Num1Digit(_), '0'..='9') => ParserState::Num1Digit(input as u8 - '0' as u8),
        (ParserState::Num1Digit(_), ',') => ParserState::Comma,
        (ParserState::Comma, '0'..='9') => ParserState::Num2Digit(input as u8 - '0' as u8),
        (ParserState::Num2Digit(_), '0'..='9') => ParserState::Num2Digit(input as u8 - '0' as u8),
        (ParserState::Num2Digit(_), ')') => ParserState::EndMulInstruction,

        (_, 'd') => ParserState::DoIns('d'),
        (ParserState::DoIns('d'), 'o') => ParserState::DoIns('o'),
        (ParserState::DoIns('o'), '(') => ParserState::CondParenOpen,
        (ParserState::DoIns('o'), 'n') => ParserState::DontIns('n'),
        (ParserState::DontIns('n'), '\'') => ParserState::DontIns('\''),
        (ParserState::DontIns('\''), 't') => ParserState::DontIns('t'),
        (ParserState::DontIns('t'), '(') => ParserState::CondParenOpen,
        (ParserState::CondParenOpen, ')') => ParserState::EndCondInsInstruction,


        _ => ParserState::Invalid,
    }
}

pub fn day_3() {
    let input = include_str!("../data/day3.txt");
    let mut state = ParserState::Invalid;
    let mut num1: u32 = 0;
    let mut num2: u32 = 0;
    let mut sum: u32 = 0;
    let mut last_cond_part = true;
    let mut last_cond = true;
    for c in input.chars() {
        state = next_state(state, c);
        println!("{:?}", state);
        match state {
            ParserState::Num1Digit(d) => num1 = num1 * 10 + d as u32,
            ParserState::Num2Digit(d) => num2 = num2 * 10 + d as u32,
            ParserState::EndMulInstruction => {
                if last_cond {
                    sum += num1 * num2;
                }
                (num1, num2) = (0, 0);
            },
            ParserState::Invalid => {
                (num1, num2) = (0, 0);
            },

            ParserState::DoIns(_) => last_cond_part = true,
            ParserState::DontIns(_) => last_cond_part = false,

            ParserState::EndCondInsInstruction => last_cond = last_cond_part,

            _ => continue,
        }
    }

    println!("Sum is {}", sum);
}