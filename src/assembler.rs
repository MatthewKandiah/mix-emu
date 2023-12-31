use std::collections::HashMap;

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
    ORIG,
    NOP,
    ADD,
    SUB,
    MUL,
    DIV,
    NUM,
    CHAR,
    HLT,
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

#[derive(Debug)]
enum OpErr {
    InvalidString(String),
}

impl Op {
    fn from_str(input: &str) -> Result<Self, OpErr> {
        match input {
            "EQU" => Ok(Op::EQU),
            "ORIG" => Ok(Op::ORIG),
            "NOP" => Ok(Op::NOP),
            "ADD" => Ok(Op::ADD),
            "SUB" => Ok(Op::SUB),
            "MUL" => Ok(Op::MUL),
            "DIV" => Ok(Op::DIV),
            "NUM" => Ok(Op::NUM),
            "CHAR" => Ok(Op::CHAR),
            "HLT" => Ok(Op::HLT),
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
            "INPUT" => Ok(Op::IN),
            "OUTPUT" => Ok(Op::OUT),
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
            _ => Err(OpErr::InvalidString(input.to_string())),
        }
    }

    fn to_instruction_code(&self) -> i32 {
        match self {
            Op::EQU | Op::ORIG => panic!("Cannot generate instruction code for pseudo instruction"),
            Op::NOP => 0,
            Op::ADD => 1,
            Op::SUB => 2,
            Op::MUL => 3,
            Op::DIV => 4,
            Op::NUM | Op::CHAR | Op::HLT => 5,
            Op::SLA | Op::SRA | Op::SLAX | Op::SRAX | Op::SLC | Op::SRC => 6,
            Op::MOVE => 7,
            Op::LDA => 8,
            Op::LD1 => 9,
            Op::LD2 => 10,
            Op::LD3 => 11,
            Op::LD4 => 12,
            Op::LD5 => 13,
            Op::LD6 => 14,
            Op::LDX => 15,
            Op::LDAN => 16,
            Op::LD1N => 17,
            Op::LD2N => 18,
            Op::LD3N => 19,
            Op::LD4N => 20,
            Op::LD5N => 21,
            Op::LD6N => 22,
            Op::LDXN => 23,
            Op::STA => 24,
            Op::ST1 => 25,
            Op::ST2 => 26,
            Op::ST3 => 27,
            Op::ST4 => 28,
            Op::ST5 => 29,
            Op::ST6 => 30,
            Op::STX => 31,
            Op::STJ => 32,
            Op::STZ => 33,
            Op::JBUS => 34,
            Op::IOC => 35,
            Op::IN => 36,
            Op::OUT => 37,
            Op::JRED => 38,
            Op::JMP
            | Op::JSJ
            | Op::JOV
            | Op::JNOV
            | Op::JL
            | Op::JE
            | Op::JG
            | Op::JGE
            | Op::JNE
            | Op::JLE => 39,
            Op::JAN | Op::JAZ | Op::JAP | Op::JANN | Op::JANZ | Op::JANP => 40,
            Op::J1N | Op::J1Z | Op::J1P | Op::J1NN | Op::J1NZ | Op::J1NP => 41,
            Op::J2N | Op::J2Z | Op::J2P | Op::J2NN | Op::J2NZ | Op::J2NP => 42,
            Op::J3N | Op::J3Z | Op::J3P | Op::J3NN | Op::J3NZ | Op::J3NP => 43,
            Op::J4N | Op::J4Z | Op::J4P | Op::J4NN | Op::J4NZ | Op::J4NP => 44,
            Op::J5N | Op::J5Z | Op::J5P | Op::J5NN | Op::J5NZ | Op::J5NP => 45,
            Op::J6N | Op::J6Z | Op::J6P | Op::J6NN | Op::J6NZ | Op::J6NP => 46,
            Op::JXN | Op::JXZ | Op::JXP | Op::JXNN | Op::JXNZ | Op::JXNP => 47,
            Op::INCA | Op::DECA | Op::ENTA | Op::ENNA => 48,
            Op::INC1 | Op::DEC1 | Op::ENT1 | Op::ENN1 => 49,
            Op::INC2 | Op::DEC2 | Op::ENT2 | Op::ENN2 => 50,
            Op::INC3 | Op::DEC3 | Op::ENT3 | Op::ENN3 => 51,
            Op::INC4 | Op::DEC4 | Op::ENT4 | Op::ENN4 => 52,
            Op::INC5 | Op::DEC5 | Op::ENT5 | Op::ENN5 => 53,
            Op::INC6 | Op::DEC6 | Op::ENT6 | Op::ENN6 => 54,
            Op::INCX | Op::DECX | Op::ENTX | Op::ENNX => 55,
            Op::CMPA => 56,
            Op::CMP1 => 57,
            Op::CMP2 => 58,
            Op::CMP3 => 59,
            Op::CMP4 => 60,
            Op::CMP5 => 61,
            Op::CMP6 => 62,
            Op::CMPX => 63,
        }
    }

