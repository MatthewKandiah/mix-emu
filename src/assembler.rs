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

    for line in source_lines {
        println!("{:?}", line);
    }

    // first pass - determine value of each symbol
    // types of symbol - EQU statements, asterisk addresses, local variables dH, dB, dF, ORIG
    // statements
    //
    // UNKNOWN:
    // - do you require an ORIG statement before the rest of a program?
    //
    // second pass - replace all symbols with values and generate instruction Words for output
    //
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
