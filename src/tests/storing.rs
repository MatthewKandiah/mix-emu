mod sta_tests {
    use crate::{computer::*, data_types::*};

    fn setup_computer() -> Computer {
        let mut computer = Computer::new();
        computer
            .memory
            .set(
                2000,
                Word::from_byte_values(Sign::MINUS, 1, 2, 3, 4, 5).unwrap(),
            )
            .unwrap();
        computer.r_a = Word::from_byte_values(Sign::PLUS, 6, 7, 8, 9, 0).unwrap();
        computer
    }

    #[test]
    fn should_store_contents_in_memory() {
        let mut computer = setup_computer();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 2000, 0, 5, 24).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::PLUS, 6, 7, 8, 9, 0).unwrap()
        );
    }

    #[test]
    fn should_store_contents_without_changing_sign() {
        let mut computer = setup_computer();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 2000, 0, 13, 24).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::MINUS, 6, 7, 8, 9, 0).unwrap()
        );
    }

    #[test]
    fn should_store_contents_in_index_adjusted_address() {
        let mut computer = setup_computer();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 2000, 0, 19, 24).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::MINUS, 1, 9, 0, 4, 5).unwrap()
        );
    }

    #[test]
    fn should_store_part_of_contents_in_memory() {
        let mut computer = setup_computer();
        computer.r_i4 = Index::from_i32(1999).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 4, 45, 24).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::MINUS, 1, 2, 3, 4, 0).unwrap()
        );
    }
}

mod stx_tests {
    use crate::{computer::*, data_types::*};

    fn setup_computer() -> Computer {
        let mut computer = Computer::new();
        computer
            .memory
            .set(
                2000,
                Word::from_byte_values(Sign::MINUS, 1, 2, 3, 4, 5).unwrap(),
            )
            .unwrap();
        computer.r_x = Word::from_byte_values(Sign::PLUS, 6, 7, 8, 9, 0).unwrap();
        computer
    }

    #[test]
    fn should_store_contents_in_memory() {
        let mut computer = setup_computer();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 2000, 0, 5, 31).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::PLUS, 6, 7, 8, 9, 0).unwrap()
        );
    }

    #[test]
    fn should_store_contents_without_changing_sign() {
        let mut computer = setup_computer();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 2000, 0, 13, 31).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::MINUS, 6, 7, 8, 9, 0).unwrap()
        );
    }

    #[test]
    fn should_store_contents_in_index_adjusted_address() {
        let mut computer = setup_computer();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 2000, 0, 19, 31).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::MINUS, 1, 9, 0, 4, 5).unwrap()
        );
    }

    #[test]
    fn should_store_part_of_contents_in_memory() {
        let mut computer = setup_computer();
        computer.r_i4 = Index::from_i32(1999).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 4, 45, 31).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::MINUS, 1, 2, 3, 4, 0).unwrap()
        );
    }
}

mod st1_tests {
    use crate::{computer::*, data_types::*};

    fn setup_computer() -> Computer {
        let mut computer = Computer::new();
        computer
            .memory
            .set(
                2000,
                Word::from_byte_values(Sign::MINUS, 1, 2, 3, 4, 5).unwrap(),
            )
            .unwrap();
        computer.r_i1 = Index::from_byte_values(Sign::PLUS, 6, 7).unwrap();
        computer
    }

    #[test]
    fn should_store_contents_in_memory() {
        let mut computer = setup_computer();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 2000, 0, 5, 25).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::PLUS, 0, 0, 0, 6, 7).unwrap()
        );
    }

    #[test]
    fn should_store_contents_without_changing_sign() {
        let mut computer = setup_computer();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 2000, 0, 13, 25).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::MINUS, 0, 0, 0, 6, 7).unwrap()
        );
    }

    #[test]
    fn should_store_contents_in_index_adjusted_address() {
        let mut computer = setup_computer();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 2000, 0, 19, 25).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::MINUS, 1, 6, 7, 4, 5).unwrap()
        );
    }

    #[test]
    fn should_store_part_of_contents_in_memory() {
        let mut computer = setup_computer();
        computer.r_i4 = Index::from_i32(1999).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 4, 45, 25).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::MINUS, 1, 2, 3, 4, 7).unwrap()
        );
    }
}
