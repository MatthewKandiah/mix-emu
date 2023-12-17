use mix_emu::computer::*;
use mix_emu::data_types::*;

fn setup_computer() -> Computer {
    let mut computer = Computer::new();
    computer.registers.a = Word::from_byte_values(Sign::PLUS, 1, 2, 3, 4, 5).unwrap();
    computer.registers.x = Word::from_byte_values(Sign::MINUS, 6, 7, 8, 9, 10).unwrap();
    computer
}

#[test]
fn book_example() {
    let mut computer = setup_computer();
    let srax1 = Word::from_instruction_parts(Sign::PLUS, 1, 0, 3, 6).unwrap();
    let sla2 = Word::from_instruction_parts(Sign::PLUS, 2, 0, 0, 6).unwrap();
    let src4 = Word::from_instruction_parts(Sign::PLUS, 4, 0, 5, 6).unwrap();
    let sra2 = Word::from_instruction_parts(Sign::PLUS, 2, 0, 1, 6).unwrap();
    let slc501 = Word::from_instruction_parts(Sign::PLUS, 501, 0, 4, 6).unwrap();

    computer.handle_instruction(srax1);
    assert_eq!(
        computer.registers.a,
        Word::from_byte_values(Sign::PLUS, 0, 1, 2, 3, 4).unwrap()
    );
    assert_eq!(
        computer.registers.x,
        Word::from_byte_values(Sign::MINUS, 5, 6, 7, 8, 9).unwrap()
    );

    computer.handle_instruction(sla2);
    assert_eq!(
        computer.registers.a,
        Word::from_byte_values(Sign::PLUS, 2, 3, 4, 0, 0).unwrap()
    );
    assert_eq!(
        computer.registers.x,
        Word::from_byte_values(Sign::MINUS, 5, 6, 7, 8, 9).unwrap()
    );

    computer.handle_instruction(src4);
    assert_eq!(
        computer.registers.a,
        Word::from_byte_values(Sign::PLUS, 6, 7, 8, 9, 2).unwrap()
    );
    assert_eq!(
        computer.registers.x,
        Word::from_byte_values(Sign::MINUS, 3, 4, 0, 0, 5).unwrap()
    );

    computer.handle_instruction(sra2);
    assert_eq!(
        computer.registers.a,
        Word::from_byte_values(Sign::PLUS, 0, 0, 6, 7, 8).unwrap()
    );
    assert_eq!(
        computer.registers.x,
        Word::from_byte_values(Sign::MINUS, 3, 4, 0, 0, 5).unwrap()
    );

    computer.handle_instruction(slc501);
    assert_eq!(
        computer.registers.a,
        Word::from_byte_values(Sign::PLUS, 0, 6, 7, 8, 3).unwrap()
    );
    assert_eq!(
        computer.registers.x,
        Word::from_byte_values(Sign::MINUS, 4, 0, 0, 5, 0).unwrap()
    );
}

mod sla {
    use mix_emu::data_types::*;

    use crate::setup_computer;

    const CODE: i32 = 6;
    const FIELD: i32 = 0;

    #[test]
    fn should_shift_one_position() {
        let mut computer = setup_computer();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.registers.a,
            Word::from_byte_values(Sign::PLUS, 2, 3, 4, 5, 0).unwrap()
        );
        assert_eq!(
            computer.registers.x,
            Word::from_byte_values(Sign::MINUS, 6, 7, 8, 9, 10).unwrap()
        );
    }

    #[test]
    fn should_shift_four_positions() {
        let mut computer = setup_computer();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 4, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.registers.a,
            Word::from_byte_values(Sign::PLUS, 5, 0, 0, 0, 0).unwrap()
        );
        assert_eq!(
            computer.registers.x,
            Word::from_byte_values(Sign::MINUS, 6, 7, 8, 9, 10).unwrap()
        );
    }

    #[test]
    fn should_shift_eight_positions() {
        let mut computer = setup_computer();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 8, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.registers.a,
            Word::from_byte_values(Sign::PLUS, 0, 0, 0, 0, 0).unwrap()
        );
        assert_eq!(
            computer.registers.x,
            Word::from_byte_values(Sign::MINUS, 6, 7, 8, 9, 10).unwrap()
        );
    }
}

mod sra {
    use crate::setup_computer;
    use mix_emu::data_types::*;

    const CODE: i32 = 6;
    const FIELD: i32 = 1;

    #[test]
    fn should_shift_one_position() {
        let mut computer = setup_computer();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.registers.a,
            Word::from_byte_values(Sign::PLUS, 0, 1, 2, 3, 4).unwrap()
        );
        assert_eq!(
            computer.registers.x,
            Word::from_byte_values(Sign::MINUS, 6, 7, 8, 9, 10).unwrap()
        );
    }

    #[test]
    fn should_shift_four_positions() {
        let mut computer = setup_computer();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 4, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.registers.a,
            Word::from_byte_values(Sign::PLUS, 0, 0, 0, 0, 1).unwrap()
        );
        assert_eq!(
            computer.registers.x,
            Word::from_byte_values(Sign::MINUS, 6, 7, 8, 9, 10).unwrap()
        );
    }

    #[test]
    fn should_shift_eight_positions() {
        let mut computer = setup_computer();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 8, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.registers.a,
            Word::from_byte_values(Sign::PLUS, 0, 0, 0, 0, 0).unwrap()
        );
        assert_eq!(
            computer.registers.x,
            Word::from_byte_values(Sign::MINUS, 6, 7, 8, 9, 10).unwrap()
        );
    }
}

