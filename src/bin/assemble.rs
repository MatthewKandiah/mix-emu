use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("USAGE: pass in paths to MIXAL source files");
    }
    if args.len() != 2 {
        panic!("USAGE: only pass in one MIXAL source file");
    }

    let path = &args[1];
    let source_content = fs::read_to_string(path).expect("Failed to read source file");
    let source_lines: Vec<Vec<&str>> = source_content
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

    for (idx, line) in source_lines.iter().enumerate() {
        println!("{}", idx);
        println!("{:?}", line);
    }

    // preprocess source lines -> replace all symbols with values
    //
    // generate instructions
    //
    // what does our output look like? Should this be changed to a library instead of an
    // executable? Could add helper methods on IO devices to load assembled instructions on to
    // them
}
