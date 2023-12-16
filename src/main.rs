use mix_emu::{
    computer::{Computer, TapeUnit},
    data_types::{Sign, Word},
};

fn main() {
    let mut computer = Computer::new();
    // set initial registers state
    computer.registers.a = Word::from_byte_values(Sign::MINUS, 0, 0, 31, 32, 39).unwrap();
    computer.registers.x = Word::from_byte_values(Sign::PLUS, 37, 57, 47, 30, 30).unwrap();

    // memory 0 is first instruction run - use it to load program from tape
    let input_from_tape_to_1 = Word::from_instruction_parts(Sign::PLUS, 1, 0, 0, 36).unwrap();
    computer.memory.set(0, input_from_tape_to_1).unwrap();

    // tape unit with a single 100 word block
    let mut tape = TapeUnit::new(1);
    // convert to number
    tape.data[0][0] = Word::from_instruction_parts(Sign::PLUS, 0, 0, 0, 5).unwrap();
    // increment a
    tape.data[0][1] = Word::from_instruction_parts(Sign::PLUS, 1, 0, 0, 48).unwrap();
    // convert to characters codes
    tape.data[0][2] = Word::from_instruction_parts(Sign::PLUS, 0, 0, 1, 5).unwrap();
    // store a and x character codes in memory starting at 1000
    tape.data[0][3] = Word::from_instruction_parts(Sign::PLUS, 1000, 0, 5, 24).unwrap();
    tape.data[0][4] = Word::from_instruction_parts(Sign::PLUS, 1001, 0, 5, 31).unwrap();
    // output to line printer
    tape.data[0][5] = Word::from_instruction_parts(Sign::PLUS, 1000, 0, 18, 37).unwrap();

    // not written a run loop yet, so manually handle instructions
    // unused memory is all zeros, which are no-op instruction, so just make sure to run
    // enough without hitting the data in address 1000
    for _ in 0..100 {
        computer.handle_next_instruction();
    }
}
