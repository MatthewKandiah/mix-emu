mod add {
    use mix_emu::computer::*;
    use mix_emu::data_types::*;

    fn setup_computer() -> Computer {
        let mut computer = Computer::new();
        computer
            .memory
            .set(
                1000,
                Word::from_byte_values(Sign::PLUS, 1, 2, 3, 4, 5).unwrap(),
            )
            .unwrap();
        computer
            .memory
            .set(
                1001,
                Word::from_byte_values(Sign::MINUS, 1, 2, 3, 4, 5).unwrap(),
            )
            .unwrap();
        computer
            .memory
            .set(2000, Word::from_i32(1234).unwrap())
            .unwrap();
        computer
            .memory
            .set(2001, Word::from_i32(-1234).unwrap())
            .unwrap();
        computer.memory.set(3000, Word::MAX).unwrap();
        computer.memory.set(3001, Word::MIN).unwrap();
        computer.registers.i1 = Index::from_i32(1).unwrap();
        computer.registers.a = Word::from_i32(1).unwrap();
        computer
    }

    #[test]
    fn should_add_value_from_memory_to_r_a() {
        let mut computer = setup_computer();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 2000, 0, 5, 1).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.registers.a.to_i32(), 1235);

        let instruction2 = Word::from_instruction_parts(Sign::PLUS, 2000, 1, 5, 1).unwrap();
        computer.handle_instruction(instruction2);
        assert_eq!(computer.registers.a.to_i32(), 1);
    }

    #[test]
    fn should_set_overflow_if_value_too_large_to_store_in_r_a() {
        let mut computer = setup_computer();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 3000, 0, 5, 1).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.registers.a.to_i32(), 0);
        assert_eq!(computer.overflow, true);
    }

    #[test]
    fn should_set_overflow_if_negative_value_too_large_to_store_in_r_a() {
        let mut computer = setup_computer();
        computer.registers.a.sign = Sign::MINUS;

        let instruction = Word::from_instruction_parts(Sign::PLUS, 3001, 0, 5, 1).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.registers.a.to_i32(), 0);
        assert_eq!(computer.overflow, true);
    }

    #[test]
    fn should_not_change_sign_of_r_a_if_result_is_zero() {
        let mut computer = setup_computer();
        computer.registers.a = Word::from_i32(-1234).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 2000, 0, 5, 1).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(
            computer.registers.a,
            Word::from_byte_values(Sign::MINUS, 0, 0, 0, 0, 0).unwrap()
        );

        computer.registers.a = Word::from_i32(1234).unwrap();

        let instruction2 = Word::from_instruction_parts(Sign::PLUS, 2001, 0, 5, 1).unwrap();
        computer.handle_instruction(instruction2);
        assert_eq!(
            computer.registers.a,
            Word::from_byte_values(Sign::PLUS, 0, 0, 0, 0, 0).unwrap()
        );
    }

    #[test]
    fn should_add_parts_of_word() {
        let mut computer = setup_computer();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 1000, 0, 45, 1).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.registers.a.to_i32(), 6);

        let instruction2 = Word::from_instruction_parts(Sign::PLUS, 1000, 0, 19, 1).unwrap();
        computer.handle_instruction(instruction2);
        assert_eq!(computer.registers.a.to_i32(), 6 + 128 + 3);

        let instruction3 = Word::from_instruction_parts(Sign::PLUS, 1001, 0, 28, 1).unwrap();
        computer.handle_instruction(instruction3);
        assert_eq!(computer.registers.a.to_i32(), 137 - 192 - 4);
    }

    #[test]
    fn should_match_example_from_book() {
        let mut computer = Computer::new();
        computer.registers.a = Word::from_instruction_parts(Sign::PLUS, 1234, 1, 2, 22).unwrap();
        computer
            .memory
            .set(
                1000,
                Word::from_instruction_parts(Sign::PLUS, 100, 5, 0, 50).unwrap(),
            )
            .unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 1000, 0, 5, 1).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.registers.a,
            Word::from_instruction_parts(Sign::PLUS, 1334, 6, 3, 8).unwrap()
        );
    }
}

mod sub {
    use mix_emu::computer::*;
    use mix_emu::data_types::*;

    fn setup_computer() -> Computer {
        let mut computer = Computer::new();
        computer
            .memory
            .set(
                1000,
                Word::from_byte_values(Sign::PLUS, 1, 2, 3, 4, 5).unwrap(),
            )
            .unwrap();
        computer
            .memory
            .set(
                1001,
                Word::from_byte_values(Sign::MINUS, 1, 2, 3, 4, 5).unwrap(),
            )
            .unwrap();
        computer
            .memory
            .set(2000, Word::from_i32(1234).unwrap())
            .unwrap();
        computer
            .memory
            .set(2001, Word::from_i32(-1234).unwrap())
            .unwrap();
        computer.memory.set(3000, Word::MAX).unwrap();
        computer.memory.set(3001, Word::MIN).unwrap();
        computer.registers.i1 = Index::from_i32(1).unwrap();
        computer.registers.a = Word::from_i32(1).unwrap();
        computer
    }

