mod enta {
    use crate::computer::*;
    use crate::data_types::*;

    const CODE: i32 = 48;
    const FIELD: i32 = 2;

    #[test]
    fn should_store_value() {
        let mut computer = Computer::new();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 123, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_a.to_i32(), 123);

        let instruction2 = Word::from_instruction_parts(Sign::MINUS, 234, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction2);
        assert_eq!(computer.r_a.to_i32(), -234);
    }

    #[test]
    fn should_store_zero_with_correct_sign() {
        let mut computer = Computer::new();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 0, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_a, Word::ZERO.with_sign(Sign::PLUS));

        let instruction = Word::from_instruction_parts(Sign::MINUS, 0, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_a, Word::ZERO.with_sign(Sign::MINUS));
    }

    #[test]
    fn should_store_index_modified_value() {
        let mut computer = Computer::new();
        computer.r_i1 = Index::from_i32(-10).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 30, 1, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_a.to_i32(), 20);
    }

    #[test]
    fn should_handle_storing_index_zero_edge_cases() {
        let mut computer = Computer::new();
        computer.r_i1 = Index::from_byte_values(Sign::PLUS, 0, 0).unwrap();
        computer.r_i2 = Index::from_byte_values(Sign::MINUS, 0, 0).unwrap();

        computer.handle_instruction(
            Word::from_instruction_parts(Sign::PLUS, 0, 1, FIELD, CODE).unwrap(),
        );
        assert_eq!(computer.r_a, Word::ZERO.with_sign(Sign::PLUS));

        computer.handle_instruction(
            Word::from_instruction_parts(Sign::MINUS, 0, 1, FIELD, CODE).unwrap(),
        );
        assert_eq!(computer.r_a, Word::ZERO.with_sign(Sign::MINUS));

        computer.handle_instruction(
            Word::from_instruction_parts(Sign::PLUS, 0, FIELD, 2, CODE).unwrap(),
        );
        assert_eq!(computer.r_a, Word::ZERO.with_sign(Sign::PLUS));

        computer.handle_instruction(
            Word::from_instruction_parts(Sign::MINUS, 0, FIELD, 2, CODE).unwrap(),
        );
        assert_eq!(computer.r_a, Word::ZERO.with_sign(Sign::MINUS));
    }
}

mod entx {
    use crate::computer::*;
    use crate::data_types::*;

    const CODE: i32 = 55;
    const FIELD: i32 = 2;

    #[test]
    fn should_store_value() {
        let mut computer = Computer::new();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 123, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_x.to_i32(), 123);

        let instruction2 = Word::from_instruction_parts(Sign::MINUS, 234, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction2);
        assert_eq!(computer.r_x.to_i32(), -234);
    }

    #[test]
    fn should_store_zero_with_correct_sign() {
        let mut computer = Computer::new();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 0, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_x, Word::ZERO.with_sign(Sign::PLUS));

        let instruction = Word::from_instruction_parts(Sign::MINUS, 0, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_x, Word::ZERO.with_sign(Sign::MINUS));
    }

    #[test]
    fn should_store_index_modified_value() {
        let mut computer = Computer::new();
        computer.r_i1 = Index::from_i32(-10).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 30, 1, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_x.to_i32(), 20);
    }

    #[test]
    fn should_handle_storing_index_zero_edge_cases() {
        let mut computer = Computer::new();
        computer.r_i1 = Index::from_byte_values(Sign::PLUS, 0, 0).unwrap();
        computer.r_i2 = Index::from_byte_values(Sign::MINUS, 0, 0).unwrap();

        computer.handle_instruction(
            Word::from_instruction_parts(Sign::PLUS, 0, 1, FIELD, CODE).unwrap(),
        );
        assert_eq!(computer.r_x, Word::ZERO.with_sign(Sign::PLUS));

        computer.handle_instruction(
            Word::from_instruction_parts(Sign::MINUS, 0, 1, FIELD, CODE).unwrap(),
        );
        assert_eq!(computer.r_x, Word::ZERO.with_sign(Sign::MINUS));

        computer.handle_instruction(
            Word::from_instruction_parts(Sign::PLUS, 0, FIELD, 2, CODE).unwrap(),
        );
        assert_eq!(computer.r_x, Word::ZERO.with_sign(Sign::PLUS));

        computer.handle_instruction(
            Word::from_instruction_parts(Sign::MINUS, 0, FIELD, 2, CODE).unwrap(),
        );
        assert_eq!(computer.r_x, Word::ZERO.with_sign(Sign::MINUS));
    }
}

mod ent1 {
    use crate::computer::*;
    use crate::data_types::*;

    const CODE: i32 = 49;
    const FIELD: i32 = 2;

    #[test]
    fn should_store_value() {
        let mut computer = Computer::new();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 123, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i1.to_i32(), 123);

        let instruction2 = Word::from_instruction_parts(Sign::MINUS, 234, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction2);
        assert_eq!(computer.r_i1.to_i32(), -234);
    }

    #[test]
    fn should_store_zero_with_correct_sign() {
        let mut computer = Computer::new();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 0, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i1, Index::ZERO.with_sign(Sign::PLUS));

        let instruction = Word::from_instruction_parts(Sign::MINUS, 0, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i1, Index::ZERO.with_sign(Sign::MINUS));
    }

    #[test]
    fn should_store_index_modified_value() {
        let mut computer = Computer::new();
        computer.r_i6 = Index::from_i32(-10).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 30, 6, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i1.to_i32(), 20);
    }

    #[test]
    fn should_handle_storing_index_zero_edge_cases() {
        let mut computer = Computer::new();
        computer.r_i6 = Index::from_byte_values(Sign::PLUS, 0, 0).unwrap();
        computer.r_i5 = Index::from_byte_values(Sign::MINUS, 0, 0).unwrap();

        computer.handle_instruction(
            Word::from_instruction_parts(Sign::PLUS, 0, 6, FIELD, CODE).unwrap(),
        );
        assert_eq!(computer.r_i1, Index::ZERO.with_sign(Sign::PLUS));

        computer.handle_instruction(
            Word::from_instruction_parts(Sign::MINUS, 0, 6, FIELD, CODE).unwrap(),
        );
        assert_eq!(computer.r_i1, Index::ZERO.with_sign(Sign::MINUS));

        computer.handle_instruction(
            Word::from_instruction_parts(Sign::PLUS, 0, 5, FIELD, CODE).unwrap(),
        );
        assert_eq!(computer.r_i1, Index::ZERO.with_sign(Sign::PLUS));

        computer.handle_instruction(
            Word::from_instruction_parts(Sign::MINUS, 0, 5, FIELD, CODE).unwrap(),
        );
        assert_eq!(computer.r_i1, Index::ZERO.with_sign(Sign::MINUS));
    }
}

mod ent2 {
    use crate::computer::*;
    use crate::data_types::*;

    const CODE: i32 = 50;
    const FIELD: i32 = 2;

