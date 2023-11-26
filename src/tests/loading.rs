mod lda_tests {
    use crate::{computer::*, data_types::*};

    #[test]
    fn should_load_value_from_memory() {
        let mut computer = Computer::new();
        let content = Word::from_i32(1234).unwrap();
        computer.memory.set(1, content).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 0, 5, 8).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_a.to_i32(), content.to_i32());
    }

    #[test]
    fn should_load_value_from_index_modified_address() {
        let mut computer = Computer::new();
        let content = Word::from_i32(2345).unwrap();
        computer.memory.set(101, content).unwrap();
        computer
            .memory
            .set(200, Word::from_i32(5432).unwrap())
            .unwrap();
        computer.r_i4 = Index::from_i32(-99).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 200, 4, 5, 8).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_a.to_i32(), content.to_i32());
    }

    #[test]
    fn should_load_value_without_changing_sign() {
        let mut computer = Computer::new();
        let content = Word::from_i32(-1).unwrap();
        computer.memory.set(5, content).unwrap();
        computer.r_a = Word::MAX;

        let instruction = Word::from_instruction_parts(Sign::PLUS, 5, 0, 13, 8).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_a.to_i32(), 1);
    }

    #[test]
    fn should_load_part_of_value() {
        let mut computer = Computer::new();
        let content = Word {
            sign: Sign::MINUS,
            bytes: (
                Byte::from_i32(1).unwrap(),
                Byte::from_i32(2).unwrap(),
                Byte::from_i32(3).unwrap(),
                Byte::from_i32(4).unwrap(),
                Byte::from_i32(5).unwrap(),
            ),
        };
        computer.memory.set(10, content).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 10, 0, 20, 8).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_a.to_i32(), 2 * 64_i32.pow(2) + 3 * 64 + 4);
    }
}

#[cfg(test)]
mod ldx_tests {
    use crate::{computer::*, data_types::*};

    #[test]
    fn should_load_value_from_memory() {
        let mut computer = Computer::new();
        let content = Word::from_i32(1234).unwrap();
        computer.memory.set(1, content).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 0, 5, 15).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_x.to_i32(), content.to_i32());
    }

    #[test]
    fn should_load_value_from_index_modified_address() {
        let mut computer = Computer::new();
        let content = Word::from_i32(2345).unwrap();
        computer.memory.set(101, content).unwrap();
        computer
            .memory
            .set(200, Word::from_i32(5432).unwrap())
            .unwrap();
        computer.r_i3 = Index::from_i32(-99).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 200, 3, 5, 15).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_x.to_i32(), content.to_i32());
    }

    #[test]
    fn should_load_value_without_changing_sign() {
        let mut computer = Computer::new();
        let content = Word::from_i32(-1).unwrap();
        computer.memory.set(5, content).unwrap();
        computer.r_x = Word::MAX;

        let instruction = Word::from_instruction_parts(Sign::PLUS, 5, 0, 13, 15).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_x.to_i32(), 1);
    }

    #[test]
    fn should_load_part_of_value() {
        let mut computer = Computer::new();
        let content = Word {
            sign: Sign::MINUS,
            bytes: (
                Byte::from_i32(1).unwrap(),
                Byte::from_i32(2).unwrap(),
                Byte::from_i32(3).unwrap(),
                Byte::from_i32(4).unwrap(),
                Byte::from_i32(5).unwrap(),
            ),
        };
        computer.memory.set(10, content).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 10, 0, 20, 15).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_x.to_i32(), 2 * 64_i32.pow(2) + 3 * 64 + 4);
    }
}

#[cfg(test)]
mod ld1_tests {
    use crate::{computer::*, data_types::*};

    #[test]
    fn should_load_value_from_memory() {
        let mut computer = Computer::new();
        let content = Word::from_i32(1234).unwrap();
        computer.memory.set(1, content).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 0, 5, 9).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_i1.to_i32(), content.to_i32());
    }

    #[test]
    fn should_load_value_from_index_modified_address() {
        let mut computer = Computer::new();
        let content = Word::from_i32(2345).unwrap();
        computer.memory.set(101, content).unwrap();
        computer
            .memory
            .set(200, Word::from_i32(5432).unwrap())
            .unwrap();
        computer.r_i2 = Index::from_i32(-99).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 200, 2, 5, 9).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_i1.to_i32(), content.to_i32());
    }

    #[test]
    fn should_load_value_without_changing_sign() {
        let mut computer = Computer::new();
        let content = Word::from_i32(-1).unwrap();
        computer.memory.set(5, content).unwrap();
        computer.r_i1 = Index::MAX;

        let instruction = Word::from_instruction_parts(Sign::PLUS, 5, 0, 13, 9).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_i1.to_i32(), 1);
    }

    #[test]
    fn should_load_part_of_value() {
        let mut computer = Computer::new();
        let content = Word {
            sign: Sign::MINUS,
            bytes: (
                Byte::from_i32(1).unwrap(),
                Byte::from_i32(2).unwrap(),
                Byte::from_i32(3).unwrap(),
                Byte::from_i32(4).unwrap(),
                Byte::from_i32(5).unwrap(),
            ),
        };
        computer.memory.set(10, content).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 10, 0, 28, 9).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_i1.to_i32(), 3 * 64 + 4);
    }
}

