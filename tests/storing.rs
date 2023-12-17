mod sta {
    use mix_emu::{computer::*, data_types::*};

    fn setup_computer() -> Computer {
        let mut computer = Computer::new();
        computer
            .memory
            .set(
                2000,
                Word::from_byte_values(Sign::MINUS, 1, 2, 3, 4, 5).unwrap(),
            )
            .unwrap();
        computer.registers.a = Word::from_byte_values(Sign::PLUS, 6, 7, 8, 9, 0).unwrap();
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
    fn should_store_part_of_contents() {
        let mut computer = setup_computer();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 2000, 0, 19, 24).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::MINUS, 1, 9, 0, 4, 5).unwrap()
        );
    }

    #[test]
    fn should_store_contents_in_index_adjusted_address() {
        let mut computer = setup_computer();
        computer.registers.i4 = Index::from_i32(1999).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 4, 45, 24).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::MINUS, 1, 2, 3, 4, 0).unwrap()
        );
    }
}

mod stx {
    use mix_emu::{computer::*, data_types::*};

    fn setup_computer() -> Computer {
        let mut computer = Computer::new();
        computer
            .memory
            .set(
                2000,
                Word::from_byte_values(Sign::MINUS, 1, 2, 3, 4, 5).unwrap(),
            )
            .unwrap();
        computer.registers.x = Word::from_byte_values(Sign::PLUS, 6, 7, 8, 9, 0).unwrap();
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
    fn should_store_part_of_contents() {
        let mut computer = setup_computer();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 2000, 0, 19, 31).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::MINUS, 1, 9, 0, 4, 5).unwrap()
        );
    }

    #[test]
    fn should_store_contents_in_index_adjusted_address() {
        let mut computer = setup_computer();
        computer.registers.i4 = Index::from_i32(1999).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 4, 45, 31).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::MINUS, 1, 2, 3, 4, 0).unwrap()
        );
    }
}

mod st1 {
    use mix_emu::{computer::*, data_types::*};

    fn setup_computer() -> Computer {
        let mut computer = Computer::new();
        computer
            .memory
            .set(
                2000,
                Word::from_byte_values(Sign::MINUS, 1, 2, 3, 4, 5).unwrap(),
            )
            .unwrap();
        computer.registers.i1 = Index::from_byte_values(Sign::PLUS, 6, 7).unwrap();
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
    fn should_store_part_of_contents() {
        let mut computer = setup_computer();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 2000, 0, 19, 25).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::MINUS, 1, 6, 7, 4, 5).unwrap()
        );
    }

    #[test]
    fn should_store_contents_in_index_adjusted_address() {
        let mut computer = setup_computer();
        computer.registers.i4 = Index::from_i32(1999).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 4, 45, 25).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::MINUS, 1, 2, 3, 4, 7).unwrap()
        );
    }
}

mod st2 {
    use mix_emu::{computer::*, data_types::*};

    fn setup_computer() -> Computer {
        let mut computer = Computer::new();
        computer
            .memory
            .set(
                2000,
                Word::from_byte_values(Sign::MINUS, 1, 2, 3, 4, 5).unwrap(),
            )
            .unwrap();
        computer.registers.i2 = Index::from_byte_values(Sign::PLUS, 6, 7).unwrap();
        computer
    }

    #[test]
    fn should_store_contents_in_memory() {
        let mut computer = setup_computer();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 2000, 0, 5, 26).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::PLUS, 0, 0, 0, 6, 7).unwrap()
        );
    }

    #[test]
    fn should_store_contents_without_changing_sign() {
        let mut computer = setup_computer();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 2000, 0, 13, 26).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::MINUS, 0, 0, 0, 6, 7).unwrap()
        );
    }

    #[test]
    fn should_store_part_of_contents() {
        let mut computer = setup_computer();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 2000, 0, 19, 26).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::MINUS, 1, 6, 7, 4, 5).unwrap()
        );
    }

    #[test]
    fn should_store_contents_in_index_adjusted_address() {
        let mut computer = setup_computer();
        computer.registers.i4 = Index::from_i32(1999).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 4, 45, 26).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::MINUS, 1, 2, 3, 4, 7).unwrap()
        );
    }
}