    #[test]
    fn should_store_value() {
        let mut computer = Computer::new();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 123, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i2.to_i32(), 123);

        let instruction2 = Word::from_instruction_parts(Sign::MINUS, 234, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction2);
        assert_eq!(computer.r_i2.to_i32(), -234);
    }

    #[test]
    fn should_store_zero_with_correct_sign() {
        let mut computer = Computer::new();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 0, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i2, Index::ZERO.with_sign(Sign::PLUS));

        let instruction = Word::from_instruction_parts(Sign::MINUS, 0, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i2, Index::ZERO.with_sign(Sign::MINUS));
    }

    #[test]
    fn should_store_index_modified_value() {
        let mut computer = Computer::new();
        computer.r_i6 = Index::from_i32(-10).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 30, 6, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i2.to_i32(), 20);
    }

    #[test]
    fn should_handle_storing_index_zero_edge_cases() {
        let mut computer = Computer::new();
        computer.r_i6 = Index::from_byte_values(Sign::PLUS, 0, 0).unwrap();
        computer.r_i5 = Index::from_byte_values(Sign::MINUS, 0, 0).unwrap();

        computer.handle_instruction(
            Word::from_instruction_parts(Sign::PLUS, 0, 6, FIELD, CODE).unwrap(),
        );
        assert_eq!(computer.r_i2, Index::ZERO.with_sign(Sign::PLUS));

        computer.handle_instruction(
            Word::from_instruction_parts(Sign::MINUS, 0, 6, FIELD, CODE).unwrap(),
        );
        assert_eq!(computer.r_i2, Index::ZERO.with_sign(Sign::MINUS));

        computer.handle_instruction(
            Word::from_instruction_parts(Sign::PLUS, 0, 5, FIELD, CODE).unwrap(),
        );
        assert_eq!(computer.r_i2, Index::ZERO.with_sign(Sign::PLUS));

        computer.handle_instruction(
            Word::from_instruction_parts(Sign::MINUS, 0, 5, FIELD, CODE).unwrap(),
        );
        assert_eq!(computer.r_i2, Index::ZERO.with_sign(Sign::MINUS));
    }
}

mod ent3 {
    use crate::computer::*;
    use crate::data_types::*;

    const CODE: i32 = 51;
    const FIELD: i32 = 2;

    #[test]
    fn should_store_value() {
        let mut computer = Computer::new();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 123, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i3.to_i32(), 123);

        let instruction2 = Word::from_instruction_parts(Sign::MINUS, 234, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction2);
        assert_eq!(computer.r_i3.to_i32(), -234);
    }

    #[test]
    fn should_store_zero_with_correct_sign() {
        let mut computer = Computer::new();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 0, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i3, Index::ZERO.with_sign(Sign::PLUS));

        let instruction = Word::from_instruction_parts(Sign::MINUS, 0, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i3, Index::ZERO.with_sign(Sign::MINUS));
    }

    #[test]
    fn should_store_index_modified_value() {
        let mut computer = Computer::new();
        computer.r_i6 = Index::from_i32(-10).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 30, 6, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i3.to_i32(), 20);
    }

    #[test]
    fn should_handle_storing_index_zero_edge_cases() {
        let mut computer = Computer::new();
        computer.r_i6 = Index::from_byte_values(Sign::PLUS, 0, 0).unwrap();
        computer.r_i5 = Index::from_byte_values(Sign::MINUS, 0, 0).unwrap();

        computer.handle_instruction(
            Word::from_instruction_parts(Sign::PLUS, 0, 6, FIELD, CODE).unwrap(),
        );
        assert_eq!(computer.r_i3, Index::ZERO.with_sign(Sign::PLUS));

        computer.handle_instruction(
            Word::from_instruction_parts(Sign::MINUS, 0, 6, FIELD, CODE).unwrap(),
        );
        assert_eq!(computer.r_i3, Index::ZERO.with_sign(Sign::MINUS));

        computer.handle_instruction(
            Word::from_instruction_parts(Sign::PLUS, 0, 5, FIELD, CODE).unwrap(),
        );
        assert_eq!(computer.r_i3, Index::ZERO.with_sign(Sign::PLUS));

        computer.handle_instruction(
            Word::from_instruction_parts(Sign::MINUS, 0, 5, FIELD, CODE).unwrap(),
        );
        assert_eq!(computer.r_i3, Index::ZERO.with_sign(Sign::MINUS));
    }
}

mod ent4 {
    use crate::computer::*;
    use crate::data_types::*;

    const CODE: i32 = 52;
    const FIELD: i32 = 2;

    #[test]
    fn should_store_value() {
        let mut computer = Computer::new();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 123, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i4.to_i32(), 123);

        let instruction2 = Word::from_instruction_parts(Sign::MINUS, 234, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction2);
        assert_eq!(computer.r_i4.to_i32(), -234);
    }

    #[test]
    fn should_store_zero_with_correct_sign() {
        let mut computer = Computer::new();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 0, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i4, Index::ZERO.with_sign(Sign::PLUS));

        let instruction = Word::from_instruction_parts(Sign::MINUS, 0, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i4, Index::ZERO.with_sign(Sign::MINUS));
    }

    #[test]
    fn should_store_index_modified_value() {
        let mut computer = Computer::new();
        computer.r_i6 = Index::from_i32(-10).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 30, 6, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i4.to_i32(), 20);
    }

    #[test]
    fn should_handle_storing_index_zero_edge_cases() {
        let mut computer = Computer::new();
        computer.r_i6 = Index::from_byte_values(Sign::PLUS, 0, 0).unwrap();
        computer.r_i5 = Index::from_byte_values(Sign::MINUS, 0, 0).unwrap();

        computer.handle_instruction(
            Word::from_instruction_parts(Sign::PLUS, 0, 6, FIELD, CODE).unwrap(),
        );
        assert_eq!(computer.r_i4, Index::ZERO.with_sign(Sign::PLUS));

        computer.handle_instruction(
            Word::from_instruction_parts(Sign::MINUS, 0, 6, FIELD, CODE).unwrap(),
        );
        assert_eq!(computer.r_i4, Index::ZERO.with_sign(Sign::MINUS));

        computer.handle_instruction(
            Word::from_instruction_parts(Sign::PLUS, 0, 5, FIELD, CODE).unwrap(),
        );
        assert_eq!(computer.r_i4, Index::ZERO.with_sign(Sign::PLUS));

        computer.handle_instruction(
            Word::from_instruction_parts(Sign::MINUS, 0, 5, FIELD, CODE).unwrap(),
        );
        assert_eq!(computer.r_i4, Index::ZERO.with_sign(Sign::MINUS));
    }
}

mod ent5 {
    use crate::computer::*;
    use crate::data_types::*;

    const CODE: i32 = 53;
    const FIELD:i32 = 2;