    #[test]
    fn should_sub_value_from_memory_from_r_a() {
        let mut computer = setup_computer();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 2000, 0, 5, 2).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.registers.a.to_i32(), -1233);

        let instruction2 = Word::from_instruction_parts(Sign::PLUS, 2000, 1, 5, 2).unwrap();
        computer.handle_instruction(instruction2);
        assert_eq!(computer.registers.a.to_i32(), 1);
    }

    #[test]
    fn should_set_overflow_if_value_too_large_to_store_in_r_a() {
        let mut computer = setup_computer();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 3001, 0, 5, 2).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.registers.a.to_i32(), 0);
        assert_eq!(computer.overflow, true);
    }

    #[test]
    fn should_set_overflow_if_negative_value_too_large_to_store_in_r_a() {
        let mut computer = setup_computer();
        computer.registers.a.sign = Sign::MINUS;

        let instruction = Word::from_instruction_parts(Sign::PLUS, 3000, 0, 5, 2).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.registers.a.to_i32(), 0);
        assert_eq!(computer.overflow, true);
    }

    #[test]
    fn should_not_change_sign_of_r_a_if_result_is_zero() {
        let mut computer = setup_computer();
        computer.registers.a = Word::from_i32(-1234).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 2001, 0, 5, 2).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(
            computer.registers.a,
            Word::from_byte_values(Sign::MINUS, 0, 0, 0, 0, 0).unwrap()
        );

        computer.registers.a = Word::from_i32(1234).unwrap();

        let instruction2 = Word::from_instruction_parts(Sign::PLUS, 2000, 0, 5, 2).unwrap();
        computer.handle_instruction(instruction2);
        assert_eq!(
            computer.registers.a,
            Word::from_byte_values(Sign::PLUS, 0, 0, 0, 0, 0).unwrap()
        );
    }

    #[test]
    fn should_add_parts_of_word() {
        let mut computer = setup_computer();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 1001, 0, 45, 2).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.registers.a.to_i32(), 6);

        let instruction2 = Word::from_instruction_parts(Sign::PLUS, 1001, 0, 19, 2).unwrap();
        computer.handle_instruction(instruction2);
        assert_eq!(computer.registers.a.to_i32(), 6 + 128 + 3);

        let instruction3 = Word::from_instruction_parts(Sign::PLUS, 1000, 0, 28, 2).unwrap();
        computer.handle_instruction(instruction3);
        assert_eq!(computer.registers.a.to_i32(), 137 - 192 - 4);
    }

    #[test]
    fn should_agree_with_example_from_book() {
        let mut computer = Computer::new();
        computer.registers.a = Word::from_instruction_parts(Sign::MINUS, 1234, 0, 0, 9).unwrap();
        computer
            .memory
            .set(
                1000,
                Word::from_instruction_parts(Sign::MINUS, 2000, 2, 22, 0).unwrap(),
            )
            .unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 1000, 0, 5, 2).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.registers.a,
            Word::from_instruction_parts(Sign::PLUS, 766, 2, 21, 55).unwrap()
        );
    }
}

mod mul {
    use mix_emu::computer::*;
    use mix_emu::data_types::*;

    const MEM_ADDRESS: i32 = 2345;
    fn set_up_computer(a_value: i32, mem_value: i32) -> Computer {
        let mut computer = Computer::new();
        computer.registers.a = Word::from_i32(a_value).unwrap();
        computer
            .memory
            .set(MEM_ADDRESS, Word::from_i32(mem_value).unwrap())
            .unwrap();
        computer
    }

    fn test_instruction() -> Word {
        Word::from_instruction_parts(Sign::PLUS, MEM_ADDRESS, 0, 5, 3).unwrap()
    }

    #[test]
    fn should_handle_positive_values_that_do_not_overflow_r_x() {
        let mut computer = set_up_computer(3, 5);
        computer.handle_instruction(test_instruction());
        assert_eq!(computer.registers.x.to_i32(), 15);
        assert_eq!(computer.registers.a.to_i32(), 0);
        assert_eq!(computer.registers.a.sign, Sign::PLUS);
        assert_eq!(computer.overflow, false);
    }

    #[test]
    fn should_handle_negative_values_that_do_not_overflow_r_x() {
        let mut computer = set_up_computer(-4, 3);
        computer.handle_instruction(test_instruction());
        assert_eq!(computer.registers.x.to_i32(), -12);
        assert_eq!(computer.registers.a.to_i32(), 0);
        assert_eq!(computer.registers.a.sign, Sign::MINUS);
        assert_eq!(computer.overflow, false);
    }

    #[test]
    fn should_handle_positive_values_that_overflow_r_x_but_not_r_a() {
        let mut computer = set_up_computer(64_i32.pow(3), 64_i32.pow(4));
        computer.handle_instruction(test_instruction());
        assert_eq!(computer.registers.x.to_i32(), 0);
        assert_eq!(computer.registers.x.sign, Sign::PLUS);
        assert_eq!(computer.registers.a.to_i32(), 64_i32.pow(2));
        assert_eq!(computer.overflow, false);
    }

