use crate::computer::*;
use crate::data_types::*;

#[test]
fn should_start_running_from_current_address() {
    let mut computer = Computer::new();
    computer.current_instruction_address = 7;
    computer.running = false;
    // enta instruction
    let instruction = Word::from_instruction_parts(Sign::PLUS, 1234, 0, 2, 48).unwrap();
    computer.memory.set(7, instruction).unwrap();

    computer.start();

    assert_eq!(computer.running, true);
    assert_eq!(computer.current_instruction_address, 8);
    assert_eq!(computer.r_a.to_i32(), 1234);
}

#[test]
fn should_do_nothing_if_not_running() {
    let mut computer = Computer::new();
    computer.running = false;
    computer.current_instruction_address = 1;
    // enta instruction
    let instruction = Word::from_instruction_parts(Sign::PLUS, 1234, 0, 2, 48).unwrap();
    computer.memory.set(1, instruction).unwrap();

    computer.handle_next_instruction();

    assert_eq!(computer.current_instruction_address, 1);
    assert_eq!(computer.r_a.to_i32(), 0);
}

#[test]
fn should_increment_current_instruction_address() {
    let mut computer = Computer::new();
    computer.running = true;
    computer.current_instruction_address = 1;
    computer.handle_next_instruction();
    assert_eq!(computer.current_instruction_address, 2);
    computer.handle_next_instruction();
    assert_eq!(computer.current_instruction_address, 3);
}

#[test]
fn should_execute_instructions_from_memory() {
    let mut computer = Computer::new();
    computer.running = true;
    computer.current_instruction_address = 1;
    // inca 100
    computer
        .memory
        .set(
            1,
            Word::from_instruction_parts(Sign::PLUS, 100, 0, 0, 48).unwrap(),
        )
        .unwrap();
    // inca 200
    computer
        .memory
        .set(
            2,
            Word::from_instruction_parts(Sign::PLUS, 200, 0, 0, 48).unwrap(),
        )
        .unwrap();
    // inca 300
    computer
        .memory
        .set(
            3,
            Word::from_instruction_parts(Sign::PLUS, 300, 0, 0, 48).unwrap(),
        )
        .unwrap();

    computer.handle_next_instruction();
    assert_eq!(computer.r_a.to_i32(), 100);
    computer.handle_next_instruction();
    assert_eq!(computer.r_a.to_i32(), 300);
    computer.handle_next_instruction();
    assert_eq!(computer.r_a.to_i32(), 600);
}

mod hlt {
    use crate::computer::*;
    use crate::data_types::*;

    fn setup_computer() -> Computer {
        let mut computer = Computer::new();
        let enta1 = Word::from_instruction_parts(Sign::PLUS, 1, 0, 2, 48).unwrap();
        let enta2 = Word::from_instruction_parts(Sign::PLUS, 2, 0, 2, 48).unwrap();
        let hlt = Word::from_instruction_parts(Sign::PLUS, 0, 0, 2, 5).unwrap();
        computer.memory.set(10, hlt).unwrap();
        computer.memory.set(11, enta1).unwrap();
        computer.memory.set(12, enta2).unwrap();
        computer.current_instruction_address = 10;
        computer.running = true;
        computer
    }

    #[test]
    fn should_halt_program_execution_and_restart_from_next_instruction() {
        let mut computer = setup_computer();
        // hlt
        computer.handle_next_instruction();
        assert!(!computer.running);

        computer.start();
        assert_eq!(computer.r_a.to_i32(), 1);
        assert_eq!(computer.current_instruction_address, 12);

        computer.handle_next_instruction();
        assert_eq!(computer.r_a.to_i32(), 2);
        assert_eq!(computer.current_instruction_address, 13);
    }
}