    #[test]
    fn should_store_value() {
        let mut computer = Computer::new();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 123, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i5.to_i32(), 123);

        let instruction2 = Word::from_instruction_parts(Sign::MINUS, 234, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction2);
        assert_eq!(computer.r_i5.to_i32(), -234);
    }

    #[test]
    fn should_store_zero_with_correct_sign() {
        let mut computer = Computer::new();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 0, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i5, Index::ZERO.with_sign(Sign::PLUS));

        let instruction = Word::from_instruction_parts(Sign::MINUS, 0, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i5, Index::ZERO.with_sign(Sign::MINUS));
    }

    #[test]
    fn should_store_index_modified_value() {
        let mut computer = Computer::new();
        computer.r_i6 = Index::from_i32(-10).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 30, 6, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i5.to_i32(), 20);
    }

    #[test]
    fn should_handle_storing_index_zero_edge_cases() {
        let mut computer = Computer::new();
        computer.r_i6 = Index::from_byte_values(Sign::PLUS, 0, 0).unwrap();
        computer.r_i5 = Index::from_byte_values(Sign::MINUS, 0, 0).unwrap();

        computer.handle_instruction(Word::from_instruction_parts(Sign::PLUS, 0, 6, FIELD, CODE).unwrap());
        assert_eq!(computer.r_i5, Index::ZERO.with_sign(Sign::PLUS));

        computer.handle_instruction(Word::from_instruction_parts(Sign::MINUS, 0, 6, FIELD, CODE).unwrap());
        assert_eq!(computer.r_i5, Index::ZERO.with_sign(Sign::MINUS));

        computer.handle_instruction(Word::from_instruction_parts(Sign::PLUS, 0, 5, FIELD, CODE).unwrap());
        assert_eq!(computer.r_i5, Index::ZERO.with_sign(Sign::PLUS));

        computer.handle_instruction(Word::from_instruction_parts(Sign::MINUS, 0, 5, FIELD, CODE).unwrap());
        assert_eq!(computer.r_i5, Index::ZERO.with_sign(Sign::MINUS));
    }
}

mod ent6 {
    use crate::computer::*;
    use crate::data_types::*;

    const CODE: i32 = 54;
    const FIELD:i32 = 2;

    #[test]
    fn should_store_value() {
        let mut computer = Computer::new();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 123, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i6.to_i32(), 123);

        let instruction2 = Word::from_instruction_parts(Sign::MINUS, 234, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction2);
        assert_eq!(computer.r_i6.to_i32(), -234);
    }

    #[test]
    fn should_store_zero_with_correct_sign() {
        let mut computer = Computer::new();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 0, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i6, Index::ZERO.with_sign(Sign::PLUS));

        let instruction = Word::from_instruction_parts(Sign::MINUS, 0, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i6, Index::ZERO.with_sign(Sign::MINUS));
    }

    #[test]
    fn should_store_index_modified_value() {
        let mut computer = Computer::new();
        computer.r_i6 = Index::from_i32(-10).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 30, 6, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i6.to_i32(), 20);
    }

    #[test]
    fn should_handle_storing_index_zero_edge_cases() {
        let mut computer = Computer::new();
        computer.r_i6 = Index::from_byte_values(Sign::PLUS, 0, 0).unwrap();
        computer.r_i5 = Index::from_byte_values(Sign::MINUS, 0, 0).unwrap();

        computer.handle_instruction(Word::from_instruction_parts(Sign::PLUS, 0, 6, FIELD, CODE).unwrap());
        assert_eq!(computer.r_i6, Index::ZERO.with_sign(Sign::PLUS));

        computer.handle_instruction(Word::from_instruction_parts(Sign::MINUS, 0, 6, FIELD, CODE).unwrap());
        assert_eq!(computer.r_i6, Index::ZERO.with_sign(Sign::MINUS));

        computer.handle_instruction(Word::from_instruction_parts(Sign::PLUS, 0, 5, FIELD, CODE).unwrap());
        assert_eq!(computer.r_i6, Index::ZERO.with_sign(Sign::PLUS));

        computer.handle_instruction(Word::from_instruction_parts(Sign::MINUS, 0, 5, FIELD, CODE).unwrap());
        assert_eq!(computer.r_i6, Index::ZERO.with_sign(Sign::MINUS));
    }
}

mod enna {
    use crate::computer::*;
    use crate::data_types::*;

    const CODE: i32 = 48;
    const FIELD: i32 = 3;

    #[test]
    fn should_store_value() {
        let mut computer = Computer::new();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 123, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_a.to_i32(), -123);

        let instruction2 = Word::from_instruction_parts(Sign::MINUS, 234, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction2);
        assert_eq!(computer.r_a.to_i32(), 234);
    }

    #[test]
    fn should_store_zero_with_correct_sign() {
        let mut computer = Computer::new();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 0, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_a, Word::ZERO.with_sign(Sign::MINUS));

        let instruction = Word::from_instruction_parts(Sign::MINUS, 0, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_a, Word::ZERO.with_sign(Sign::PLUS));
    }

    #[test]
    fn should_store_index_modified_value() {
        let mut computer = Computer::new();
        computer.r_i1 = Index::from_i32(-10).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 30, 1, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_a.to_i32(), -20);
    }

    #[test]
    fn should_handle_storing_index_zero_edge_cases() {
        let mut computer = Computer::new();
        computer.r_i1 = Index::from_byte_values(Sign::PLUS, 0, 0).unwrap();
        computer.r_i2 = Index::from_byte_values(Sign::MINUS, 0, 0).unwrap();

        computer.handle_instruction(Word::from_instruction_parts(Sign::PLUS, 0, 1, FIELD, CODE).unwrap());
        assert_eq!(computer.r_a, Word::ZERO.with_sign(Sign::MINUS));

        computer.handle_instruction(Word::from_instruction_parts(Sign::MINUS, 0, 1, FIELD, CODE).unwrap());
        assert_eq!(computer.r_a, Word::ZERO.with_sign(Sign::PLUS));

        computer.handle_instruction(Word::from_instruction_parts(Sign::PLUS, 0, 2, FIELD, CODE).unwrap());
        assert_eq!(computer.r_a, Word::ZERO.with_sign(Sign::MINUS));

        computer.handle_instruction(Word::from_instruction_parts(Sign::MINUS, 0, 2, FIELD, CODE).unwrap());
        assert_eq!(computer.r_a, Word::ZERO.with_sign(Sign::PLUS));
    }
}

mod ennx {
    use crate::computer::*;
    use crate::data_types::*;

    const CODE: i32 = 55;
    const FIELD: i32 = 3;

    #[test]
    fn should_store_value() {
        let mut computer = Computer::new();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 123, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_x.to_i32(), -123);

