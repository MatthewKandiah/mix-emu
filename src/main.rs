use mix::word::Word;

mod mix;

fn main() {
    let mut computer = mix::computer::Computer::new();
    let instruction = Word::from_u8s([0,0,0,0,0,8]).unwrap();
    computer.handle_instruction(instruction);
}
