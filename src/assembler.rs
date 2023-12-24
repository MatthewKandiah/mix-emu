use crate::data_types::Word;

struct SourceLine {
    loc: Option<String>,
    op: Op,
    address: Option<Address>,
}

enum Op {
    EQU,
}

struct Address {
    value: SymbolOrInt,
    index: Option<SymbolOrInt>,
    field: Option<Field>,
}

struct Field {
    left: SymbolOrInt,
    right: SymbolOrInt,
}

enum SymbolOrInt {
    Symbol(String),
    Int(i32),
}

pub fn read_source_string_as_instructions(source: String) -> Vec<Word> {
    let source_lines = source
        .split('\n')
        .filter(is_not_empty)
        .filter(is_not_comment)
        .map(parse_line);

    return vec![];
}

fn is_not_empty(line: &&str) -> bool {
    !line.is_empty()
}

fn is_not_comment(line: &&str) -> bool {
    line.chars().next() != Some('*')
}

fn parse_line(line: &str) -> SourceLine {
    unimplemented!();
}