#[cfg(test)]
mod ld2_tests {
    use crate::{computer::*, data_types::*};

    #[test]
    fn should_load_value_from_memory() {
        let mut computer = Computer::new();
        let content = Word::from_i32(1234).unwrap();
        computer.memory.set(1, content).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 0, 5, 10).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_i2.to_i32(), content.to_i32());
    }

    #[test]
    fn should_load_value_from_index_modified_address() {
        let mut computer = Computer::new();
        let content = Word::from_i32(2345).unwrap();
        computer.memory.set(101, content).unwrap();
        computer
            .memory
            .set(200, Word::from_i32(5432).unwrap())
            .unwrap();
        computer.r_i3 = Index::from_i32(-99).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 200, 3, 5, 10).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_i2.to_i32(), content.to_i32());
    }

    #[test]
    fn should_load_value_without_changing_sign() {
        let mut computer = Computer::new();
        let content = Word::from_i32(-1).unwrap();
        computer.memory.set(5, content).unwrap();
        computer.r_i2 = Index::MAX;

        let instruction = Word::from_instruction_parts(Sign::PLUS, 5, 0, 13, 10).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_i2.to_i32(), 1);
    }

    #[test]
    fn should_load_part_of_value() {
        let mut computer = Computer::new();
        let content = Word {
            sign: Sign::MINUS,
            bytes: (
                Byte::from_i32(1).unwrap(),
                Byte::from_i32(2).unwrap(),
                Byte::from_i32(3).unwrap(),
                Byte::from_i32(4).unwrap(),
                Byte::from_i32(5).unwrap(),
            ),
        };
        computer.memory.set(10, content).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 10, 0, 28, 10).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_i2.to_i32(), 3 * 64 + 4);
    }
}

#[cfg(test)]
mod ld3_tests {
    use crate::{computer::*, data_types::*};

    #[test]
    fn should_load_value_from_memory() {
        let mut computer = Computer::new();
        let content = Word::from_i32(1234).unwrap();
        computer.memory.set(1, content).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 0, 5, 11).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_i3.to_i32(), content.to_i32());
    }

    #[test]
    fn should_load_value_from_index_modified_address() {
        let mut computer = Computer::new();
        let content = Word::from_i32(2345).unwrap();
        computer.memory.set(101, content).unwrap();
        computer
            .memory
            .set(200, Word::from_i32(5432).unwrap())
            .unwrap();
        computer.r_i4 = Index::from_i32(-99).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 200, 4, 5, 11).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_i3.to_i32(), content.to_i32());
    }

    #[test]
    fn should_load_value_without_changing_sign() {
        let mut computer = Computer::new();
        let content = Word::from_i32(-1).unwrap();
        computer.memory.set(5, content).unwrap();
        computer.r_i3 = Index::MAX;

        let instruction = Word::from_instruction_parts(Sign::PLUS, 5, 0, 13, 11).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_i3.to_i32(), 1);
    }

    #[test]
    fn should_load_part_of_value() {
        let mut computer = Computer::new();
        let content = Word {
            sign: Sign::MINUS,
            bytes: (
                Byte::from_i32(1).unwrap(),
                Byte::from_i32(2).unwrap(),
                Byte::from_i32(3).unwrap(),
                Byte::from_i32(4).unwrap(),
                Byte::from_i32(5).unwrap(),
            ),
        };
        computer.memory.set(10, content).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 10, 0, 28, 11).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_i3.to_i32(), 3 * 64 + 4);
    }
}

#[cfg(test)]
mod ld4_tests {
    use crate::{computer::*, data_types::*};

