use crate::data_types::Word;

pub fn read_source_string_as_instructions(source: String) -> Vec<Word> {
    let source_lines: Vec<Vec<&str>> = source
        .split('\n')
        .map(str::split_whitespace)
        .map(Iterator::collect)
        .filter(|x: &Vec<&str>| !x.is_empty())
        .filter(|x: &Vec<&str>| {
            let is_comment = x
                .first()
                .expect("Should have already filtered out empty lines")
                .chars()
                .next()
                .expect("Should have stripped out leading whitespace characters")
                == '*';
            return !is_comment;
        })
        .collect();

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
