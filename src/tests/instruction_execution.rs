use crate::computer::*;
use crate::data_types::*;

#[test]
fn should_increment_current_instruction_address() {
    let mut computer = Computer::new();
    computer.current_instruction_address = 1;
    computer.handle_next_instruction();
    assert_eq!(computer.current_instruction_address, 2);
    computer.handle_next_instruction();
    assert_eq!(computer.current_instruction_address, 3);
}

#[test]
fn should_execute_instructions_from_memory() {
    let mut computer = Computer::new();
    computer.current_instruction_address = 1;
    // inca 100
    computer.memory.set(1, Word::from_instruction_parts(Sign::PLUS, 100, 0, 0, 48).unwrap()).unwrap();
    // inca 200
    computer.memory.set(2, Word::from_instruction_parts(Sign::PLUS, 200, 0, 0, 48).unwrap()).unwrap();
    // inca 300
    computer.memory.set(3, Word::from_instruction_parts(Sign::PLUS, 300, 0, 0, 48).unwrap()).unwrap();

    computer.handle_next_instruction();
    assert_eq!(computer.r_a.to_i32(), 100);
    computer.handle_next_instruction();
    assert_eq!(computer.r_a.to_i32(), 300);
    computer.handle_next_instruction();
    assert_eq!(computer.r_a.to_i32(), 600);
}
