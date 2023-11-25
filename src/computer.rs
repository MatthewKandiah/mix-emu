use crate::data_types::{Byte, FieldSpecification, Index, JumpAddress, Sign, Word};
use crate::memory::Memory;

pub enum ComparisonIndicatorState {
    EQUAL,
    GREATER,
    LESS,
}

pub struct Computer {
    current_instruction_address: i32,
    r_a: Word,
    r_x: Word,
    r_i1: Index,
    r_i2: Index,
    r_i3: Index,
    r_i4: Index,
    r_i5: Index,
    r_i6: Index,
    r_j: JumpAddress,
    overflow: bool,
    comparison_indicator: Option<ComparisonIndicatorState>,
    memory: Memory,
}

impl Computer {
    pub fn new() -> Self {
        Self {
            current_instruction_address: 0,
            r_a: Word::ZERO,
            r_x: Word::ZERO,
            r_i1: Index::ZERO,
            r_i2: Index::ZERO,
            r_i3: Index::ZERO,
            r_i4: Index::ZERO,
            r_i5: Index::ZERO,
            r_i6: Index::ZERO,
            r_j: JumpAddress::ZERO,
            overflow: false,
            comparison_indicator: None,
            memory: Memory::ZERO,
        }
    }

    pub fn handle_next_instruction(&mut self) {
        let current_instruction = self.memory.get(self.current_instruction_address).unwrap();
        self.current_instruction_address += 1;
        self.handle_instruction(current_instruction);
    }

    fn handle_instruction(&mut self, instruction: Word) {
        match instruction.code() {
            0 => (),
            1 => self.add(instruction),
            2 => self.sub(instruction),
            3 => self.mul(instruction),
            4 => self.div(instruction),
            5 => self.handle_5(instruction),
            6 => self.handle_6(instruction),
            7 => self.mov(instruction),
            8 => self.lda(instruction),
            9 => self.ld1(instruction),
            10 => self.ld2(instruction),
            11 => self.ld3(instruction),
            12 => self.ld4(instruction),
            13 => self.ld5(instruction),
            14 => self.ld6(instruction),
            15 => self.ldx(instruction),
            16 => self.ldan(instruction),
            17 => self.ld1n(instruction),
            18 => self.ld2n(instruction),
            19 => self.ld3n(instruction),
            20 => self.ld4n(instruction),
            21 => self.ld5n(instruction),
            22 => self.ld6n(instruction),
            23 => self.ldxn(instruction),
            24 => self.sta(instruction),
            25 => self.st1(instruction),
            26 => self.st2(instruction),
            27 => self.st3(instruction),
            28 => self.st4(instruction),
            29 => self.st5(instruction),
            30 => self.st6(instruction),
            31 => self.stx(instruction),
            32 => self.stj(instruction),
            33 => self.stz(instruction),
            34 => self.jbus(instruction),
            35 => self.ioc(instruction),
            36 => self.input(instruction),
            37 => self.output(instruction),
            38 => self.jred(instruction),
            39 => self.handle_39(instruction),
            40 => self.handle_40(instruction),
            41 => self.handle_41(instruction),
            42 => self.handle_42(instruction),
            43 => self.handle_43(instruction),
            44 => self.handle_44(instruction),
            45 => self.handle_45(instruction),
            46 => self.handle_46(instruction),
            47 => self.handle_47(instruction),
            48 => self.handle_48(instruction),
            49 => self.handle_49(instruction),
            50 => self.handle_50(instruction),
            51 => self.handle_51(instruction),
            52 => self.handle_52(instruction),
            53 => self.handle_53(instruction),
            54 => self.handle_54(instruction),
            55 => self.handle_55(instruction),
            56 => self.cmpa(instruction),
            57 => self.cmp1(instruction),
            58 => self.cmp2(instruction),
            59 => self.cmp3(instruction),
            60 => self.cmp4(instruction),
            61 => self.cmp5(instruction),
            62 => self.cmp6(instruction),
            63 => self.cmpx(instruction),
            _ => panic!("Invalid instruction code"),
        }
    }

    fn add(&mut self, instruction: Word) {}

    fn sub(&mut self, instruction: Word) {}

    fn mul(&mut self, instruction: Word) {}

    fn div(&mut self, instruction: Word) {}

    fn handle_5(&mut self, instruction: Word) {}

    fn handle_6(&mut self, instruction: Word) {}

    fn mov(&mut self, instruction: Word) {}

    fn modified_address(&self, instruction: Word) -> i32 {
        let index_modifier = match instruction.index() {
            0 => 0,
            1 => self.r_i1.to_i32(),
            2 => self.r_i2.to_i32(),
            3 => self.r_i3.to_i32(),
            4 => self.r_i4.to_i32(),
            5 => self.r_i5.to_i32(),
            6 => self.r_i6.to_i32(),
            _ => panic!("Invalid index"),
        };
        instruction.address() + index_modifier
    }

    fn sign_to_load(
        field_specifier: &FieldSpecification,
        contents: Word,
        original_value: Sign,
    ) -> Sign {
        match field_specifier.inclusive_range().contains(&0) {
            true => contents.sign,
            false => original_value,
        }
    }