mod slax {
    use crate::setup_computer;
    use mix_emu::data_types::*;

    const CODE: i32 = 6;
    const FIELD: i32 = 2;

    #[test]
    fn should_shift_one_position() {
        let mut computer = setup_computer();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.registers.a,
            Word::from_byte_values(Sign::PLUS, 2, 3, 4, 5, 6).unwrap()
        );
        assert_eq!(
            computer.registers.x,
            Word::from_byte_values(Sign::MINUS, 7, 8, 9, 10, 0).unwrap()
        );
    }

    #[test]
    fn should_shift_four_positions() {
        let mut computer = setup_computer();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 4, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.registers.a,
            Word::from_byte_values(Sign::PLUS, 5, 6, 7, 8, 9).unwrap()
        );
        assert_eq!(
            computer.registers.x,
            Word::from_byte_values(Sign::MINUS, 10, 0, 0, 0, 0).unwrap()
        );
    }

    #[test]
    fn should_shift_eight_positions() {
        let mut computer = setup_computer();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 8, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.registers.a,
            Word::from_byte_values(Sign::PLUS, 9, 10, 0, 0, 0).unwrap()
        );
        assert_eq!(
            computer.registers.x,
            Word::from_byte_values(Sign::MINUS, 0, 0, 0, 0, 0).unwrap()
        );
    }
}

mod srax {
    use crate::setup_computer;
    use mix_emu::data_types::*;

    const CODE: i32 = 6;
    const FIELD: i32 = 3;

    #[test]
    fn should_shift_one_position() {
        let mut computer = setup_computer();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.registers.a,
            Word::from_byte_values(Sign::PLUS, 0, 1, 2, 3, 4).unwrap()
        );
        assert_eq!(
            computer.registers.x,
            Word::from_byte_values(Sign::MINUS, 5, 6, 7, 8, 9).unwrap()
        );
    }

    #[test]
    fn should_shift_four_positions() {
        let mut computer = setup_computer();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 4, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.registers.a,
            Word::from_byte_values(Sign::PLUS, 0, 0, 0, 0, 1).unwrap()
        );
        assert_eq!(
            computer.registers.x,
            Word::from_byte_values(Sign::MINUS, 2, 3, 4, 5, 6).unwrap()
        );
    }

    #[test]
    fn should_shift_eight_positions() {
        let mut computer = setup_computer();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 8, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.registers.a,
            Word::from_byte_values(Sign::PLUS, 0, 0, 0, 0, 0).unwrap()
        );
        assert_eq!(
            computer.registers.x,
            Word::from_byte_values(Sign::MINUS, 0, 0, 0, 1, 2).unwrap()
        );
    }
}

mod slc {
    use crate::setup_computer;
    use mix_emu::data_types::*;

    const CODE: i32 = 6;
    const FIELD: i32 = 4;

    #[test]
    fn should_shift_one_position() {
        let mut computer = setup_computer();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.registers.a,
            Word::from_byte_values(Sign::PLUS, 2, 3, 4, 5, 6).unwrap()
        );
        assert_eq!(
            computer.registers.x,
            Word::from_byte_values(Sign::MINUS, 7, 8, 9, 10, 1).unwrap()
        );
    }

    #[test]
    fn should_shift_four_positions() {
        let mut computer = setup_computer();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 4, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.registers.a,
            Word::from_byte_values(Sign::PLUS, 5, 6, 7, 8, 9).unwrap()
        );
        assert_eq!(
            computer.registers.x,
            Word::from_byte_values(Sign::MINUS, 10, 1, 2, 3, 4).unwrap()
        );
    }

    #[test]
    fn should_shift_eight_positions() {
        let mut computer = setup_computer();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 8, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.registers.a,
            Word::from_byte_values(Sign::PLUS, 9, 10, 1, 2, 3).unwrap()
        );
        assert_eq!(
            computer.registers.x,
            Word::from_byte_values(Sign::MINUS, 4, 5, 6, 7, 8).unwrap()
        );
    }
}

mod src {
    use crate::setup_computer;
    use mix_emu::data_types::*;

    const CODE: i32 = 6;
    const FIELD: i32 = 5;

    #[test]
    fn should_shift_one_position() {
        let mut computer = setup_computer();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.registers.a,
            Word::from_byte_values(Sign::PLUS, 10, 1, 2, 3, 4).unwrap()
        );
        assert_eq!(
            computer.registers.x,
            Word::from_byte_values(Sign::MINUS, 5, 6, 7, 8, 9).unwrap()
        );
    }

    #[test]
    fn should_shift_four_positions() {
        let mut computer = setup_computer();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 4, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.registers.a,
            Word::from_byte_values(Sign::PLUS, 7, 8, 9, 10, 1).unwrap()
        );
        assert_eq!(
            computer.registers.x,
            Word::from_byte_values(Sign::MINUS, 2, 3, 4, 5, 6).unwrap()
        );
    }

    #[test]
    fn should_shift_eight_positions() {
        let mut computer = setup_computer();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 8, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(
            computer.registers.a,
            Word::from_byte_values(Sign::PLUS, 3, 4, 5, 6, 7).unwrap()
        );
        assert_eq!(
            computer.registers.x,
            Word::from_byte_values(Sign::MINUS, 8, 9, 10, 1, 2).unwrap()
        );
    }
}
