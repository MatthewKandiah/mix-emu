use mix_emu::assembler;
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
    let instructions = assembler::read_source_string_as_instructions(source_content);
    for instruction in instructions.iter() {
        println!("{:?}", instruction);
    }
}
