mod add{
    use crate::computer::*;
    use crate::data_types::*;

    fn setup_computer() -> Computer {
        let mut computer = Computer::new();
        computer.memory.set(1000, Word::from_byte_values(Sign::PLUS, 1, 2, 3, 4, 5).unwrap()).unwrap();
        computer.memory.set(1001, Word::from_byte_values(Sign::MINUS, 1, 2, 3, 4, 5).unwrap()).unwrap();
        computer.memory.set(2000, Word::from_i32(1234).unwrap()).unwrap();
        computer.memory.set(2001, Word::from_i32(-1234).unwrap()).unwrap();
        computer.memory.set(3000, Word::MAX).unwrap();
        computer.memory.set(3001, Word::MIN).unwrap();
        computer.r_i1 = Index::from_i32(1).unwrap();
        computer.r_a = Word::from_i32(1).unwrap();
        computer
    }

    #[test]
    fn should_add_value_from_memory_to_r_a() {
        let mut computer = setup_computer();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 2000, 0, 5, 1).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_a.to_i32(), 1235);

        let instruction2 = Word::from_instruction_parts(Sign::PLUS, 2000, 1, 5, 1).unwrap();
        computer.handle_instruction(instruction2);
        assert_eq!(computer.r_a.to_i32(), 1);
    }

    #[test]
    fn should_set_overflow_if_value_too_large_to_store_in_r_a() {
        let mut computer = setup_computer();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 3000, 0, 5, 1).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_a.to_i32(), 0);
        assert_eq!(computer.overflow, true);
    }

    #[test]
    fn should_set_overflow_if_negative_value_too_large_to_store_in_r_a() {
        let mut computer = setup_computer();
        computer.r_a.sign = Sign::MINUS;

        let instruction = Word::from_instruction_parts(Sign::PLUS, 3001, 0, 5, 1).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_a.to_i32(), 0);
        assert_eq!(computer.overflow, true);
    }

    #[test]
    fn should_not_change_sign_of_r_a_if_result_is_zero() {
        let mut computer = setup_computer();
        computer.r_a = Word::from_i32(-1234).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 2000, 0, 5, 1).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_a, Word::from_byte_values(Sign::MINUS, 0, 0, 0, 0, 0).unwrap());

        computer.r_a = Word::from_i32(1234).unwrap();

        let instruction2 = Word::from_instruction_parts(Sign::PLUS, 2001, 0, 5, 1).unwrap();
        computer.handle_instruction(instruction2);
        assert_eq!(computer.r_a, Word::from_byte_values(Sign::PLUS, 0, 0, 0, 0, 0).unwrap());
    }

    #[test]
    fn should_add_parts_of_word() {
        let mut computer = setup_computer();
        
        let instruction = Word::from_instruction_parts(Sign::PLUS, 1000, 0, 45, 1).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_a.to_i32(), 6);

        let instruction2 = Word::from_instruction_parts(Sign::PLUS, 1000, 0, 19, 1).unwrap();
        computer.handle_instruction(instruction2);
        assert_eq!(computer.r_a.to_i32(), 6 + 128 + 3);

        let instruction3 = Word::from_instruction_parts(Sign::PLUS, 1001, 0, 28, 1).unwrap();
        computer.handle_instruction(instruction3);
        assert_eq!(computer.r_a.to_i32(), 137 - 192 - 4);
    }
}