    fn bytes_to_load_word(
        field_specifier: &FieldSpecification,
        contents: Word,
    ) -> (Byte, Byte, Byte, Byte, Byte) {
        let mut bytes_to_load = Vec::<Byte>::new();
        let prepend_zero_byte = |bytes_to_copy: &mut Vec<Byte>| {
            bytes_to_copy.reverse();
            bytes_to_copy.push(Byte::ZERO);
            bytes_to_copy.reverse();
        };
        match field_specifier.inclusive_range().contains(&1) {
            true => bytes_to_load.push(contents.bytes.0),
            false => prepend_zero_byte(&mut bytes_to_load),
        };
        match field_specifier.inclusive_range().contains(&2) {
            true => bytes_to_load.push(contents.bytes.1),
            false => prepend_zero_byte(&mut bytes_to_load),
        };
        match field_specifier.inclusive_range().contains(&3) {
            true => bytes_to_load.push(contents.bytes.2),
            false => prepend_zero_byte(&mut bytes_to_load),
        };
        match field_specifier.inclusive_range().contains(&4) {
            true => bytes_to_load.push(contents.bytes.3),
            false => prepend_zero_byte(&mut bytes_to_load),
        };
        match field_specifier.inclusive_range().contains(&5) {
            true => bytes_to_load.push(contents.bytes.4),
            false => prepend_zero_byte(&mut bytes_to_load),
        };
        (
            bytes_to_load[0],
            bytes_to_load[1],
            bytes_to_load[2],
            bytes_to_load[3],
            bytes_to_load[4],
        )
    }

    fn bytes_to_load_index(field_specifier: &FieldSpecification, contents: Word) -> (Byte, Byte) {
        let bytes_for_word = Self::bytes_to_load_word(field_specifier, contents);
        if bytes_for_word.0 != Byte::ZERO
            || bytes_for_word.1 != Byte::ZERO
            || bytes_for_word.2 != Byte::ZERO
        {
            panic!("Attempting to load too many bytes into index register");
        }
        (bytes_for_word.3, bytes_for_word.4)
    }

    fn field_specifier_and_contents(&self, instruction: Word) -> (FieldSpecification, Word) {
        let address = self.modified_address(instruction);
        let contents = self.memory.get(address).unwrap();
        let field_specifier = instruction.field();
        if !field_specifier.is_valid() {
            panic!("illegal field specifier: {:?}", field_specifier);
        }
        (field_specifier, contents)
    }

    fn lda(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.r_a = Word {
            sign: Self::sign_to_load(&field_specifier, contents, self.r_a.sign),
            bytes: Self::bytes_to_load_word(&field_specifier, contents),
        };
    }

    fn ld1(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.r_i1 = Index {
            sign: Self::sign_to_load(&field_specifier, contents, self.r_i1.sign),
            bytes: Self::bytes_to_load_index(&field_specifier, contents),
        };
    }

    fn ld2(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.r_i2 = Index {
            sign: Self::sign_to_load(&field_specifier, contents, self.r_i2.sign),
            bytes: Self::bytes_to_load_index(&field_specifier, contents),
        };
    }

    fn ld3(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.r_i3 = Index {
            sign: Self::sign_to_load(&field_specifier, contents, self.r_i3.sign),
            bytes: Self::bytes_to_load_index(&field_specifier, contents),
        };
    }

    fn ld4(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.r_i4 = Index {
            sign: Self::sign_to_load(&field_specifier, contents, self.r_i4.sign),
            bytes: Self::bytes_to_load_index(&field_specifier, contents),
        };
    }

    fn ld5(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.r_i5 = Index {
            sign: Self::sign_to_load(&field_specifier, contents, self.r_i5.sign),
            bytes: Self::bytes_to_load_index(&field_specifier, contents),
        };
    }

    fn ld6(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.r_i6 = Index {
            sign: Self::sign_to_load(&field_specifier, contents, self.r_i6.sign),
            bytes: Self::bytes_to_load_index(&field_specifier, contents),
        };
    }

    fn ldx(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.r_x = Word {
            sign: Self::sign_to_load(&field_specifier, contents, self.r_x.sign),
            bytes: Self::bytes_to_load_word(&field_specifier, contents),
        };
    }

    fn ldan(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.r_a = Word {
            sign: Self::sign_to_load(&field_specifier, contents, self.r_a.sign).opposite(),
            bytes: Self::bytes_to_load_word(&field_specifier, contents),
        };
    }

    fn ld1n(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.r_i1 = Index {
            sign: Self::sign_to_load(&field_specifier, contents, self.r_i1.sign).opposite(),
            bytes: Self::bytes_to_load_index(&field_specifier, contents),
        };
    }

    fn ld2n(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.r_i2 = Index {
            sign: Self::sign_to_load(&field_specifier, contents, self.r_i2.sign).opposite(),
            bytes: Self::bytes_to_load_index(&field_specifier, contents),
        };
    }

