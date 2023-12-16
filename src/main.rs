use mix_emu::{
    computer::{Computer, TapeUnit},
    data_types::{Sign, Word},
};

fn main() {
    let mut computer = Computer::new();
    let a_sign = Sign::MINUS;
    let a0 = 0;
    let a1 = 0;
    let a2 = 31;
    let a3 = 32;
    let a4 = 39;
    let x_sign = Sign::PLUS;
    let x0 = 37;
    let x1 = 57;
    let x2 = 37;
    let x3 = 30;
    let x4 = 30;

    // memory 0 is first instruction run - use it to load program from tape
    // register x is initialised to 0, so we read the 0th block from the tape
    let input_from_tape_to_1 = Word::from_instruction_parts(Sign::PLUS, 1, 0, 0, 36).unwrap();
    computer.memory.set(0, input_from_tape_to_1).unwrap();

    // tape unit with a single 100 word block
    let mut tape = TapeUnit::new(1);
    // call it convention that the program is loaded into memory at address 1, so absolute
    // memory addresses will assume this
    // lda with whole word from 10
    tape.data[0][0] = Word::from_instruction_parts(Sign::PLUS, 10, 0, 5, 8).unwrap();
    // ldx with whole word from 11
    tape.data[0][1] = Word::from_instruction_parts(Sign::PLUS, 11, 0, 5, 15).unwrap();
    // convert to number
    tape.data[0][2] = Word::from_instruction_parts(Sign::PLUS, 0, 0, 0, 5).unwrap();
    // increment a
    tape.data[0][3] = Word::from_instruction_parts(Sign::PLUS, 1, 0, 0, 48).unwrap();
    // convert to characters codes
    tape.data[0][4] = Word::from_instruction_parts(Sign::PLUS, 0, 0, 1, 5).unwrap();
    // store a and x character codes in memory starting at 1000
    tape.data[0][5] = Word::from_instruction_parts(Sign::PLUS, 1000, 0, 5, 24).unwrap();
    tape.data[0][6] = Word::from_instruction_parts(Sign::PLUS, 1001, 0, 5, 31).unwrap();
    // output to line printer
    tape.data[0][7] = Word::from_instruction_parts(Sign::PLUS, 1000, 0, 18, 37).unwrap();
    // halt - stop us running into the data block and breaking things
    tape.data[0][8] = Word::from_instruction_parts(Sign::PLUS, 0, 0, 2, 5).unwrap();

    // data block
    tape.data[0][9] = Word::from_byte_values(a_sign, a0, a1, a2, a3, a4).unwrap();
    tape.data[0][10] = Word::from_byte_values(x_sign, x0, x1, x2, x3, x4).unwrap();
    // not written a run loop yet, so manually handle instructions
    // unused memory is all zeros, which are no-op instruction, so just make sure to run
    // enough without hitting the data in address 1000
    computer.tape_unit = tape;
    computer.start();
    for _ in 0..100 {
        computer.handle_next_instruction();
    }
}
