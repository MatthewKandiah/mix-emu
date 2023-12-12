mod jmp {
    use crate::computer::*;
    use crate::data_types::*;

    const FIELD: i32 = 0;
    const CODE: i32 = 39;

    #[test]
    fn should_set_current_instruction_address_and_j_register() {
        let mut computer = Computer::new();
        computer.current_instruction_address = 10;
        computer.registers.i2 = Index::from_i32(30).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 10, 2, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.current_instruction_address, 40);
        assert_eq!(computer.registers.j.to_i32(), 10);
    }

    #[test]
    fn should_work_when_called_from_memory() {
        let mut computer = Computer::new();
        computer.running = true;
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
        assert_eq!(computer.registers.j.to_i32(), 11);
    }
}

mod jsj {
    use crate::computer::*;
    use crate::data_types::*;

    const FIELD: i32 = 1;
    const CODE: i32 = 39;

    #[test]
    fn should_set_current_instruction_address_but_not_j_register() {
        let mut computer = Computer::new();
        computer.current_instruction_address = 10;
        computer.registers.i2 = Index::from_i32(30).unwrap();
        computer.registers.j = JumpAddress::from_i32(4).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 10, 2, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.current_instruction_address, 40);
        assert_eq!(computer.registers.j.to_i32(), 4);
    }

    #[test]
    fn should_work_when_called_from_memory() {
        let mut computer = Computer::new();
        computer.running = true;
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
        assert_eq!(computer.registers.j.to_i32(), 0);
    }
}

mod jov {
    use crate::computer::*;
    use crate::data_types::*;

    const FIELD: i32 = 2;
    const CODE: i32 = 39;

    #[test]
    fn should_jump_if_overflow_on() {
        let mut computer = Computer::new();
        computer.overflow = true;
        computer.current_instruction_address = 12;

        let instruction = Word::from_instruction_parts(Sign::PLUS, 100, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.current_instruction_address, 100);
        assert_eq!(computer.overflow, false);
    }

    #[test]
    fn should_not_jump_if_overflow_off() {
        let mut computer = Computer::new();
        computer.overflow = false;
        computer.current_instruction_address = 12;

        let instruction = Word::from_instruction_parts(Sign::PLUS, 100, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.current_instruction_address, 12);
        assert_eq!(computer.overflow, false);
    }
}

mod jnov {
    use crate::computer::*;
    use crate::data_types::*;

    const FIELD: i32 = 3;
    const CODE: i32 = 39;

    #[test]
    fn should_jump_if_overflow_off() {
        let mut computer = Computer::new();
        computer.overflow = false;
        computer.current_instruction_address = 12;

        let instruction = Word::from_instruction_parts(Sign::PLUS, 100, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.current_instruction_address, 100);
        assert_eq!(computer.overflow, false);
    }

    #[test]
    fn should_not_jump_if_overflow_on() {
        let mut computer = Computer::new();
        computer.overflow = true;
        computer.current_instruction_address = 12;

        let instruction = Word::from_instruction_parts(Sign::PLUS, 100, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.current_instruction_address, 12);
        assert_eq!(computer.overflow, false);
    }
}
