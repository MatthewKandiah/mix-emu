mod cmpa {
    use crate::computer::*;
    use crate::data_types::*;

    const CODE: i32 = 56;
    const ADDRESS: i32 = 1000;

    fn setup_computer(register_value: i32, memory_value: i32) -> Computer {
        let mut computer = Computer::new();
        computer.r_a = Word::from_i32(register_value).unwrap();
        computer
            .memory
            .set(ADDRESS, Word::from_i32(memory_value).unwrap())
            .unwrap();
        computer
    }

    #[test]
    fn should_be_greater() {
        let mut computer = setup_computer(100, 10);

        let instruction = Word::from_instruction_parts(Sign::PLUS, ADDRESS, 0, 5, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.comparison_indicator,
            Some(ComparisonIndicatorState::GREATER)
        );
    }

    #[test]
    fn should_be_less() {
        let mut computer = setup_computer(-100, 10);

        let instruction = Word::from_instruction_parts(Sign::PLUS, ADDRESS, 0, 5, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.comparison_indicator,
            Some(ComparisonIndicatorState::LESS)
        );
    }

    #[test]
    fn should_be_equal() {
        let mut computer = setup_computer(10, 10);

        let instruction = Word::from_instruction_parts(Sign::PLUS, ADDRESS, 0, 5, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.comparison_indicator,
            Some(ComparisonIndicatorState::EQUAL),
        );
    }

    #[test]
    fn should_be_greater_partial_comparison() {
        let mut computer = Computer::new();
        computer.r_a = Word::from_byte_values(Sign::PLUS, 63, 63, 0, 0, 0).unwrap();
        computer
            .memory
            .set(
                ADDRESS,
                Word::from_byte_values(Sign::PLUS, 10, 10, 10, 10, 10).unwrap(),
            )
            .unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, ADDRESS, 0, 10, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.comparison_indicator,
            Some(ComparisonIndicatorState::GREATER),
        );
    }

    #[test]
    fn should_be_less_partial_comparison() {
        let mut computer = Computer::new();
        computer.r_a = Word::from_byte_values(Sign::PLUS, 0, 0, 63, 63, 63).unwrap();
        computer
            .memory
            .set(
                ADDRESS,
                Word::from_byte_values(Sign::PLUS, 10, 10, 10, 10, 10).unwrap(),
            )
            .unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, ADDRESS, 0, 10, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.comparison_indicator,
            Some(ComparisonIndicatorState::LESS),
        );
    }

    #[test]
    fn should_be_equal_partial_comparison() {
        let mut computer = Computer::new();
        computer.r_a = Word::from_byte_values(Sign::PLUS, 10, 10, 63, 63, 63).unwrap();
        computer
            .memory
            .set(
                ADDRESS,
                Word::from_byte_values(Sign::PLUS, 10, 10, 4, 4, 4).unwrap(),
            )
            .unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, ADDRESS, 0, 10, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.comparison_indicator,
            Some(ComparisonIndicatorState::EQUAL),
        );
    }

    #[test]
    fn minus_zero_should_equal_zero() {
        let mut computer = Computer::new();
        computer.r_a = Word::ZERO.with_sign(Sign::PLUS);
        computer
            .memory
            .set(ADDRESS, Word::ZERO.with_sign(Sign::MINUS))
            .unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, ADDRESS, 0, 5, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.comparison_indicator,
            Some(ComparisonIndicatorState::EQUAL)
        );
    }
}

mod cmpx{
    use crate::computer::*;
    use crate::data_types::*;

    const CODE: i32 = 63;
    const ADDRESS: i32 = 1000;

    fn setup_computer(register_value: i32, memory_value: i32) -> Computer {
        let mut computer = Computer::new();
        computer.r_x = Word::from_i32(register_value).unwrap();
        computer
            .memory
            .set(ADDRESS, Word::from_i32(memory_value).unwrap())
            .unwrap();
        computer
    }

    #[test]
    fn should_be_greater() {
        let mut computer = setup_computer(100, 10);

        let instruction = Word::from_instruction_parts(Sign::PLUS, ADDRESS, 0, 5, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.comparison_indicator,
            Some(ComparisonIndicatorState::GREATER)
        );
    }

    #[test]
    fn should_be_less() {
        let mut computer = setup_computer(-100, 10);

        let instruction = Word::from_instruction_parts(Sign::PLUS, ADDRESS, 0, 5, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.comparison_indicator,
            Some(ComparisonIndicatorState::LESS)
        );
    }

    #[test]
    fn should_be_equal() {
        let mut computer = setup_computer(10, 10);

        let instruction = Word::from_instruction_parts(Sign::PLUS, ADDRESS, 0, 5, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.comparison_indicator,
            Some(ComparisonIndicatorState::EQUAL),
        );
    }

    #[test]
    fn should_be_greater_partial_comparison() {
        let mut computer = Computer::new();
        computer.r_x = Word::from_byte_values(Sign::PLUS, 63, 63, 0, 0, 0).unwrap();
        computer
            .memory
            .set(
                ADDRESS,
                Word::from_byte_values(Sign::PLUS, 10, 10, 10, 10, 10).unwrap(),
            )
            .unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, ADDRESS, 0, 10, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.comparison_indicator,
            Some(ComparisonIndicatorState::GREATER),
        );
    }

    #[test]
    fn should_be_less_partial_comparison() {
        let mut computer = Computer::new();
        computer.r_x = Word::from_byte_values(Sign::PLUS, 0, 0, 63, 63, 63).unwrap();
        computer
            .memory
            .set(
                ADDRESS,
                Word::from_byte_values(Sign::PLUS, 10, 10, 10, 10, 10).unwrap(),
            )
            .unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, ADDRESS, 0, 10, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.comparison_indicator,
            Some(ComparisonIndicatorState::LESS),
        );
    }

    #[test]
    fn should_be_equal_partial_comparison() {
        let mut computer = Computer::new();
        computer.r_x = Word::from_byte_values(Sign::PLUS, 10, 10, 63, 63, 63).unwrap();
        computer
            .memory
            .set(
                ADDRESS,
                Word::from_byte_values(Sign::PLUS, 10, 10, 4, 4, 4).unwrap(),
            )
            .unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, ADDRESS, 0, 10, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.comparison_indicator,
            Some(ComparisonIndicatorState::EQUAL),
        );
    }

    #[test]
    fn minus_zero_should_equal_zero() {
        let mut computer = Computer::new();
        computer.r_x = Word::ZERO.with_sign(Sign::PLUS);
        computer
            .memory
            .set(ADDRESS, Word::ZERO.with_sign(Sign::MINUS))
            .unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, ADDRESS, 0, 5, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.comparison_indicator,
            Some(ComparisonIndicatorState::EQUAL)
        );
    }
}