        let instruction2 = Word::from_instruction_parts(Sign::MINUS, 234, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction2);
        assert_eq!(computer.r_x.to_i32(), 234);
    }

    #[test]
    fn should_store_zero_with_correct_sign() {
        let mut computer = Computer::new();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 0, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_x, Word::ZERO.with_sign(Sign::MINUS));

        let instruction = Word::from_instruction_parts(Sign::MINUS, 0, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_x, Word::ZERO.with_sign(Sign::PLUS));
    }

    #[test]
    fn should_store_index_modified_value() {
        let mut computer = Computer::new();
        computer.r_i1 = Index::from_i32(-10).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 30, 1, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_x.to_i32(), -20);
    }

    #[test]
    fn should_handle_storing_index_zero_edge_cases() {
        let mut computer = Computer::new();
        computer.r_i1 = Index::from_byte_values(Sign::PLUS, 0, 0).unwrap();
        computer.r_i2 = Index::from_byte_values(Sign::MINUS, 0, 0).unwrap();

        computer.handle_instruction(Word::from_instruction_parts(Sign::PLUS, 0, 1, FIELD, CODE).unwrap());
        assert_eq!(computer.r_x, Word::ZERO.with_sign(Sign::MINUS));

        computer.handle_instruction(Word::from_instruction_parts(Sign::MINUS, 0, 1, FIELD, CODE).unwrap());
        assert_eq!(computer.r_x, Word::ZERO.with_sign(Sign::PLUS));

        computer.handle_instruction(Word::from_instruction_parts(Sign::PLUS, 0, 2, FIELD, CODE).unwrap());
        assert_eq!(computer.r_x, Word::ZERO.with_sign(Sign::MINUS));

        computer.handle_instruction(Word::from_instruction_parts(Sign::MINUS, 0, 2, FIELD, CODE).unwrap());
        assert_eq!(computer.r_x, Word::ZERO.with_sign(Sign::PLUS));
    }
}

mod enn1 {
    use crate::computer::*;
    use crate::data_types::*;

    const CODE: i32 = 49;
    const FIELD: i32 = 3;

    #[test]
    fn should_store_value() {
        let mut computer = Computer::new();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 123, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i1.to_i32(), -123);

        let instruction2 = Word::from_instruction_parts(Sign::MINUS, 234, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction2);
        assert_eq!(computer.r_i1.to_i32(), 234);
    }

    #[test]
    fn should_store_zero_with_correct_sign() {
        let mut computer = Computer::new();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 0, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i1, Index::ZERO.with_sign(Sign::MINUS));

        let instruction = Word::from_instruction_parts(Sign::MINUS, 0, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i1, Index::ZERO.with_sign(Sign::PLUS));
    }

    #[test]
    fn should_store_index_modified_value() {
        let mut computer = Computer::new();
        computer.r_i6 = Index::from_i32(-10).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 30, 6, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i1.to_i32(), -20);
    }

    #[test]
    fn should_handle_storing_index_zero_edge_cases() {
        let mut computer = Computer::new();
        computer.r_i6 = Index::from_byte_values(Sign::PLUS, 0, 0).unwrap();
        computer.r_i5 = Index::from_byte_values(Sign::MINUS, 0, 0).unwrap();

        computer.handle_instruction(Word::from_instruction_parts(Sign::PLUS, 0, 6, FIELD, CODE).unwrap());
        assert_eq!(computer.r_i1, Index::ZERO.with_sign(Sign::MINUS));

        computer.handle_instruction(Word::from_instruction_parts(Sign::MINUS, 0, 6, FIELD, CODE).unwrap());
        assert_eq!(computer.r_i1, Index::ZERO.with_sign(Sign::PLUS));

        computer.handle_instruction(Word::from_instruction_parts(Sign::PLUS, 0, 5, FIELD, CODE).unwrap());
        assert_eq!(computer.r_i1, Index::ZERO.with_sign(Sign::MINUS));

        computer.handle_instruction(Word::from_instruction_parts(Sign::MINUS, 0, 5, FIELD, CODE).unwrap());
        assert_eq!(computer.r_i1, Index::ZERO.with_sign(Sign::PLUS));
    }
}

mod enn2 {
    use crate::computer::*;
    use crate::data_types::*;

    const CODE: i32 = 50;
    const FIELD: i32 = 3;

    #[test]
    fn should_store_value() {
        let mut computer = Computer::new();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 123, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i2.to_i32(), -123);

        let instruction2 = Word::from_instruction_parts(Sign::MINUS, 234, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction2);
        assert_eq!(computer.r_i2.to_i32(), 234);
    }

    #[test]
    fn should_store_zero_with_correct_sign() {
        let mut computer = Computer::new();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 0, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i2, Index::ZERO.with_sign(Sign::MINUS));

        let instruction = Word::from_instruction_parts(Sign::MINUS, 0, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i2, Index::ZERO.with_sign(Sign::PLUS));
    }

    #[test]
    fn should_store_index_modified_value() {
        let mut computer = Computer::new();
        computer.r_i6 = Index::from_i32(-10).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 30, 6, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i2.to_i32(), -20);
    }

    #[test]
    fn should_handle_storing_index_zero_edge_cases() {
        let mut computer = Computer::new();
        computer.r_i6 = Index::from_byte_values(Sign::PLUS, 0, 0).unwrap();
        computer.r_i5 = Index::from_byte_values(Sign::MINUS, 0, 0).unwrap();

        computer.handle_instruction(Word::from_instruction_parts(Sign::PLUS, 0, 6, FIELD, CODE).unwrap());
        assert_eq!(computer.r_i2, Index::ZERO.with_sign(Sign::MINUS));

        computer.handle_instruction(Word::from_instruction_parts(Sign::MINUS, 0, 6, FIELD, CODE).unwrap());
        assert_eq!(computer.r_i2, Index::ZERO.with_sign(Sign::PLUS));

        computer.handle_instruction(Word::from_instruction_parts(Sign::PLUS, 0, 5, FIELD, CODE).unwrap());
        assert_eq!(computer.r_i2, Index::ZERO.with_sign(Sign::MINUS));

        computer.handle_instruction(Word::from_instruction_parts(Sign::MINUS, 0, 5, FIELD, CODE).unwrap());
        assert_eq!(computer.r_i2, Index::ZERO.with_sign(Sign::PLUS));
    }
}

mod enn3 {
    use crate::computer::*;
    use crate::data_types::*;

    const CODE: i32 = 51;
    const FIELD: i32 = 3;

    #[test]
    fn should_store_value() {
        let mut computer = Computer::new();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 123, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i3.to_i32(), -123);

