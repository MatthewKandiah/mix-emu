use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("USAGE: pass in paths to MIXAL source files");
    }

    let path = &args[1];

    println!("source file: {}", path);
}
