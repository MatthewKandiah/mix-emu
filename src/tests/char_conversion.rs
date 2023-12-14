use crate::computer::*;
use crate::data_types::*;

#[test]
fn should_convert_to_numerical_value() {
    let mut computer = Computer::new();
    computer.registers.a = Word::from_byte_values(Sign::MINUS, 0, 0, 31, 32, 39).unwrap();
    computer.registers.x = Word::from_byte_values(Sign::PLUS, 37, 57, 47, 30, 30).unwrap();

    let instruction = Word::from_instruction_parts(Sign::PLUS, 0, 0, 0, 5).unwrap();
    computer.handle_instruction(instruction);

    assert_eq!(computer.registers.a.to_i32(), -12977700);
    assert_eq!(
        computer.registers.x,
        Word::from_byte_values(Sign::PLUS, 37, 57, 47, 30, 30).unwrap()
    );
}

#[test]
fn should_convert_to_char_codes() {
    let mut computer = Computer::new();
    computer.registers.a = Word::from_i32(-12977699).unwrap();
    computer.registers.x = Word::from_byte_values(Sign::PLUS, 37, 57, 47, 30, 30).unwrap();

    let instruction = Word::from_instruction_parts(Sign::PLUS, 0, 0, 1, 5).unwrap();
    computer.handle_instruction(instruction);

    assert_eq!(computer.registers.x.sign, Sign::PLUS);
    assert_eq!(computer.registers.x.bytes.4.to_i32(), 39);
    assert_eq!(computer.registers.x.bytes.3.to_i32(), 39);
    assert_eq!(computer.registers.x.bytes.2.to_i32(), 36);
    assert_eq!(computer.registers.x.bytes.1.to_i32(), 37);
    assert_eq!(computer.registers.x.bytes.0.to_i32(), 37);
    assert_eq!(computer.registers.a.sign, Sign::MINUS);
    assert_eq!(computer.registers.a.bytes.4.to_i32(), 39);
    assert_eq!(computer.registers.a.bytes.3.to_i32(), 32);
    assert_eq!(computer.registers.a.bytes.2.to_i32(), 31);
    assert_eq!(computer.registers.a.bytes.1.to_i32(), 30);
    assert_eq!(computer.registers.a.bytes.0.to_i32(), 30);
}