    fn default_field(&self) -> Field {
        match self {
            Op::EQU | Op::ORIG => panic!("Pseudo instructions do not have a default field"),
            Op::NOP
            | Op::NUM
            | Op::SLA
            | Op::JBUS
            | Op::IOC
            | Op::IN
            | Op::OUT
            | Op::JRED
            | Op::JMP
            | Op::JAN
            | Op::J1N
            | Op::J2N
            | Op::J3N
            | Op::J4N
            | Op::J5N
            | Op::J6N
            | Op::JXN
            | Op::INCA
            | Op::INC1
            | Op::INC2
            | Op::INC3
            | Op::INC4
            | Op::INC5
            | Op::INC6
            | Op::INCX => Field {
                left: SymbolOrInt::Int(0),
                right: SymbolOrInt::Int(0),
            },
            Op::CHAR
            | Op::SRA
            | Op::MOVE
            | Op::JSJ
            | Op::JAZ
            | Op::J1Z
            | Op::J2Z
            | Op::J3Z
            | Op::J4Z
            | Op::J5Z
            | Op::J6Z
            | Op::JXZ
            | Op::DECA
            | Op::DEC1
            | Op::DEC2
            | Op::DEC3
            | Op::DEC4
            | Op::DEC5
            | Op::DEC6
            | Op::DECX => Field {
                left: SymbolOrInt::Int(0),
                right: SymbolOrInt::Int(1),
            },
            Op::HLT
            | Op::SLAX
            | Op::STJ
            | Op::JOV
            | Op::JAP
            | Op::J1P
            | Op::J2P
            | Op::J3P
            | Op::J4P
            | Op::J5P
            | Op::J6P
            | Op::JXP
            | Op::ENTA
            | Op::ENT1
            | Op::ENT2
            | Op::ENT3
            | Op::ENT4
            | Op::ENT5
            | Op::ENT6
            | Op::ENTX => Field {
                left: SymbolOrInt::Int(0),
                right: SymbolOrInt::Int(2),
            },
            Op::SRAX
            | Op::JNOV
            | Op::JANN
            | Op::J1NN
            | Op::J2NN
            | Op::J3NN
            | Op::J4NN
            | Op::J5NN
            | Op::J6NN
            | Op::JXNN
            | Op::ENNA
            | Op::ENN1
            | Op::ENN2
            | Op::ENN3
            | Op::ENN4
            | Op::ENN5
            | Op::ENN6
            | Op::ENNX => Field {
                left: SymbolOrInt::Int(0),
                right: SymbolOrInt::Int(3),
            },
            Op::SLC
            | Op::JL
            | Op::JANZ
            | Op::J1NZ
            | Op::J2NZ
            | Op::J3NZ
            | Op::J4NZ
            | Op::J5NZ
            | Op::J6NZ
            | Op::JXNZ => Field {
                left: SymbolOrInt::Int(0),
                right: SymbolOrInt::Int(4),
            },
            Op::ADD
            | Op::SUB
            | Op::MUL
            | Op::DIV
            | Op::SRC
            | Op::LDA
            | Op::LD1
            | Op::LD2
            | Op::LD3
            | Op::LD4
            | Op::LD5
            | Op::LD6
            | Op::LDX
            | Op::LDAN
            | Op::LD1N
            | Op::LD2N
            | Op::LD3N
            | Op::LD4N
            | Op::LD5N
            | Op::LD6N
            | Op::LDXN
            | Op::STA
            | Op::ST1
            | Op::ST2
            | Op::ST3
            | Op::ST4
            | Op::ST5
            | Op::ST6
            | Op::STX
            | Op::STZ
            | Op::JE
            | Op::JANP
            | Op::J1NP
            | Op::J2NP
            | Op::J3NP
            | Op::J4NP
            | Op::J5NP
            | Op::J6NP
            | Op::JXNP
            | Op::CMPA
            | Op::CMP1
            | Op::CMP2
            | Op::CMP3
            | Op::CMP4
            | Op::CMP5
            | Op::CMP6
            | Op::CMPX => Field::from_int(5),
            Op::JG => Field::from_int(6),
            Op::JGE => Field::from_int(7),
            Op::JNE => Field::from_int(8),
            Op::JLE => Field::from_int(9),
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
    left: i32,
    right: i32,
}

impl Field {
    fn from_left_right(left: i32, right: i32) -> Self {
        Self { left, right }
    }

    fn from_int(value: i32) -> Self {
        Self {
            left: value / 8,
            right: value % 8,
        }
    }
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
    let source_lines: Vec<SourceLine> = source
        .split('\n')
        .filter(is_not_empty)
        .filter(is_not_comment)
        .map(parse_line)
        .collect();
    let mut line_count = 0;
    let mut symbol_table: HashMap<String, SymbolOrInt> = HashMap::new();
    let mut result: Vec<Word> = vec![];
    for source_line in source_lines {
        match source_line.op {
            Op::EQU => todo!("update symbol table"),
            Op::ORIG => todo!("update line_count"),
            x => todo!("append instruction word to result"),
        };
    }
    result
}

fn symbol_lookup(
    symbol: &str,
    symbol_table: &HashMap<String, SymbolOrInt>,
    previous_keys: Vec<&str>,
) -> i32 {
    if previous_keys.contains(&symbol) {
        panic!("Illegal circular symbol definition {:?}", previous_keys);
    }
    match symbol_table.get(symbol) {
        None => panic!("Failed symbol lookup {:?}", symbol),
        Some(SymbolOrInt::Int(x)) => *x,
        Some(SymbolOrInt::Symbol(sym)) => {
            let mut next_previous_keys = previous_keys;
            next_previous_keys.push(sym);
            return symbol_lookup(&sym, symbol_table, next_previous_keys);
        }
    }
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
    unimplemented!()
}
