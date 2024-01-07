use std::{iter::Peekable, str::Chars};

use crate::data_types::Word;

#[derive(Debug)]
pub enum AssemblerError {
    BadLine(String),
    BadOpCode(Token),
}

#[derive(Debug, PartialEq, Eq)]
enum Op {
    EQU,
    ORIG,
    CON,
    ALF,
    END,
    NOP,
    ADD,
    SUB,
    MUL,
    DIV,
    NUM,
    CHAR,
    HALT,
    SLA,
    SRA,
    SLAX,
    SRAX,
    SLC,
    SRC,
    MOVE,
    LDA,
    LD1,
    LD2,
    LD3,
    LD4,
    LD5,
    LD6,
    LDX,
    LDAN,
    LD1N,
    LD2N,
    LD3N,
    LD4N,
    LD5N,
    LD6N,
    LDXN,
    STA,
    ST1,
    ST2,
    ST3,
    ST4,
    ST5,
    ST6,
    STX,
    STJ,
    STZ,
    JBUS,
    IOC,
    IN,
    OUT,
    JRED,
    JMP,
    JSJ,
    JOV,
    JNOV,
    JL,
    JE,
    JG,
    JGE,
    JNE,
    JLE,
    JAN,
    JAZ,
    JAP,
    JANN,
    JANZ,
    JANP,
    J1N,
    J1Z,
    J1P,
    J1NN,
    J1NZ,
    J1NP,
    J2N,
    J2Z,
    J2P,
    J2NN,
    J2NZ,
    J2NP,
    J3N,
    J3Z,
    J3P,
    J3NN,
    J3NZ,
    J3NP,
    J4N,
    J4Z,
    J4P,
    J4NN,
    J4NZ,
    J4NP,
    J5N,
    J5Z,
    J5P,
    J5NN,
    J5NZ,
    J5NP,
    J6N,
    J6Z,
    J6P,
    J6NN,
    J6NZ,
    J6NP,
    JXN,
    JXZ,
    JXP,
    JXNN,
    JXNZ,
    JXNP,
    INCA,
    DECA,
    ENTA,
    ENNA,
    INC1,
    DEC1,
    ENT1,
    ENN1,
    INC2,
    DEC2,
    ENT2,
    ENN2,
    INC3,
    DEC3,
    ENT3,
    ENN3,
    INC4,
    DEC4,
    ENT4,
    ENN4,
    INC5,
    DEC5,
    ENT5,
    ENN5,
    INC6,
    DEC6,
    ENT6,
    ENN6,
    INCX,
    DECX,
    ENTX,
    ENNX,
    CMPA,
    CMP1,
    CMP2,
    CMP3,
    CMP4,
    CMP5,
    CMP6,
    CMPX,
}

impl Op {
    fn from_token(t: Token) -> Result<Self, AssemblerError> {
        match t {
            Token::Symbol(x) => Self::from_str(&x),
            _ => Err(AssemblerError::BadOpCode(t)),
        }
    }