mod st3 {
    use mix_emu::{computer::*, data_types::*};

    fn setup_computer() -> Computer {
        let mut computer = Computer::new();
        computer
            .memory
            .set(
                2000,
                Word::from_byte_values(Sign::MINUS, 1, 2, 3, 4, 5).unwrap(),
            )
            .unwrap();
        computer.registers.i3 = Index::from_byte_values(Sign::PLUS, 6, 7).unwrap();
        computer
    }

    #[test]
    fn should_store_contents_in_memory() {
        let mut computer = setup_computer();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 2000, 0, 5, 27).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::PLUS, 0, 0, 0, 6, 7).unwrap()
        );
    }

    #[test]
    fn should_store_contents_without_changing_sign() {
        let mut computer = setup_computer();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 2000, 0, 13, 27).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::MINUS, 0, 0, 0, 6, 7).unwrap()
        );
    }

    #[test]
    fn should_store_part_of_contents() {
        let mut computer = setup_computer();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 2000, 0, 19, 27).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::MINUS, 1, 6, 7, 4, 5).unwrap()
        );
    }

    #[test]
    fn should_store_contents_in_index_adjusted_address() {
        let mut computer = setup_computer();
        computer.registers.i4 = Index::from_i32(1999).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 4, 45, 27).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::MINUS, 1, 2, 3, 4, 7).unwrap()
        );
    }
}

mod st4 {
    use mix_emu::{computer::*, data_types::*};

    fn setup_computer() -> Computer {
        let mut computer = Computer::new();
        computer
            .memory
            .set(
                2000,
                Word::from_byte_values(Sign::MINUS, 1, 2, 3, 4, 5).unwrap(),
            )
            .unwrap();
        computer.registers.i4 = Index::from_byte_values(Sign::PLUS, 6, 7).unwrap();
        computer
    }

    #[test]
    fn should_store_contents_in_memory() {
        let mut computer = setup_computer();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 2000, 0, 5, 28).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::PLUS, 0, 0, 0, 6, 7).unwrap()
        );
    }

    #[test]
    fn should_store_contents_without_changing_sign() {
        let mut computer = setup_computer();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 2000, 0, 13, 28).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::MINUS, 0, 0, 0, 6, 7).unwrap()
        );
    }

    #[test]
    fn should_store_part_of_contents() {
        let mut computer = setup_computer();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 2000, 0, 19, 28).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::MINUS, 1, 6, 7, 4, 5).unwrap()
        );
    }

    #[test]
    fn should_store_contents_in_index_adjusted_address() {
        let mut computer = setup_computer();
        computer.registers.i2 = Index::from_i32(1999).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 2, 45, 28).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::MINUS, 1, 2, 3, 4, 7).unwrap()
        );
    }
}

mod st5 {
    use mix_emu::{computer::*, data_types::*};

    fn setup_computer() -> Computer {
        let mut computer = Computer::new();
        computer
            .memory
            .set(
                2000,
                Word::from_byte_values(Sign::MINUS, 1, 2, 3, 4, 5).unwrap(),
            )
            .unwrap();
        computer.registers.i5 = Index::from_byte_values(Sign::PLUS, 6, 7).unwrap();
        computer
    }

    #[test]
    fn should_store_contents_in_memory() {
        let mut computer = setup_computer();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 2000, 0, 5, 29).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::PLUS, 0, 0, 0, 6, 7).unwrap()
        );
    }

    #[test]
    fn should_store_contents_without_changing_sign() {
        let mut computer = setup_computer();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 2000, 0, 13, 29).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::MINUS, 0, 0, 0, 6, 7).unwrap()
        );
    }

    #[test]
    fn should_store_part_of_contents() {
        let mut computer = setup_computer();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 2000, 0, 19, 29).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::MINUS, 1, 6, 7, 4, 5).unwrap()
        );
    }

    #[test]
    fn should_store_contents_in_index_adjusted_address() {
        let mut computer = setup_computer();
        computer.registers.i4 = Index::from_i32(1999).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 4, 45, 29).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::MINUS, 1, 2, 3, 4, 7).unwrap()
        );
    }
}