        let instruction2 = Word::from_instruction_parts(Sign::MINUS, 234, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction2);
        assert_eq!(computer.r_i3.to_i32(), 234);
    }

    #[test]
    fn should_store_zero_with_correct_sign() {
        let mut computer = Computer::new();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 0, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i3, Index::ZERO.with_sign(Sign::MINUS));

        let instruction = Word::from_instruction_parts(Sign::MINUS, 0, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i3, Index::ZERO.with_sign(Sign::PLUS));
    }

    #[test]
    fn should_store_index_modified_value() {
        let mut computer = Computer::new();
        computer.r_i6 = Index::from_i32(-10).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 30, 6, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i3.to_i32(), -20);
    }

    #[test]
    fn should_handle_storing_index_zero_edge_cases() {
        let mut computer = Computer::new();
        computer.r_i6 = Index::from_byte_values(Sign::PLUS, 0, 0).unwrap();
        computer.r_i5 = Index::from_byte_values(Sign::MINUS, 0, 0).unwrap();

        computer.handle_instruction(Word::from_instruction_parts(Sign::PLUS, 0, 6, FIELD, CODE).unwrap());
        assert_eq!(computer.r_i3, Index::ZERO.with_sign(Sign::MINUS));

        computer.handle_instruction(Word::from_instruction_parts(Sign::MINUS, 0, 6, FIELD, CODE).unwrap());
        assert_eq!(computer.r_i3, Index::ZERO.with_sign(Sign::PLUS));

        computer.handle_instruction(Word::from_instruction_parts(Sign::PLUS, 0, 5, FIELD, CODE).unwrap());
        assert_eq!(computer.r_i3, Index::ZERO.with_sign(Sign::MINUS));

        computer.handle_instruction(Word::from_instruction_parts(Sign::MINUS, 0, 5, FIELD, CODE).unwrap());
        assert_eq!(computer.r_i3, Index::ZERO.with_sign(Sign::PLUS));
    }
}

mod enn4 {
    use crate::computer::*;
    use crate::data_types::*;

    const CODE: i32 = 52;
    const FIELD: i32 = 3;

    #[test]
    fn should_store_value() {
        let mut computer = Computer::new();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 123, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i4.to_i32(), -123);

        let instruction2 = Word::from_instruction_parts(Sign::MINUS, 234, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction2);
        assert_eq!(computer.r_i4.to_i32(), 234);
    }

    #[test]
    fn should_store_zero_with_correct_sign() {
        let mut computer = Computer::new();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 0, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i4, Index::ZERO.with_sign(Sign::MINUS));

        let instruction = Word::from_instruction_parts(Sign::MINUS, 0, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i4, Index::ZERO.with_sign(Sign::PLUS));
    }

    #[test]
    fn should_store_index_modified_value() {
        let mut computer = Computer::new();
        computer.r_i6 = Index::from_i32(-10).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 30, 6, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i4.to_i32(), -20);
    }

    #[test]
    fn should_handle_storing_index_zero_edge_cases() {
        let mut computer = Computer::new();
        computer.r_i6 = Index::from_byte_values(Sign::PLUS, 0, 0).unwrap();
        computer.r_i5 = Index::from_byte_values(Sign::MINUS, 0, 0).unwrap();

        computer.handle_instruction(Word::from_instruction_parts(Sign::PLUS, 0, 6, FIELD, CODE).unwrap());
        assert_eq!(computer.r_i4, Index::ZERO.with_sign(Sign::MINUS));

        computer.handle_instruction(Word::from_instruction_parts(Sign::MINUS, 0, 6, FIELD, CODE).unwrap());
        assert_eq!(computer.r_i4, Index::ZERO.with_sign(Sign::PLUS));

        computer.handle_instruction(Word::from_instruction_parts(Sign::PLUS, 0, 5, FIELD, CODE).unwrap());
        assert_eq!(computer.r_i4, Index::ZERO.with_sign(Sign::MINUS));

        computer.handle_instruction(Word::from_instruction_parts(Sign::MINUS, 0, 5, FIELD, CODE).unwrap());
        assert_eq!(computer.r_i4, Index::ZERO.with_sign(Sign::PLUS));
    }
}

mod enn5 {
    use crate::computer::*;
    use crate::data_types::*;

    const CODE: i32 = 53;
    const FIELD: i32 = 3;

    #[test]
    fn should_store_value() {
        let mut computer = Computer::new();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 123, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i5.to_i32(), -123);

        let instruction2 = Word::from_instruction_parts(Sign::MINUS, 234, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction2);
        assert_eq!(computer.r_i5.to_i32(), 234);
    }

    #[test]
    fn should_store_zero_with_correct_sign() {
        let mut computer = Computer::new();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 0, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i5, Index::ZERO.with_sign(Sign::MINUS));

        let instruction = Word::from_instruction_parts(Sign::MINUS, 0, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i5, Index::ZERO.with_sign(Sign::PLUS));
    }

    #[test]
    fn should_store_index_modified_value() {
        let mut computer = Computer::new();
        computer.r_i6 = Index::from_i32(-10).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 30, 6, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i5.to_i32(), -20);
    }

    #[test]
    fn should_handle_storing_index_zero_edge_cases() {
        let mut computer = Computer::new();
        computer.r_i6 = Index::from_byte_values(Sign::PLUS, 0, 0).unwrap();
        computer.r_i5 = Index::from_byte_values(Sign::MINUS, 0, 0).unwrap();

        computer.handle_instruction(Word::from_instruction_parts(Sign::PLUS, 0, 6, FIELD, CODE).unwrap());
        assert_eq!(computer.r_i5, Index::ZERO.with_sign(Sign::MINUS));

        computer.handle_instruction(Word::from_instruction_parts(Sign::MINUS, 0, 6, FIELD, CODE).unwrap());
        assert_eq!(computer.r_i5, Index::ZERO.with_sign(Sign::PLUS));

        computer.handle_instruction(Word::from_instruction_parts(Sign::PLUS, 0, 5, FIELD, CODE).unwrap());
        assert_eq!(computer.r_i5, Index::ZERO.with_sign(Sign::MINUS));

        computer.handle_instruction(Word::from_instruction_parts(Sign::MINUS, 0, 5, FIELD, CODE).unwrap());
        assert_eq!(computer.r_i5, Index::ZERO.with_sign(Sign::PLUS));
    }
}

mod enn6 {
    use crate::computer::*;
    use crate::data_types::*;

    const CODE: i32 = 54;
    const FIELD: i32 = 3;

    #[test]
    fn should_store_value() {
        let mut computer = Computer::new();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 123, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i6.to_i32(), -123);