    fn from_str(s: &str) -> Result<Self, AssemblerError> {
        match s {
            "EQU" => Ok(Op::EQU),
            "ORIG" => Ok(Op::ORIG),
            "CON" => Ok(Op::CON),
            "ALF" => Ok(Op::ALF),
            "END" => Ok(Op::END),
            "NOP" => Ok(Op::NOP),
            "ADD" => Ok(Op::ADD),
            "SUB" => Ok(Op::SUB),
            "MUL" => Ok(Op::MUL),
            "DIV" => Ok(Op::DIV),
            "NUM" => Ok(Op::NUM),
            "CHAR" => Ok(Op::CHAR),
            "HALT" => Ok(Op::HALT),
            "SLA" => Ok(Op::SLA),
            "SRA" => Ok(Op::SRA),
            "SLAX" => Ok(Op::SLAX),
            "SRAX" => Ok(Op::SRAX),
            "SLC" => Ok(Op::SLC),
            "SRC" => Ok(Op::SRC),
            "MOVE" => Ok(Op::MOVE),
            "LDA" => Ok(Op::LDA),
            "LD1" => Ok(Op::LD1),
            "LD2" => Ok(Op::LD2),
            "LD3" => Ok(Op::LD3),
            "LD4" => Ok(Op::LD4),
            "LD5" => Ok(Op::LD5),
            "LD6" => Ok(Op::LD6),
            "LDX" => Ok(Op::LDX),
            "LDAN" => Ok(Op::LDAN),
            "LD1N" => Ok(Op::LD1N),
            "LD2N" => Ok(Op::LD2N),
            "LD3N" => Ok(Op::LD3N),
            "LD4N" => Ok(Op::LD4N),
            "LD5N" => Ok(Op::LD5N),
            "LD6N" => Ok(Op::LD6N),
            "LDXN" => Ok(Op::LDXN),
            "STA" => Ok(Op::STA),
            "ST1" => Ok(Op::ST1),
            "ST2" => Ok(Op::ST2),
            "ST3" => Ok(Op::ST3),
            "ST4" => Ok(Op::ST4),
            "ST5" => Ok(Op::ST5),
            "ST6" => Ok(Op::ST6),
            "STX" => Ok(Op::STX),
            "STJ" => Ok(Op::STJ),
            "STZ" => Ok(Op::STZ),
            "JBUS" => Ok(Op::JBUS),
            "IOC" => Ok(Op::IOC),
            "IN" => Ok(Op::IN),
            "OUT" => Ok(Op::OUT),
            "JRED" => Ok(Op::JRED),
            "JMP" => Ok(Op::JMP),
            "JSJ" => Ok(Op::JSJ),
            "JOV" => Ok(Op::JOV),
            "JNOV" => Ok(Op::JNOV),
            "JL" => Ok(Op::JL),
            "JE" => Ok(Op::JE),
            "JG" => Ok(Op::JG),
            "JGE" => Ok(Op::JGE),
            "JNE" => Ok(Op::JNE),
            "JLE" => Ok(Op::JLE),
            "JAN" => Ok(Op::JAN),
            "JAZ" => Ok(Op::JAZ),
            "JAP" => Ok(Op::JAP),
            "JANN" => Ok(Op::JANN),
            "JANZ" => Ok(Op::JANZ),
            "JANP" => Ok(Op::JANP),
            "J1N" => Ok(Op::J1N),
            "J1Z" => Ok(Op::J1Z),
            "J1P" => Ok(Op::J1P),
            "J1NN" => Ok(Op::J1NN),
            "J1NZ" => Ok(Op::J1NZ),
            "J1NP" => Ok(Op::J1NP),
            "J2N" => Ok(Op::J2N),
            "J2Z" => Ok(Op::J2Z),
            "J2P" => Ok(Op::J2P),
            "J2NN" => Ok(Op::J2NN),
            "J2NZ" => Ok(Op::J2NZ),
            "J2NP" => Ok(Op::J2NP),
            "J3N" => Ok(Op::J3N),
            "J3Z" => Ok(Op::J3Z),
            "J3P" => Ok(Op::J3P),
            "J3NN" => Ok(Op::J3NN),
            "J3NZ" => Ok(Op::J3NZ),
            "J3NP" => Ok(Op::J3NP),
            "J4N" => Ok(Op::J4N),
            "J4Z" => Ok(Op::J4Z),
            "J4P" => Ok(Op::J4P),
            "J4NN" => Ok(Op::J4NN),
            "J4NZ" => Ok(Op::J4NZ),
            "J4NP" => Ok(Op::J4NP),
            "J5N" => Ok(Op::J5N),
            "J5Z" => Ok(Op::J5Z),
            "J5P" => Ok(Op::J5P),
            "J5NN" => Ok(Op::J5NN),
            "J5NZ" => Ok(Op::J5NZ),
            "J5NP" => Ok(Op::J5NP),
            "J6N" => Ok(Op::J6N),
            "J6Z" => Ok(Op::J6Z),
            "J6P" => Ok(Op::J6P),
            "J6NN" => Ok(Op::J6NN),
            "J6NZ" => Ok(Op::J6NZ),
            "J6NP" => Ok(Op::J6NP),
            "JXN" => Ok(Op::JXN),
            "JXZ" => Ok(Op::JXZ),
            "JXP" => Ok(Op::JXP),
            "JXNN" => Ok(Op::JXNN),
            "JXNZ" => Ok(Op::JXNZ),
            "JXNP" => Ok(Op::JXNP),
            "INCA" => Ok(Op::INCA),
            "DECA" => Ok(Op::DECA),
            "ENTA" => Ok(Op::ENTA),
            "ENNA" => Ok(Op::ENNA),
            "INC1" => Ok(Op::INC1),
            "DEC1" => Ok(Op::DEC1),
            "ENT1" => Ok(Op::ENT1),
            "ENN1" => Ok(Op::ENN1),
            "INC2" => Ok(Op::INC2),
            "DEC2" => Ok(Op::DEC2),
            "ENT2" => Ok(Op::ENT2),
            "ENN2" => Ok(Op::ENN2),
            "INC3" => Ok(Op::INC3),
            "DEC3" => Ok(Op::DEC3),
            "ENT3" => Ok(Op::ENT3),
            "ENN3" => Ok(Op::ENN3),
            "INC4" => Ok(Op::INC4),
            "DEC4" => Ok(Op::DEC4),
            "ENT4" => Ok(Op::ENT4),
            "ENN4" => Ok(Op::ENN4),
            "INC5" => Ok(Op::INC5),
            "DEC5" => Ok(Op::DEC5),
            "ENT5" => Ok(Op::ENT5),
            "ENN5" => Ok(Op::ENN5),
            "INC6" => Ok(Op::INC6),
            "DEC6" => Ok(Op::DEC6),
            "ENT6" => Ok(Op::ENT6),
            "ENN6" => Ok(Op::ENN6),
            "INCX" => Ok(Op::INCX),
            "DECX" => Ok(Op::DECX),
            "ENTX" => Ok(Op::ENTX),
            "ENNX" => Ok(Op::ENNX),
            "CMPA" => Ok(Op::CMPA),
            "CMP1" => Ok(Op::CMP1),
            "CMP2" => Ok(Op::CMP2),
            "CMP3" => Ok(Op::CMP3),
            "CMP4" => Ok(Op::CMP4),
            "CMP5" => Ok(Op::CMP5),
            "CMP6" => Ok(Op::CMP6),
            "CMPX" => Ok(Op::CMPX),
            s => Err(AssemblerError::BadOpCode(Token::Symbol(s.to_string()))),
        }
    }
}