mod st6 {
    use mix_emu::{computer::*, data_types::*};

    fn setup_computer() -> Computer {
        let mut computer = Computer::new();
        computer
            .memory
            .set(
                2000,
                Word::from_byte_values(Sign::MINUS, 1, 2, 3, 4, 5).unwrap(),
            )
            .unwrap();
        computer.registers.i6 = Index::from_byte_values(Sign::PLUS, 6, 7).unwrap();
        computer
    }

    #[test]
    fn should_store_contents_in_memory() {
        let mut computer = setup_computer();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 2000, 0, 5, 30).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::PLUS, 0, 0, 0, 6, 7).unwrap()
        );
    }

    #[test]
    fn should_store_contents_without_changing_sign() {
        let mut computer = setup_computer();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 2000, 0, 13, 30).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::MINUS, 0, 0, 0, 6, 7).unwrap()
        );
    }

    #[test]
    fn should_store_part_of_contents() {
        let mut computer = setup_computer();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 2000, 0, 19, 30).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::MINUS, 1, 6, 7, 4, 5).unwrap()
        );
    }

    #[test]
    fn should_store_contents_in_index_adjusted_address() {
        let mut computer = setup_computer();
        computer.registers.i4 = Index::from_i32(1999).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 4, 45, 30).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::MINUS, 1, 2, 3, 4, 7).unwrap()
        );
    }
}

mod stj {
    use mix_emu::{computer::*, data_types::*};

    fn setup_computer() -> Computer {
        let mut computer = Computer::new();
        computer
            .memory
            .set(
                2000,
                Word::from_byte_values(Sign::MINUS, 1, 2, 3, 4, 5).unwrap(),
            )
            .unwrap();
        computer.registers.j = JumpAddress::from_byte_values(6, 7).unwrap();
        computer
    }

    #[test]
    fn should_store_contents_in_memory() {
        let mut computer = setup_computer();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 2000, 0, 5, 32).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::PLUS, 0, 0, 0, 6, 7).unwrap()
        );
    }

    #[test]
    fn should_store_contents_without_changing_sign() {
        let mut computer = setup_computer();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 2000, 0, 13, 32).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::MINUS, 0, 0, 0, 6, 7).unwrap()
        );
    }

    #[test]
    fn should_store_part_of_contents() {
        let mut computer = setup_computer();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 2000, 0, 19, 32).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::MINUS, 1, 6, 7, 4, 5).unwrap()
        );
    }

    #[test]
    fn should_store_contents_in_index_adjusted_address() {
        let mut computer = setup_computer();
        computer.registers.i4 = Index::from_i32(1999).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 4, 45, 32).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::MINUS, 1, 2, 3, 4, 7).unwrap()
        );
    }
}

mod stz {
    use mix_emu::{computer::*, data_types::*};

    fn setup_computer() -> Computer {
        let mut computer = Computer::new();
        computer
            .memory
            .set(
                2000,
                Word::from_byte_values(Sign::MINUS, 1, 2, 3, 4, 5).unwrap(),
            )
            .unwrap();
        computer.registers.a = Word::from_byte_values(Sign::PLUS, 6, 7, 8, 9, 0).unwrap();
        computer
    }

    #[test]
    fn should_store_contents_in_memory() {
        let mut computer = setup_computer();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 2000, 0, 5, 33).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::PLUS, 0, 0, 0, 0, 0).unwrap()
        );
    }

    #[test]
    fn should_store_contents_without_changing_sign() {
        let mut computer = setup_computer();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 2000, 0, 13, 33).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::MINUS, 0, 0, 0, 0, 0).unwrap()
        );
    }

    #[test]
    fn should_store_part_of_contents() {
        let mut computer = setup_computer();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 2000, 0, 19, 33).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::MINUS, 1, 0, 0, 4, 5).unwrap()
        );
    }

    #[test]
    fn should_store_contents_in_index_adjusted_address() {
        let mut computer = setup_computer();
        computer.registers.i4 = Index::from_i32(1999).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 4, 45, 33).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.memory.get(2000).unwrap(),
            Word::from_byte_values(Sign::MINUS, 1, 2, 3, 4, 0).unwrap()
        );
    }
}