        let instruction2 = Word::from_instruction_parts(Sign::MINUS, 234, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction2);
        assert_eq!(computer.r_i6.to_i32(), 234);
    }

    #[test]
    fn should_store_zero_with_correct_sign() {
        let mut computer = Computer::new();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 0, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i6, Index::ZERO.with_sign(Sign::MINUS));

        let instruction = Word::from_instruction_parts(Sign::MINUS, 0, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i6, Index::ZERO.with_sign(Sign::PLUS));
    }

    #[test]
    fn should_store_index_modified_value() {
        let mut computer = Computer::new();
        computer.r_i6 = Index::from_i32(-10).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 30, 6, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i6.to_i32(), -20);
    }

    #[test]
    fn should_handle_storing_index_zero_edge_cases() {
        let mut computer = Computer::new();
        computer.r_i6 = Index::from_byte_values(Sign::PLUS, 0, 0).unwrap();
        computer.r_i5 = Index::from_byte_values(Sign::MINUS, 0, 0).unwrap();

        computer.handle_instruction(Word::from_instruction_parts(Sign::PLUS, 0, 6, FIELD, CODE).unwrap());
        assert_eq!(computer.r_i6, Index::ZERO.with_sign(Sign::MINUS));

        computer.handle_instruction(Word::from_instruction_parts(Sign::MINUS, 0, 6, FIELD, CODE).unwrap());
        assert_eq!(computer.r_i6, Index::ZERO.with_sign(Sign::PLUS));

        computer.handle_instruction(Word::from_instruction_parts(Sign::PLUS, 0, 5, FIELD, CODE).unwrap());
        assert_eq!(computer.r_i6, Index::ZERO.with_sign(Sign::MINUS));

        computer.handle_instruction(Word::from_instruction_parts(Sign::MINUS, 0, 5, FIELD, CODE).unwrap());
        assert_eq!(computer.r_i6, Index::ZERO.with_sign(Sign::PLUS));
    }
}

mod inca {
    use crate::computer::*;
    use crate::data_types::*;

    const CODE: i32 = 48;
    const FIELD: i32 = 0;