// using the terminal input rules -> empty LOC indicated by leading space
// changing ALF so characters are enclosed in "s instead of working by character count
pub fn read_source_string_as_instructions(
    source_content: &str,
) -> Vec<Word> {
    let lines = split_tokens_by_line(tokenise(source_content));
    return parse_lines(lines);
}

fn parse_lines(lines: Vec<Vec<Token>>) -> Vec<Word> {
    let mut result = Vec::new();
    for line in lines {
        if let Some(x) = parse_line(line) {
            result.push(x);
        }
    }
    return result;
}

fn parse_line(line: Vec<Token>) -> Option<Word> {
    unimplemented!("{:?}", line)
}

fn split_tokens_by_line(tokens: Vec<Token>) -> Vec<Vec<Token>> {
    let mut lines = Vec::new();
    let mut line = Vec::new();
    for token in tokens {
        match token {
            Token::LineBreak => {
                lines.push(line.clone());
                line.clear();
            }
            x => line.push(x),
        };
    }
    return lines;
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Illegal(String),
    Symbol(String),
    Number(i32),
    Plus,
    Minus,
    Asterisk,
    Slash,
    DoubleSlash,
    Colon,
    LeftBracket,
    RightBracket,
    LineBreak,
    Space,
    DoubleQuote,
}

fn tokenise(source_content: &str) -> Vec<Token> {
    let mut iter = source_content.chars().into_iter().peekable();
    let mut result: Vec<Token> = Vec::new();
    while iter.peek() != None {
        result.push(match iter.next() {
            None => break,
            Some(x) => get_token(&mut iter, x),
        });
    }
    return result;
}

#[test]
fn should_tokenise_correctly() {
    use Token::*;
    let input = "1\n2\nword\n1337code toolongtotokenise    123456789123456789)(:/sep//*-\"+&";
    let results = tokenise(input);
    let expected = vec![
        Number(1),
        LineBreak,
        Number(2),
        LineBreak,
        Symbol("word".to_string()),
        LineBreak,
        Symbol("1337code".to_string()),
        Space,
        Illegal("toolongtotokenise".to_string()),
        Space,
        Illegal("123456789123456789".to_string()),
        RightBracket,
        LeftBracket,
        Colon,
        Slash,
        Symbol("sep".to_string()),
        DoubleSlash,
        Asterisk,
        Minus,
        DoubleQuote,
        Plus,
        Illegal("&".to_string()),
    ];
    for (idx, result) in results.iter().enumerate() {
        assert_eq!(result, &expected[idx]);
    }
}

fn get_token(iter: &mut Peekable<Chars>, current_value: char) -> Token {
    use Token::*;
    match current_value {
        '\n' => LineBreak,
        ' ' => absorb_consecutive_spaces_into_single_token(iter),
        ')' => RightBracket,
        '(' => LeftBracket,
        ':' => Colon,
        '/' => handle_slash_or_double_slash(iter),
        '*' => Asterisk,
        '-' => Minus,
        '+' => Plus,
        '"' => DoubleQuote,
        x if x.is_ascii_alphanumeric() => {
            absorb_consecutive_alphanumerics_into_single_token(iter, x)
        }
        _ => Illegal(current_value.to_string()),
    }
}

fn handle_slash_or_double_slash(iter: &mut Peekable<Chars>) -> Token {
    match iter.peek() {
        Some('/') => {
            iter.next();
            Token::DoubleSlash
        }
        _ => Token::Slash,
    }
}

fn absorb_consecutive_alphanumerics_into_single_token(
    iter: &mut Peekable<Chars>,
    current_value: char,
) -> Token {
    let mut buffer = vec![current_value];
    const NON_ASCII_ALPHANUMERIC_CHAR: char = '/'; // lazy hack to make next statement false when iter.peek() == None
    while iter
        .peek()
        .unwrap_or(&NON_ASCII_ALPHANUMERIC_CHAR)
        .is_ascii_alphanumeric()
    {
        match iter.next() {
            Some(y) if y.is_ascii_alphanumeric() => {
                buffer.push(y);
            }
            _ => panic!("Previous checks should make this impossible"),
        };
    }
    let string_value: String = buffer.iter().collect();
    if buffer.len() > 10 {
        return Token::Illegal(string_value);
    }
    return match string_value.parse::<i32>() {
        Ok(z) => Token::Number(z),
        _ => Token::Symbol(string_value),
    };
}

fn absorb_consecutive_spaces_into_single_token(iter: &mut Peekable<Chars>) -> Token {
    while iter.peek() == Some(&' ') {
        iter.next();
    }
    return Token::Space;
}
