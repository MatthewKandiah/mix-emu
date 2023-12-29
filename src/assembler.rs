use crate::data_types::Word;

struct SourceLine {
    loc: Option<Loc>,
    op: Op,
    address: Option<Address>,
}

struct Loc {
    value: SymbolOrInt,
}

impl Loc {
    fn from_str(input: &str) -> Self {
        if let Result::Err(op) = Op::from_str(input) {
            panic!(
                "Illegal loc - op names may not be used as loc names: {:?}",
                op
            );
        }
        Self {
            value: SymbolOrInt::from_str(input),
        }
    }
}

enum Op {
    EQU,
}

#[derive(Debug)]
enum OpErr {
    InvalidString(String),
}

impl Op {
    fn from_str(input: &str) -> Result<Self, OpErr> {
        match input {
            "EQU" => Ok(Self::EQU),
            _ => Err(OpErr::InvalidString(input.to_string())),
        }
    }
}

struct Address {
    value: SymbolOrInt,
    index: Option<SymbolOrInt>,
    field: Option<Field>,
}

impl Address {
    fn from_str(input: &str) -> Self {
        unimplemented!()
    }
}

struct Field {
    left: SymbolOrInt,
    right: SymbolOrInt,
}

enum SymbolOrInt {
    Symbol(String),
    Int(i32),
}

impl SymbolOrInt {
    fn from_str(input: &str) -> Self {
        if let Ok(val) = input.parse::<i32>() {
            return SymbolOrInt::Int(val);
        } else {
            return SymbolOrInt::Symbol(input.to_string());
        }
    }
}

pub fn read_source_string_as_instructions(source: String) -> Vec<Word> {
    source
        .split('\n')
        .filter(is_not_empty)
        .filter(is_not_comment)
        .map(parse_line)
        .map(generate_instruction)
        .flatten()
        .collect()
}

fn is_not_empty(line: &&str) -> bool {
    !line.is_empty()
}

fn is_not_comment(line: &&str) -> bool {
    line.chars().next() != Some('*')
}

fn parse_line(line: &str) -> SourceLine {
    let split_line: Vec<&str> = line.split_whitespace().collect();
    let fixed_split_line = join_splits_within_quotes(split_line);
    match fixed_split_line.len() {
        1 => SourceLine {
            loc: None,
            op: Op::from_str(&fixed_split_line[0]).unwrap(),
            address: None,
        },
        2 => parse_2_column_line(&fixed_split_line[0], &fixed_split_line[1]),
        3 => SourceLine {
            loc: Some(Loc::from_str(&fixed_split_line[0])),
            op: Op::from_str(&fixed_split_line[1]).unwrap(),
            address: Some(Address::from_str(&fixed_split_line[2])),
        },
        _ => panic!(
            "Parsing error - line split into illegal number of columns: {}",
            fixed_split_line.len()
        ),
    }
}

// an ALF instruction may contain a space character inside quotes
// This will be the last thing in its line, and should have the `"` as its first character,
// so we can look for an opening quote, and then join all the elements from that point on
fn join_splits_within_quotes(split_line: Vec<&str>) -> Vec<String> {
    let double_quote_start_index = split_line
        .iter()
        .position(|part| part.chars().next() == Some('"'));
    if let Some(x) = double_quote_start_index {
        let mut result: Vec<String> = split_line[0..x]
            .iter()
            .map(|part| part.to_string())
            .collect();
        result.push(split_line[x..].join(" "));
        return result;
    } else {
        return split_line.iter().map(|part| part.to_string()).collect();
    }
}

fn parse_2_column_line(first_column: &str, second_column: &str) -> SourceLine {
    match Op::from_str(first_column) {
        Ok(op) => SourceLine {
            loc: None,
            op,
            address: Some(Address::from_str(second_column)),
        },
        Err(_) => SourceLine {
            loc: Some(Loc::from_str(first_column)),
            op: Op::from_str(second_column).unwrap(),
            address: None,
        },
    }
}

fn generate_instruction(source_line: SourceLine) -> Option<Word> {
    unimplemented!();
}