    #[test]
    fn should_add_positive_number() {
        let mut computer = Computer::new();
        computer.r_a = Word::from_i32(1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 789, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_a.to_i32(), 790);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_add_negative_number() {
        let mut computer = Computer::new();
        computer.r_a = Word::from_i32(1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::MINUS, 789, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_a.to_i32(), -788);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_handle_positive_overflow() {
        let mut computer = Computer::new();
        computer.r_a = Word::MAX;
        let instruction = Word::from_instruction_parts(Sign::PLUS, 10, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_a.to_i32(), 9);
        assert!(computer.overflow);
    }

    #[test]
    fn should_handle_negative_overflow() {
        let mut computer = Computer::new();
        computer.r_a = Word::MIN;
        let instruction = Word::from_instruction_parts(Sign::MINUS, 10, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_a.to_i32(), -9);
        assert!(computer.overflow);
    }

    #[test]
    fn should_not_change_sign_if_result_is_zero() {
        let mut computer = Computer::new();
        computer.r_a = Word::from_i32(-1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_a.to_i32(), 0);
        assert_eq!(computer.r_a.sign, Sign::MINUS);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_add_index_modified_value() {
        let mut computer = Computer::new();
        computer.r_a = Word::from_i32(1).unwrap();
        computer.r_i1 = Index::from_i32(2).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 3, 1, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_a.to_i32(), 6);
        assert!(!computer.overflow);
    }
}

mod incx {
    use crate::computer::*;
    use crate::data_types::*;

    const CODE: i32 = 55;
    const FIELD: i32 = 0;

    #[test]
    fn should_add_positive_number() {
        let mut computer = Computer::new();
        computer.r_x = Word::from_i32(1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 789, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_x.to_i32(), 790);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_add_negative_number() {
        let mut computer = Computer::new();
        computer.r_x = Word::from_i32(1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::MINUS, 789, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_x.to_i32(), -788);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_handle_positive_overflow() {
        let mut computer = Computer::new();
        computer.r_x = Word::MAX;
        let instruction = Word::from_instruction_parts(Sign::PLUS, 10, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_x.to_i32(), 9);
        assert!(computer.overflow);
    }

    #[test]
    fn should_handle_negative_overflow() {
        let mut computer = Computer::new();
        computer.r_x = Word::MIN;
        let instruction = Word::from_instruction_parts(Sign::MINUS, 10, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_x.to_i32(), -9);
        assert!(computer.overflow);
    }

    #[test]
    fn should_not_change_sign_if_result_is_zero() {
        let mut computer = Computer::new();
        computer.r_x = Word::from_i32(-1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_x.to_i32(), 0);
        assert_eq!(computer.r_x.sign, Sign::MINUS);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_add_index_modified_value() {
        let mut computer = Computer::new();
        computer.r_x = Word::from_i32(1).unwrap();
        computer.r_i1 = Index::from_i32(2).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 3, 1, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_x.to_i32(), 6);
        assert!(!computer.overflow);
    }
}

mod inc1 {
    use crate::computer::*;
    use crate::data_types::*;

    const CODE: i32 = 49;
    const FIELD: i32 = 0;

    #[test]
    fn should_add_positive_number() {
        let mut computer = Computer::new();
        computer.r_i1 = Index::from_i32(1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 789, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i1.to_i32(), 790);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_add_negative_number() {
        let mut computer = Computer::new();
        computer.r_i1 = Index::from_i32(1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::MINUS, 789, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i1.to_i32(), -788);
        assert!(!computer.overflow);
    }
    
    #[test]
    fn should_not_change_sign_if_result_is_zero() {
        let mut computer = Computer::new();
        computer.r_i1 = Index::from_i32(-1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i1.to_i32(), 0);
        assert_eq!(computer.r_i1.sign, Sign::MINUS);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_add_index_modified_value() {
        let mut computer = Computer::new();
        computer.r_i1 = Index::from_i32(1).unwrap();
        computer.r_i2 = Index::from_i32(2).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 3, 2, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i1.to_i32(), 6);
        assert!(!computer.overflow);
    }
}

mod inc2 {
    use crate::computer::*;
    use crate::data_types::*;

    const CODE: i32 = 50;
    const FIELD: i32 = 0;

    #[test]
    fn should_add_positive_number() {
        let mut computer = Computer::new();
        computer.r_i2 = Index::from_i32(1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 789, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i2.to_i32(), 790);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_add_negative_number() {
        let mut computer = Computer::new();
        computer.r_i2 = Index::from_i32(1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::MINUS, 789, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i2.to_i32(), -788);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_not_change_sign_if_result_is_zero() {
        let mut computer = Computer::new();
        computer.r_i2 = Index::from_i32(-1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i2.to_i32(), 0);
        assert_eq!(computer.r_i2.sign, Sign::MINUS);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_add_index_modified_value() {
        let mut computer = Computer::new();
        computer.r_i1 = Index::from_i32(1).unwrap();
        computer.r_i2 = Index::from_i32(2).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 3, 1, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i2.to_i32(), 6);
        assert!(!computer.overflow);
    }
}

mod inc3 {
    use crate::computer::*;
    use crate::data_types::*;

    const CODE: i32 = 51;
    const FIELD: i32 = 0;

    #[test]
    fn should_add_positive_number() {
        let mut computer = Computer::new();
        computer.r_i3 = Index::from_i32(1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 789, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i3.to_i32(), 790);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_add_negative_number() {
        let mut computer = Computer::new();
        computer.r_i3 = Index::from_i32(1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::MINUS, 789, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i3.to_i32(), -788);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_not_change_sign_if_result_is_zero() {
        let mut computer = Computer::new();
        computer.r_i3 = Index::from_i32(-1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i3.to_i32(), 0);
        assert_eq!(computer.r_i3.sign, Sign::MINUS);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_add_index_modified_value() {
        let mut computer = Computer::new();
        computer.r_i1 = Index::from_i32(1).unwrap();
        computer.r_i3 = Index::from_i32(2).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 3, 1, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i3.to_i32(), 6);
        assert!(!computer.overflow);
    }
}

mod inc4 {
    use crate::computer::*;
    use crate::data_types::*;

    const CODE: i32 = 52;
    const FIELD: i32 = 0;

    #[test]
    fn should_add_positive_number() {
        let mut computer = Computer::new();
        computer.r_i4 = Index::from_i32(1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 789, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i4.to_i32(), 790);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_add_negative_number() {
        let mut computer = Computer::new();
        computer.r_i4 = Index::from_i32(1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::MINUS, 789, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i4.to_i32(), -788);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_not_change_sign_if_result_is_zero() {
        let mut computer = Computer::new();
        computer.r_i4 = Index::from_i32(-1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i4.to_i32(), 0);
        assert_eq!(computer.r_i4.sign, Sign::MINUS);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_add_index_modified_value() {
        let mut computer = Computer::new();
        computer.r_i1 = Index::from_i32(1).unwrap();
        computer.r_i4 = Index::from_i32(2).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 3, 1, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i4.to_i32(), 6);
        assert!(!computer.overflow);
    }
}

mod inc5 {
    use crate::computer::*;
    use crate::data_types::*;

    const CODE: i32 = 53;
    const FIELD: i32 = 0;

    #[test]
    fn should_add_positive_number() {
        let mut computer = Computer::new();
        computer.r_i5 = Index::from_i32(1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 789, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i5.to_i32(), 790);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_add_negative_number() {
        let mut computer = Computer::new();
        computer.r_i5 = Index::from_i32(1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::MINUS, 789, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i5.to_i32(), -788);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_not_change_sign_if_result_is_zero() {
        let mut computer = Computer::new();
        computer.r_i5 = Index::from_i32(-1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i5.to_i32(), 0);
        assert_eq!(computer.r_i5.sign, Sign::MINUS);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_add_index_modified_value() {
        let mut computer = Computer::new();
        computer.r_i1 = Index::from_i32(1).unwrap();
        computer.r_i5 = Index::from_i32(2).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 3, 1, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i5.to_i32(), 6);
        assert!(!computer.overflow);
    }
}

mod inc6 {
    use crate::computer::*;
    use crate::data_types::*;

    const CODE: i32 = 54;
    const FIELD: i32 = 0;

    #[test]
    fn should_add_positive_number() {
        let mut computer = Computer::new();
        computer.r_i6 = Index::from_i32(1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 789, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i6.to_i32(), 790);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_add_negative_number() {
        let mut computer = Computer::new();
        computer.r_i6 = Index::from_i32(1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::MINUS, 789, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i6.to_i32(), -788);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_not_change_sign_if_result_is_zero() {
        let mut computer = Computer::new();
        computer.r_i6 = Index::from_i32(-1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i6.to_i32(), 0);
        assert_eq!(computer.r_i6.sign, Sign::MINUS);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_add_index_modified_value() {
        let mut computer = Computer::new();
        computer.r_i1 = Index::from_i32(1).unwrap();
        computer.r_i6 = Index::from_i32(2).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 3, 1, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i6.to_i32(), 6);
        assert!(!computer.overflow);
    }
}

mod deca {
    use crate::computer::*;
    use crate::data_types::*;

    const CODE: i32 = 48;
    const FIELD: i32 = 1;

    #[test]
    fn should_subtract_positive_number() {
        let mut computer = Computer::new();
        computer.r_a = Word::from_i32(1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 789, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_a.to_i32(), -788);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_subtract_negative_number() {
        let mut computer = Computer::new();
        computer.r_a = Word::from_i32(1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::MINUS, 789, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_a.to_i32(), 790);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_handle_positive_overflow() {
        let mut computer = Computer::new();
        computer.r_a = Word::MAX;
        let instruction = Word::from_instruction_parts(Sign::MINUS, 10, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_a.to_i32(), 9);
        assert!(computer.overflow);
    }

    #[test]
    fn should_handle_negative_overflow() {
        let mut computer = Computer::new();
        computer.r_a = Word::MIN;
        let instruction = Word::from_instruction_parts(Sign::PLUS, 10, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_a.to_i32(), -9);
        assert!(computer.overflow);
    }

    #[test]
    fn should_not_change_sign_if_result_is_zero() {
        let mut computer = Computer::new();
        computer.r_a = Word::from_i32(-1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::MINUS, 1, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_a.to_i32(), 0);
        assert_eq!(computer.r_a.sign, Sign::MINUS);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_subtract_index_modified_value() {
        let mut computer = Computer::new();
        computer.r_a = Word::from_i32(1).unwrap();
        computer.r_i1 = Index::from_i32(8).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 3, 1, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_a.to_i32(), 6);
        assert!(!computer.overflow);
    }
}

mod decx {
    use crate::computer::*;
    use crate::data_types::*;

    const CODE: i32 = 55;
    const FIELD: i32 = 1;

    #[test]
    fn should_subtract_positive_number() {
        let mut computer = Computer::new();
        computer.r_x = Word::from_i32(1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 789, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_x.to_i32(), -788);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_subtract_negative_number() {
        let mut computer = Computer::new();
        computer.r_x = Word::from_i32(1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::MINUS, 789, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_x.to_i32(), 790);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_handle_positive_overflow() {
        let mut computer = Computer::new();
        computer.r_x = Word::MAX;
        let instruction = Word::from_instruction_parts(Sign::MINUS, 10, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_x.to_i32(), 9);
        assert!(computer.overflow);
    }

    #[test]
    fn should_handle_negative_overflow() {
        let mut computer = Computer::new();
        computer.r_x = Word::MIN;
        let instruction = Word::from_instruction_parts(Sign::PLUS, 10, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_x.to_i32(), -9);
        assert!(computer.overflow);
    }

    #[test]
    fn should_not_change_sign_if_result_is_zero() {
        let mut computer = Computer::new();
        computer.r_x = Word::from_i32(-1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::MINUS, 1, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_x.to_i32(), 0);
        assert_eq!(computer.r_x.sign, Sign::MINUS);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_subtract_index_modified_value() {
        let mut computer = Computer::new();
        computer.r_x = Word::from_i32(1).unwrap();
        computer.r_i1 = Index::from_i32(8).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 3, 1, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_x.to_i32(), 6);
        assert!(!computer.overflow);
    }
}

mod dec1 {
    use crate::computer::*;
    use crate::data_types::*;

    const CODE: i32 = 49;
    const FIELD: i32 = 1;

    #[test]
    fn should_subtract_positive_number() {
        let mut computer = Computer::new();
        computer.r_i1 = Index::from_i32(1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 789, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i1.to_i32(), -788);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_subtract_negative_number() {
        let mut computer = Computer::new();
        computer.r_i1 = Index::from_i32(1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::MINUS, 789, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i1.to_i32(), 790);
        assert!(!computer.overflow);
    }
    
    #[test]
    fn should_not_change_sign_if_result_is_zero() {
        let mut computer = Computer::new();
        computer.r_i1 = Index::from_i32(-1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::MINUS, 1, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i1.to_i32(), 0);
        assert_eq!(computer.r_i1.sign, Sign::MINUS);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_subtract_index_modified_value() {
        let mut computer = Computer::new();
        computer.r_i1 = Index::from_i32(1).unwrap();
        computer.r_i2 = Index::from_i32(8).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 3, 2, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i1.to_i32(), 6);
        assert!(!computer.overflow);
    }
}

mod dec2 {
    use crate::computer::*;
    use crate::data_types::*;

    const CODE: i32 = 50;
    const FIELD: i32 = 1;

    #[test]
    fn should_subtract_positive_number() {
        let mut computer = Computer::new();
        computer.r_i2 = Index::from_i32(1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 789, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i2.to_i32(), -788);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_subtract_negative_number() {
        let mut computer = Computer::new();
        computer.r_i2 = Index::from_i32(1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::MINUS, 789, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i2.to_i32(), 790);
        assert!(!computer.overflow);
    }
    
    #[test]
    fn should_not_change_sign_if_result_is_zero() {
        let mut computer = Computer::new();
        computer.r_i2 = Index::from_i32(-1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::MINUS, 1, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i2.to_i32(), 0);
        assert_eq!(computer.r_i2.sign, Sign::MINUS);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_subtract_index_modified_value() {
        let mut computer = Computer::new();
        computer.r_i1 = Index::from_i32(1).unwrap();
        computer.r_i2 = Index::from_i32(8).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 3, 1, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i2.to_i32(), 6);
        assert!(!computer.overflow);
    }
}

mod dec3 {
    use crate::computer::*;
    use crate::data_types::*;

    const CODE: i32 = 51;
    const FIELD: i32 = 1;

    #[test]
    fn should_subtract_positive_number() {
        let mut computer = Computer::new();
        computer.r_i3 = Index::from_i32(1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 789, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i3.to_i32(), -788);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_subtract_negative_number() {
        let mut computer = Computer::new();
        computer.r_i3 = Index::from_i32(1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::MINUS, 789, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i3.to_i32(), 790);
        assert!(!computer.overflow);
    }
    
    #[test]
    fn should_not_change_sign_if_result_is_zero() {
        let mut computer = Computer::new();
        computer.r_i3 = Index::from_i32(-1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::MINUS, 1, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i3.to_i32(), 0);
        assert_eq!(computer.r_i3.sign, Sign::MINUS);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_subtract_index_modified_value() {
        let mut computer = Computer::new();
        computer.r_i3 = Index::from_i32(1).unwrap();
        computer.r_i2 = Index::from_i32(8).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 3, 2, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i3.to_i32(), 6);
        assert!(!computer.overflow);
    }
}

mod dec4 {
    use crate::computer::*;
    use crate::data_types::*;

    const CODE: i32 = 52;
    const FIELD: i32 = 1;

    #[test]
    fn should_subtract_positive_number() {
        let mut computer = Computer::new();
        computer.r_i4 = Index::from_i32(1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 789, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i4.to_i32(), -788);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_subtract_negative_number() {
        let mut computer = Computer::new();
        computer.r_i4 = Index::from_i32(1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::MINUS, 789, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i4.to_i32(), 790);
        assert!(!computer.overflow);
    }
    
    #[test]
    fn should_not_change_sign_if_result_is_zero() {
        let mut computer = Computer::new();
        computer.r_i4 = Index::from_i32(-1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::MINUS, 1, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i4.to_i32(), 0);
        assert_eq!(computer.r_i4.sign, Sign::MINUS);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_subtract_index_modified_value() {
        let mut computer = Computer::new();
        computer.r_i4 = Index::from_i32(1).unwrap();
        computer.r_i2 = Index::from_i32(8).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 3, 2, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i4.to_i32(), 6);
        assert!(!computer.overflow);
    }
}

mod dec5 {
    use crate::computer::*;
    use crate::data_types::*;

    const CODE: i32 = 53;
    const FIELD: i32 = 1;

    #[test]
    fn should_subtract_positive_number() {
        let mut computer = Computer::new();
        computer.r_i5 = Index::from_i32(1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 789, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i5.to_i32(), -788);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_subtract_negative_number() {
        let mut computer = Computer::new();
        computer.r_i5 = Index::from_i32(1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::MINUS, 789, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i5.to_i32(), 790);
        assert!(!computer.overflow);
    }
    
    #[test]
    fn should_not_change_sign_if_result_is_zero() {
        let mut computer = Computer::new();
        computer.r_i5 = Index::from_i32(-1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::MINUS, 1, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i5.to_i32(), 0);
        assert_eq!(computer.r_i5.sign, Sign::MINUS);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_subtract_index_modified_value() {
        let mut computer = Computer::new();
        computer.r_i5 = Index::from_i32(1).unwrap();
        computer.r_i2 = Index::from_i32(8).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 3, 2, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i5.to_i32(), 6);
        assert!(!computer.overflow);
    }
}

mod dec6 {
    use crate::computer::*;
    use crate::data_types::*;

    const CODE: i32 = 54;
    const FIELD: i32 = 1;

    #[test]
    fn should_subtract_positive_number() {
        let mut computer = Computer::new();
        computer.r_i6 = Index::from_i32(1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 789, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i6.to_i32(), -788);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_subtract_negative_number() {
        let mut computer = Computer::new();
        computer.r_i6 = Index::from_i32(1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::MINUS, 789, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i6.to_i32(), 790);
        assert!(!computer.overflow);
    }
    
    #[test]
    fn should_not_change_sign_if_result_is_zero() {
        let mut computer = Computer::new();
        computer.r_i6 = Index::from_i32(-1).unwrap();
        let instruction = Word::from_instruction_parts(Sign::MINUS, 1, 0, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i6.to_i32(), 0);
        assert_eq!(computer.r_i6.sign, Sign::MINUS);
        assert!(!computer.overflow);
    }

    #[test]
    fn should_subtract_index_modified_value() {
        let mut computer = Computer::new();
        computer.r_i6 = Index::from_i32(1).unwrap();
        computer.r_i2 = Index::from_i32(8).unwrap();
        let instruction = Word::from_instruction_parts(Sign::PLUS, 3, 2, FIELD, CODE).unwrap();
        computer.handle_instruction(instruction);
        assert_eq!(computer.r_i6.to_i32(), 6);
        assert!(!computer.overflow);
    }
}