    #[test]
    fn should_load_value_from_memory() {
        let mut computer = Computer::new();
        let content = Word::from_i32(1234).unwrap();
        computer.memory.set(1, content).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 0, 5, 12).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_i4.to_i32(), content.to_i32());
    }

    #[test]
    fn should_load_value_from_index_modified_address() {
        let mut computer = Computer::new();
        let content = Word::from_i32(2345).unwrap();
        computer.memory.set(101, content).unwrap();
        computer
            .memory
            .set(200, Word::from_i32(5432).unwrap())
            .unwrap();
        computer.r_i5 = Index::from_i32(-99).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 200, 5, 5, 12).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_i4.to_i32(), content.to_i32());
    }

    #[test]
    fn should_load_value_without_changing_sign() {
        let mut computer = Computer::new();
        let content = Word::from_i32(-1).unwrap();
        computer.memory.set(5, content).unwrap();
        computer.r_i4 = Index::MAX;

        let instruction = Word::from_instruction_parts(Sign::PLUS, 5, 0, 13, 12).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_i4.to_i32(), 1);
    }

    #[test]
    fn should_load_part_of_value() {
        let mut computer = Computer::new();
        let content = Word {
            sign: Sign::MINUS,
            bytes: (
                Byte::from_i32(1).unwrap(),
                Byte::from_i32(2).unwrap(),
                Byte::from_i32(3).unwrap(),
                Byte::from_i32(4).unwrap(),
                Byte::from_i32(5).unwrap(),
            ),
        };
        computer.memory.set(10, content).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 10, 0, 28, 12).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_i4.to_i32(), 3 * 64 + 4);
    }
}

#[cfg(test)]
mod ld5_tests {
    use crate::{computer::*, data_types::*};

    #[test]
    fn should_load_value_from_memory() {
        let mut computer = Computer::new();
        let content = Word::from_i32(1234).unwrap();
        computer.memory.set(1, content).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 0, 5, 13).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_i5.to_i32(), content.to_i32());
    }

    #[test]
    fn should_load_value_from_index_modified_address() {
        let mut computer = Computer::new();
        let content = Word::from_i32(2345).unwrap();
        computer.memory.set(101, content).unwrap();
        computer
            .memory
            .set(200, Word::from_i32(5432).unwrap())
            .unwrap();
        computer.r_i6 = Index::from_i32(-99).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 200, 6, 5, 13).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_i5.to_i32(), content.to_i32());
    }

    #[test]
    fn should_load_value_without_changing_sign() {
        let mut computer = Computer::new();
        let content = Word::from_i32(-1).unwrap();
        computer.memory.set(5, content).unwrap();
        computer.r_i5 = Index::MAX;

        let instruction = Word::from_instruction_parts(Sign::PLUS, 5, 0, 13, 13).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_i5.to_i32(), 1);
    }

    #[test]
    fn should_load_part_of_value() {
        let mut computer = Computer::new();
        let content = Word {
            sign: Sign::MINUS,
            bytes: (
                Byte::from_i32(1).unwrap(),
                Byte::from_i32(2).unwrap(),
                Byte::from_i32(3).unwrap(),
                Byte::from_i32(4).unwrap(),
                Byte::from_i32(5).unwrap(),
            ),
        };
        computer.memory.set(10, content).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 10, 0, 28, 13).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_i5.to_i32(), 3 * 64 + 4);
    }
}

#[cfg(test)]
mod ld6_tests {
    use crate::{computer::*, data_types::*};

    #[test]
    fn should_load_value_from_memory() {
        let mut computer = Computer::new();
        let content = Word::from_i32(1234).unwrap();
        computer.memory.set(1, content).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 0, 5, 14).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_i6.to_i32(), content.to_i32());
    }

    #[test]
    fn should_load_value_from_index_modified_address() {
        let mut computer = Computer::new();
        let content = Word::from_i32(2345).unwrap();
        computer.memory.set(101, content).unwrap();
        computer
            .memory
            .set(200, Word::from_i32(5432).unwrap())
            .unwrap();
        computer.r_i1 = Index::from_i32(-99).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 200, 1, 5, 14).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_i6.to_i32(), content.to_i32());
    }

    #[test]
    fn should_load_value_without_changing_sign() {
        let mut computer = Computer::new();
        let content = Word::from_i32(-1).unwrap();
        computer.memory.set(5, content).unwrap();
        computer.r_i6 = Index::MAX;

        let instruction = Word::from_instruction_parts(Sign::PLUS, 5, 0, 13, 14).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_i6.to_i32(), 1);
    }

    #[test]
    fn should_load_part_of_value() {
        let mut computer = Computer::new();
        let content = Word {
            sign: Sign::MINUS,
            bytes: (
                Byte::from_i32(1).unwrap(),
                Byte::from_i32(2).unwrap(),
                Byte::from_i32(3).unwrap(),
                Byte::from_i32(4).unwrap(),
                Byte::from_i32(5).unwrap(),
            ),
        };
        computer.memory.set(10, content).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 10, 0, 28, 14).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_i6.to_i32(), 3 * 64 + 4);
    }
}