    fn ld3n(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.r_i3 = Index {
            sign: Self::sign_to_load(&field_specifier, contents, self.r_i3.sign).opposite(),
            bytes: Self::bytes_to_load_index(&field_specifier, contents),
        };
    }

    fn ld4n(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.r_i4 = Index {
            sign: Self::sign_to_load(&field_specifier, contents, self.r_i4.sign).opposite(),
            bytes: Self::bytes_to_load_index(&field_specifier, contents),
        };
    }

    fn ld5n(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.r_i5 = Index {
            sign: Self::sign_to_load(&field_specifier, contents, self.r_i5.sign).opposite(),
            bytes: Self::bytes_to_load_index(&field_specifier, contents),
        };
    }

    fn ld6n(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.r_i6 = Index {
            sign: Self::sign_to_load(&field_specifier, contents, self.r_i6.sign).opposite(),
            bytes: Self::bytes_to_load_index(&field_specifier, contents),
        };
    }

    fn ldxn(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.r_x = Word {
            sign: Self::sign_to_load(&field_specifier, contents, self.r_x.sign).opposite(),
            bytes: Self::bytes_to_load_word(&field_specifier, contents),
        };
    }

    fn sta(&mut self, instruction: Word) {}

    fn st1(&mut self, instruction: Word) {}

    fn st2(&mut self, instruction: Word) {}

    fn st3(&mut self, instruction: Word) {}

    fn st4(&mut self, instruction: Word) {}

    fn st5(&mut self, instruction: Word) {}

    fn st6(&mut self, instruction: Word) {}

    fn stx(&mut self, instruction: Word) {}

    fn stj(&mut self, instruction: Word) {}

    fn stz(&mut self, instruction: Word) {}

    fn jbus(&mut self, instruction: Word) {}

    fn ioc(&mut self, instruction: Word) {}

    fn input(&mut self, instruction: Word) {}

    fn output(&mut self, instruction: Word) {}

    fn jred(&mut self, instruction: Word) {}

    fn handle_39(&mut self, instruction: Word) {}

    fn handle_40(&mut self, instruction: Word) {}

    fn handle_41(&mut self, instruction: Word) {}

    fn handle_42(&mut self, instruction: Word) {}

    fn handle_43(&mut self, instruction: Word) {}

    fn handle_44(&mut self, instruction: Word) {}

    fn handle_45(&mut self, instruction: Word) {}

    fn handle_46(&mut self, instruction: Word) {}

    fn handle_47(&mut self, instruction: Word) {}

    fn handle_48(&mut self, instruction: Word) {}

    fn handle_49(&mut self, instruction: Word) {}

    fn handle_50(&mut self, instruction: Word) {}

    fn handle_51(&mut self, instruction: Word) {}

    fn handle_52(&mut self, instruction: Word) {}

    fn handle_53(&mut self, instruction: Word) {}

    fn handle_54(&mut self, instruction: Word) {}

    fn handle_55(&mut self, instruction: Word) {}

    fn cmpa(&mut self, instruction: Word) {}

    fn cmp1(&mut self, instruction: Word) {}

    fn cmp2(&mut self, instruction: Word) {}

    fn cmp3(&mut self, instruction: Word) {}

    fn cmp4(&mut self, instruction: Word) {}

    fn cmp5(&mut self, instruction: Word) {}

    fn cmp6(&mut self, instruction: Word) {}

    fn cmpx(&mut self, instruction: Word) {}
}

#[cfg(test)]
mod lda_tests {
    use crate::{
        computer::Computer,
        data_types::{Byte, Index, Sign, Word},
    };

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
    use crate::{
        computer::Computer,
        data_types::{Byte, Index, Sign, Word},
    };

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
    use crate::{
        computer::Computer,
        data_types::{Byte, Index, Sign, Word},
    };

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
    use crate::{
        computer::Computer,
        data_types::{Byte, Index, Sign, Word},
    };

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
    use crate::{
        computer::Computer,
        data_types::{Byte, Index, Sign, Word},
    };

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
    use crate::{
        computer::Computer,
        data_types::{Byte, Index, Sign, Word},
    };

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
    use crate::{
        computer::Computer,
        data_types::{Byte, Index, Sign, Word},
    };

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
    use crate::{
        computer::Computer,
        data_types::{Byte, Index, Sign, Word},
    };

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
    use crate::{
        computer::Computer,
        data_types::{Byte, Index, Sign, Word},
    };

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
    use crate::{
        computer::Computer,
        data_types::{Sign, Word},
    };

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
    use crate::{
        computer::Computer,
        data_types::{Sign, Word},
    };

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
    use crate::{
        computer::Computer,
        data_types::{Sign, Word},
    };

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
    use crate::{
        computer::Computer,
        data_types::{Sign, Word},
    };

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
    use crate::{
        computer::Computer,
        data_types::{Sign, Word},
    };

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
    use crate::{
        computer::Computer,
        data_types::{Sign, Word},
    };

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
    use crate::{
        computer::Computer,
        data_types::{Sign, Word},
    };

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
