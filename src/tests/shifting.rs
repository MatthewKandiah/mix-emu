mod sla {
    use crate::computer::*;
    use crate::data_types::*;

    const CODE: i32 = 6;
    const FIELD: i32 = 0;

    fn setup_computer() -> Computer {
        let mut computer = Computer::new();
        computer.r_a = Word::from_byte_values(Sign::PLUS, 1, 2, 3, 4, 5).unwrap();
        computer.r_x = Word::from_byte_values(Sign::MINUS, 6, 7, 8, 9, 10).unwrap();
        computer
    }

    #[test]
    fn should_shift_one_position() {
        let mut computer = setup_computer();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.r_a,
            Word::from_byte_values(Sign::PLUS, 2, 3, 4, 5, 0).unwrap()
        );
        assert_eq!(
            computer.r_x,
            Word::from_byte_values(Sign::MINUS, 6, 7, 8, 9, 10).unwrap()
        );
    }

    #[test]
    fn should_shift_four_positions() {
        let mut computer = setup_computer();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 4, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.r_a,
            Word::from_byte_values(Sign::PLUS, 5, 0, 0, 0, 0).unwrap()
        );
        assert_eq!(
            computer.r_x,
            Word::from_byte_values(Sign::MINUS, 6, 7, 8, 9, 10).unwrap()
        );
    }

    #[test]
    fn should_shift_eight_positions() {
        let mut computer = setup_computer();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 8, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.r_a,
            Word::from_byte_values(Sign::PLUS, 0, 0, 0, 0, 0).unwrap()
        );
        assert_eq!(
            computer.r_x,
            Word::from_byte_values(Sign::MINUS, 6, 7, 8, 9, 10).unwrap()
        );
    }
}
