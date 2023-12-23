use std::collections::HashMap;

use crate::data_types::Word;

type LineParts<'a> = (Option<&'a str>, &'a str, &'a str);

pub fn read_source_string_as_instructions(source: String) -> Vec<Word> {
    let source_lines: Vec<LineParts> = source
        .split('\n')
        .map(str::split_whitespace)
        .map(Iterator::collect)
        .filter(|x: &Vec<&str>| !x.is_empty())
        .filter(is_not_comment)
        .map(parse_line_parts)
        .collect();

    // a symbol can be defined equal to another symbol, which may not have a value yet
    // handle this by storing their values as strings, attempting to parse value as int, and
    // if that fails, then in the replacement step we will use the retrieve value as a search key in the hashmap again. If we ever fail to
    // find the symbol then panic, the program has failed to define all its symbols.
    let mut symbols = HashMap::<String, String>::new();
    for line in source_lines {
        if line.1 == "EQU" {
            if line.0 == None {
                panic!("EQU statement requires a LOC field");
            }
            let sym = line.0.unwrap();
            if symbols.contains_key(sym) {
                panic!("Redefining global symbol not allowed in EQU statement");
            }
            symbols.insert(sym.to_string(), line.2.to_string());
        }
    }
    // now we know what global symbols are defined, we can generate non-clashing names for local
    // symbols. Iterate through source, and replace dH, dF, dB with non-clashing symbol names
    let mut result: Vec<Word> = vec![];
    result.push(Word::ZERO);
    return result;
}

fn is_comment(line: &Vec<&str>) -> bool {
    line.first()
        .expect("Should have already filtered out empty lines")
        .chars()
        .next()
        .expect("Should have stripped out leading whitespace characters")
        == '*'
}

fn is_not_comment(line: &Vec<&str>) -> bool {
    !is_comment(line)
}

fn parse_line_parts(line: Vec<&str>) -> LineParts {
    if line.len() == 2 {
        return (None, line[0], line[1]);
    }
    if line.len() == 3 {
        return (Some(line[0]), line[1], line[2]);
    }
    panic!("Invalid line");
}