#[cfg(test)]
mod ldan_tests {
    use crate::{computer::*, data_types::*};

    #[test]
    fn should_load_value_from_memory() {
        let mut computer = Computer::new();
        let content = Word::from_i32(1234).unwrap();
        computer.memory.set(1, content).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 0, 5, 16).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_a.to_i32(), -1234);
    }

    #[test]
    fn should_load_value_from_index_modified_address() {
        let mut computer = Computer::new();
        let content = Word::from_i32(-2345).unwrap();
        computer.memory.set(101, content).unwrap();
        computer
            .memory
            .set(200, Word::from_i32(5432).unwrap())
            .unwrap();
        computer.r_i4 = Index::from_i32(-99).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 200, 4, 5, 16).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_a.to_i32(), content.to_i32() * (-1));
    }

    #[test]
    fn should_load_value_and_change_sign() {
        let mut computer = Computer::new();
        let content = Word::from_i32(-1).unwrap();
        computer.memory.set(5, content).unwrap();
        computer.r_a = Word::MAX;

        let instruction = Word::from_instruction_parts(Sign::PLUS, 5, 0, 13, 16).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_a.to_i32(), -1);
    }

    #[test]
    fn should_load_part_of_value() {
        let mut computer = Computer::new();
        let content = Word {
            sign: Sign::MINUS,
            bytes: (
                Byte::from_i32(1).unwrap(),
                Byte::from_i32(2).unwrap(),
                Byte::from_i32(3).unwrap(),
                Byte::from_i32(4).unwrap(),
                Byte::from_i32(5).unwrap(),
            ),
        };
        computer.memory.set(10, content).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 10, 0, 20, 16).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_a.to_i32(), (-1) * (2 * 64_i32.pow(2) + 3 * 64 + 4));
    }
}

#[cfg(test)]
mod ldxn_tests {
    use crate::{computer::*, data_types::*};

    #[test]
    fn should_load_value_from_memory() {
        let mut computer = Computer::new();
        let content = Word::from_i32(1234).unwrap();
        computer.memory.set(1, content).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 0, 5, 23).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_x.to_i32(), -1234);
    }
}

#[cfg(test)]
mod ld1n_tests {
    use crate::{computer::*, data_types::*};

    #[test]
    fn should_load_value_from_memory() {
        let mut computer = Computer::new();
        let content = Word::from_i32(1234).unwrap();
        computer.memory.set(1, content).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 0, 5, 17).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_i1.to_i32(), -1234);
    }
}

#[cfg(test)]
mod ld2n_tests {
    use crate::{computer::*, data_types::*};

    #[test]
    fn should_load_value_from_memory() {
        let mut computer = Computer::new();
        let content = Word::from_i32(1234).unwrap();
        computer.memory.set(1, content).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 0, 5, 18).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_i2.to_i32(), -1234);
    }
}

#[cfg(test)]
mod ld3n_tests {
    use crate::{computer::*, data_types::*};

    #[test]
    fn should_load_value_from_memory() {
        let mut computer = Computer::new();
        let content = Word::from_i32(1234).unwrap();
        computer.memory.set(1, content).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 0, 5, 19).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_i3.to_i32(), -1234);
    }
}

#[cfg(test)]
mod ld4n_tests {
    use crate::{computer::*, data_types::*};

    #[test]
    fn should_load_value_from_memory() {
        let mut computer = Computer::new();
        let content = Word::from_i32(1234).unwrap();
        computer.memory.set(1, content).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 0, 5, 20).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_i4.to_i32(), -1234);
    }
}

#[cfg(test)]
mod ld5n_tests {
    use crate::{computer::*, data_types::*};

    #[test]
    fn should_load_value_from_memory() {
        let mut computer = Computer::new();
        let content = Word::from_i32(1234).unwrap();
        computer.memory.set(1, content).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 0, 5, 21).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_i5.to_i32(), -1234);
    }
}

#[cfg(test)]
mod ld6n_tests {
    use crate::{computer::*, data_types::*};

    #[test]
    fn should_load_value_from_memory() {
        let mut computer = Computer::new();
        let content = Word::from_i32(1234).unwrap();
        computer.memory.set(1, content).unwrap();

        let instruction = Word::from_instruction_parts(Sign::PLUS, 1, 0, 5, 22).unwrap();
        computer.handle_instruction(instruction);

        assert_eq!(computer.r_i6.to_i32(), -1234);
    }
}
