use crate::computer::*;
use crate::data_types::*;

fn setup_computer() -> Computer {
    let mut computer = Computer::new();
    for i in 0..4000 {
        computer.memory.set(i, Word::from_i32(i).unwrap()).unwrap();
    }
    computer
}

#[test]
fn should_move_words() {
    let mut computer = setup_computer();
    computer.r_i1 = Index::from_i32(999).unwrap();

    let instruction = Word::from_byte_values(Sign::PLUS, 1000 / 64, 1000 % 64, 0, 3, 7).unwrap();
    computer.handle_instruction(instruction);

    for i in 0..4000 {
        let contents = computer.memory.get(i).unwrap();
        match i {
            999 => assert_eq!(contents.to_i32(), 1000),
            1000 => assert_eq!(contents.to_i32(), 1001),
            1001 => assert_eq!(contents.to_i32(), 1002),
            _ => assert_eq!(contents.to_i32(), i),
        }
    }
    assert_eq!(computer.r_i1.to_i32(), 1002);
}

#[test]
fn overlapping_region_edge_case() {
    let mut computer = setup_computer();
    computer.r_i1 = Index::from_i32(1001).unwrap();

    let instruction = Word::from_byte_values(Sign::PLUS, 1000 / 64, 1000 % 64, 0, 3, 7).unwrap();
    computer.handle_instruction(instruction);

    for i in 0..4000 {
        let contents = computer.memory.get(i).unwrap();
        match i {
            1001 => assert_eq!(contents.to_i32(), 1000),
            1002 => assert_eq!(contents.to_i32(), 1000),
            1003 => assert_eq!(contents.to_i32(), 1000),
            _ => assert_eq!(contents.to_i32(), i),
        }
    }
    assert_eq!(computer.r_i1.to_i32(), 1004);
}
