mod jmp {
    use crate::computer::*;
    use crate::data_types::*;

    const FIELD: i32 = 0;
    const CODE: i32 = 39;

    #[test]
    fn should_set_current_instruction_address_and_j_register() {
        let mut computer = Computer::new();
        computer.current_instruction_address = 10;
        computer.r_i2 = Index::from_i32(30).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 10, 2, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.current_instruction_address, 40);
        assert_eq!(computer.r_j.to_i32(), 10);
    }

    #[test]
    fn should_work_when_called_from_memory() {
        let mut computer = Computer::new();
        computer.current_instruction_address = 10;
        computer
            .memory
            .set(
                10,
                Word::from_instruction_parts(Sign::PLUS, 1000, 0, FIELD, CODE).unwrap(),
            )
            .unwrap();

        computer.handle_next_instruction();

        assert_eq!(computer.current_instruction_address, 1000);
        assert_eq!(computer.r_j.to_i32(), 11);
    }
}