    #[test]
    fn should_handle_negative_values_that_overflow_r_x_but_not_r_a() {
        let mut computer = set_up_computer(-64_i32.pow(3), 64_i32.pow(4));
        computer.handle_instruction(test_instruction());
        assert_eq!(computer.registers.x.to_i32(), 0);
        assert_eq!(computer.registers.x.sign, Sign::MINUS);
        assert_eq!(computer.registers.a.to_i32(), -64_i32.pow(2));
        assert_eq!(computer.overflow, false);
    }

    #[test]
    fn should_handle_multiplication_by_positive_zero_of_positive() {
        let mut computer = set_up_computer(0, 1234);
        computer.registers.a.sign = Sign::PLUS;

        computer.handle_instruction(test_instruction());

        assert_eq!(computer.registers.a, Word::ZERO.with_sign(Sign::PLUS));
        assert_eq!(computer.registers.x, Word::ZERO.with_sign(Sign::PLUS));
    }

    #[test]
    fn should_handle_multiplication_by_positive_zero_of_negative() {
        let mut computer = set_up_computer(0, -1234);
        computer.registers.a.sign = Sign::PLUS;

        computer.handle_instruction(test_instruction());

        assert_eq!(computer.registers.a, Word::ZERO.with_sign(Sign::MINUS));
        assert_eq!(computer.registers.x, Word::ZERO.with_sign(Sign::MINUS));
    }

    #[test]
    fn should_handle_multiplication_by_negative_zero_of_positive() {
        let mut computer = set_up_computer(0, 1234);
        computer.registers.a.sign = Sign::MINUS;

        computer.handle_instruction(test_instruction());

        assert_eq!(computer.registers.a, Word::ZERO.with_sign(Sign::MINUS));
        assert_eq!(computer.registers.x, Word::ZERO.with_sign(Sign::MINUS));
    }

    #[test]
    fn should_handle_multiplication_by_negative_zero_of_negative() {
        let mut computer = set_up_computer(0, -1234);
        computer.registers.a.sign = Sign::MINUS;

        computer.handle_instruction(test_instruction());

        assert_eq!(computer.registers.a, Word::ZERO.with_sign(Sign::PLUS));
        assert_eq!(computer.registers.x, Word::ZERO.with_sign(Sign::PLUS));
    }

    #[test]
    fn should_agree_with_example_from_book() {
        let mut computer = Computer::new();
        computer.registers.a = Word::from_byte_values(Sign::PLUS, 1, 1, 1, 1, 1).unwrap();
        computer
            .memory
            .set(
                1000,
                Word::from_byte_values(Sign::PLUS, 1, 1, 1, 1, 1).unwrap(),
            )
            .unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 1000, 0, 5, 3).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.registers.a,
            Word::from_byte_values(Sign::PLUS, 0, 1, 2, 3, 4).unwrap()
        );
        assert_eq!(
            computer.registers.x,
            Word::from_byte_values(Sign::PLUS, 5, 4, 3, 2, 1).unwrap()
        );
    }
}

mod div {
    use mix_emu::computer::*;
    use mix_emu::data_types::*;

    #[test]
    fn should_agree_with_first_book_example() {
        let mut computer = Computer::new();
        computer.registers.a = Word::ZERO.with_sign(Sign::PLUS);
        computer.registers.x = Word::from_i32(-17).unwrap();
        computer
            .memory
            .set(1000, Word::from_i32(3).unwrap())
            .unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 1000, 0, 5, 4).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.registers.a, Word::from_i32(5).unwrap());
        assert_eq!(computer.registers.x, Word::from_i32(2).unwrap());
    }

    #[test]
    fn should_agree_with_second_book_example() {
        let mut computer = Computer::new();
        computer.registers.a = Word::from_byte_values(Sign::MINUS, 0, 0, 0, 0, 0).unwrap();
        computer.registers.x = Word::from_instruction_parts(Sign::PLUS, 1235, 0, 3, 1).unwrap();
        computer
            .memory
            .set(
                1000,
                Word::from_byte_values(Sign::MINUS, 0, 0, 0, 2, 0).unwrap(),
            )
            .unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 1000, 0, 5, 4).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.registers.a,
            Word::from_byte_values(Sign::PLUS, 0, 617 / 64, 617 % 64, 2049 / 64, 2049 % 64)
                .unwrap()
        );
        assert_eq!(
            computer.registers.x,
            Word::from_byte_values(Sign::MINUS, 0, 0, 0, 1, 1).unwrap()
        );
    }

    #[test]
    fn should_overflow_and_not_panic_on_divide_by_zero() {
        let mut computer = Computer::new();
        computer.memory.set(1, Word::ZERO).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 0, 5, 4).unwrap();
        computer.handle_instruction(instruction);

        assert!(computer.overflow);
    }

    #[test]
    fn should_overflow_and_not_panic_on_result_overflow() {
        let mut computer = Computer::new();
        computer.registers.a = Word::MIN;
        computer.registers.x = Word::MAX;
        computer.memory.set(1, Word::from_i32(1).unwrap()).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 0, 5, 4).unwrap();
        computer.handle_instruction(instruction);

        assert!(computer.overflow);
    }
}
