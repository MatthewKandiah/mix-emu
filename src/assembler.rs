use std::{iter::Peekable, str::Chars};

use crate::data_types::Word;

#[derive(Debug)]
pub enum AssemblerError {
    BadLine(String),
}

pub fn read_source_string_as_instructions(
    source_content: &str,
) -> Result<Vec<Word>, AssemblerError> {
    let tokens: Vec<Token> = tokenise(source_content);
    unimplemented!();
}

#[derive(Debug, PartialEq, Eq)]
enum Token {
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
    let input = "1\n2\nword\n1337code toolongtotokenise    123456789123456789)(:/sep//*-+&";
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
