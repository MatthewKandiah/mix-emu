use crate::data_types::{Byte, FieldSpecification, Index, JumpAddress, Sign, Word};
use crate::memory::Memory;

pub enum ComparisonIndicatorState {
    EQUAL,
    GREATER,
    LESS,
}

pub struct Computer {
    pub current_instruction_address: i32,
    pub r_a: Word,
    pub r_x: Word,
    pub r_i1: Index,
    pub r_i2: Index,
    pub r_i3: Index,
    pub r_i4: Index,
    pub r_i5: Index,
    pub r_i6: Index,
    pub r_j: JumpAddress,
    pub overflow: bool,
    pub comparison_indicator: Option<ComparisonIndicatorState>,
    pub memory: Memory,
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

    // TODO - add tests for this!
    pub fn handle_next_instruction(&mut self) {
        let current_instruction = self.memory.get(self.current_instruction_address).unwrap();
        self.current_instruction_address += 1;
        self.handle_instruction(current_instruction);
    }

    pub fn handle_instruction(&mut self, instruction: Word) {
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

    fn sign_to_load_or_store(
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
        match field_specifier.inclusive_range().contains(&1) {
            true => bytes_to_load.push(contents.bytes.0),
            false => bytes_to_load.insert(0, Byte::ZERO),
        };
        match field_specifier.inclusive_range().contains(&2) {
            true => bytes_to_load.push(contents.bytes.1),
            false => bytes_to_load.insert(0, Byte::ZERO),
        };
        match field_specifier.inclusive_range().contains(&3) {
            true => bytes_to_load.push(contents.bytes.2),
            false => bytes_to_load.insert(0, Byte::ZERO),
        };
        match field_specifier.inclusive_range().contains(&4) {
            true => bytes_to_load.push(contents.bytes.3),
            false => bytes_to_load.insert(0, Byte::ZERO),
        };
        match field_specifier.inclusive_range().contains(&5) {
            true => bytes_to_load.push(contents.bytes.4),
            false => bytes_to_load.insert(0, Byte::ZERO),
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

    fn get_v(&self, instruction: Word) -> i32 {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        let needed_bytes = Self::bytes_to_load_word(&field_specifier, contents);
        contents.sign.value()
            * (needed_bytes.0.to_i32() * 64_i32.pow(4)
                + needed_bytes.1.to_i32() * 64_i32.pow(3)
                + needed_bytes.2.to_i32() * 64_i32.pow(2)
                + needed_bytes.3.to_i32() * 64
                + needed_bytes.4.to_i32())
    }

    fn add(&mut self, instruction: Word) {
        let result = self.r_a.to_i32() + self.get_v(instruction);
        if result == 0 {
            self.r_a.bytes = (Byte::ZERO, Byte::ZERO, Byte::ZERO, Byte::ZERO, Byte::ZERO);
            return;
        }
        if result / 64_i32.pow(5) != 0 {
            self.overflow = true;
        }
        self.r_a = Word::from_i32(result % 64_i32.pow(5)).unwrap();
    }

    fn sub(&mut self, instruction: Word) {
        let result = self.r_a.to_i32() - self.get_v(instruction);
        if result == 0 {
            self.r_a.bytes = (Byte::ZERO, Byte::ZERO, Byte::ZERO, Byte::ZERO, Byte::ZERO);
            return;
        }
        if result / 64_i32.pow(5) != 0 {
            self.overflow = true;
        }
        self.r_a = Word::from_i32(result % 64_i32.pow(5)).unwrap();
    }

    fn mul(&mut self, instruction: Word) {
        // Word::MAX * Word::MAX too large to store in i32
        let result: i64 = <i32 as Into<i64>>::into(self.r_a.to_i32())
            * <i32 as Into<i64>>::into(self.get_v(instruction));
        let (_, contents) = self.field_specifier_and_contents(instruction);
        let result_sign = match self.r_a.sign == contents.sign {
            true => Sign::PLUS,
            false => Sign::MINUS,
        };
        let x_value = result.abs() % 64_i64.pow(5);
        let full_a_value = result.abs() / 64_i64.pow(5);
        let a_value = full_a_value % 64_i64.pow(5);
        // multiplication cannot overflow, consider the largest possible absolute value
        // (64^5-1)*(64^5-1) < 64^10-1
        // so result can always fit in 10 bytes!
        self.r_a = Word::from_i32(a_value.try_into().unwrap())
            .unwrap()
            .with_sign(result_sign);
        self.r_x = Word::from_i32(x_value.try_into().unwrap())
            .unwrap()
            .with_sign(result_sign);
    }

    fn div(&mut self, instruction: Word) {
        let numerator: i64 = <i32 as Into<i64>>::into(self.r_a.to_i32()) * 64_i64.pow(5)
            + <i32 as Into<i64>>::into(self.r_x.to_i32());
        let v = self.get_v(instruction);
        if v == 0 {
            self.overflow = true;
            return;
        }

        let x_sign = self.r_a.sign;
        let (_, contents) = self.field_specifier_and_contents(instruction);
        let a_sign = match self.r_a.sign == contents.sign {
            true => Sign::PLUS,
            false => Sign::MINUS,
        };

        let a_result_value: i32 = match (numerator / <i32 as Into<i64>>::into(v)).try_into() {
            Ok(x) => x,
            Err(_) => {
                self.overflow = true;
                return;
            }
        };
        let a_result_word = match Word::from_i32(a_result_value) {
            Ok(x) => x,
            Err(_) => {
                self.overflow = true;
                return;
            }
        }
        .with_sign(a_sign);

        let x_result_value: i32 = match (numerator % <i32 as Into<i64>>::into(v)).try_into() {
            Ok(x) => x,
            Err(_) => {
                self.overflow = true;
                return;
            }
        };
        let x_result_word = match Word::from_i32(x_result_value) {
            Ok(x) => x,
            Err(_) => {
                self.overflow = true;
                return;
            }
        }
        .with_sign(x_sign);

        self.r_x = x_result_word;
        self.r_a = a_result_word;
    }

    fn handle_5(&mut self, instruction: Word) {}

    fn handle_6(&mut self, instruction: Word) {}

    fn mov(&mut self, instruction: Word) {}

    fn lda(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.r_a = Word {
            sign: Self::sign_to_load_or_store(&field_specifier, contents, self.r_a.sign),
            bytes: Self::bytes_to_load_word(&field_specifier, contents),
        };
    }

    fn ld1(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.r_i1 = Index {
            sign: Self::sign_to_load_or_store(&field_specifier, contents, self.r_i1.sign),
            bytes: Self::bytes_to_load_index(&field_specifier, contents),
        };
    }

    fn ld2(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.r_i2 = Index {
            sign: Self::sign_to_load_or_store(&field_specifier, contents, self.r_i2.sign),
            bytes: Self::bytes_to_load_index(&field_specifier, contents),
        };
    }

    fn ld3(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.r_i3 = Index {
            sign: Self::sign_to_load_or_store(&field_specifier, contents, self.r_i3.sign),
            bytes: Self::bytes_to_load_index(&field_specifier, contents),
        };
    }

    fn ld4(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.r_i4 = Index {
            sign: Self::sign_to_load_or_store(&field_specifier, contents, self.r_i4.sign),
            bytes: Self::bytes_to_load_index(&field_specifier, contents),
        };
    }

    fn ld5(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.r_i5 = Index {
            sign: Self::sign_to_load_or_store(&field_specifier, contents, self.r_i5.sign),
            bytes: Self::bytes_to_load_index(&field_specifier, contents),
        };
    }

    fn ld6(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.r_i6 = Index {
            sign: Self::sign_to_load_or_store(&field_specifier, contents, self.r_i6.sign),
            bytes: Self::bytes_to_load_index(&field_specifier, contents),
        };
    }

    fn ldx(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.r_x = Word {
            sign: Self::sign_to_load_or_store(&field_specifier, contents, self.r_x.sign),
            bytes: Self::bytes_to_load_word(&field_specifier, contents),
        };
    }

    fn ldan(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.r_a = Word {
            sign: Self::sign_to_load_or_store(&field_specifier, contents, self.r_a.sign).opposite(),
            bytes: Self::bytes_to_load_word(&field_specifier, contents),
        };
    }

    fn ld1n(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.r_i1 = Index {
            sign: Self::sign_to_load_or_store(&field_specifier, contents, self.r_i1.sign)
                .opposite(),
            bytes: Self::bytes_to_load_index(&field_specifier, contents),
        };
    }

    fn ld2n(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.r_i2 = Index {
            sign: Self::sign_to_load_or_store(&field_specifier, contents, self.r_i2.sign)
                .opposite(),
            bytes: Self::bytes_to_load_index(&field_specifier, contents),
        };
    }

    fn ld3n(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.r_i3 = Index {
            sign: Self::sign_to_load_or_store(&field_specifier, contents, self.r_i3.sign)
                .opposite(),
            bytes: Self::bytes_to_load_index(&field_specifier, contents),
        };
    }

    fn ld4n(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.r_i4 = Index {
            sign: Self::sign_to_load_or_store(&field_specifier, contents, self.r_i4.sign)
                .opposite(),
            bytes: Self::bytes_to_load_index(&field_specifier, contents),
        };
    }

    fn ld5n(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.r_i5 = Index {
            sign: Self::sign_to_load_or_store(&field_specifier, contents, self.r_i5.sign)
                .opposite(),
            bytes: Self::bytes_to_load_index(&field_specifier, contents),
        };
    }

    fn ld6n(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.r_i6 = Index {
            sign: Self::sign_to_load_or_store(&field_specifier, contents, self.r_i6.sign)
                .opposite(),
            bytes: Self::bytes_to_load_index(&field_specifier, contents),
        };
    }

    fn ldxn(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.r_x = Word {
            sign: Self::sign_to_load_or_store(&field_specifier, contents, self.r_x.sign).opposite(),
            bytes: Self::bytes_to_load_word(&field_specifier, contents),
        };
    }

    fn bytes_to_store(
        memory_contents: Word,
        register_contents: Word,
        field_specifier: &FieldSpecification,
    ) -> (Byte, Byte, Byte, Byte, Byte) {
        let mut bytes_to_store = memory_contents.bytes;
        let mut number_of_bytes_altered = 0;
        let get_next_byte =
            |number_of_bytes_altered: i32, bytes: (Byte, Byte, Byte, Byte, Byte)| -> Byte {
                match number_of_bytes_altered {
                    0 => bytes.4,
                    1 => bytes.3,
                    2 => bytes.2,
                    3 => bytes.1,
                    4 => bytes.0,
                    _ => panic!("invalid byte access"),
                }
            };
        let mut next_byte = get_next_byte(number_of_bytes_altered, register_contents.bytes);

        if field_specifier.inclusive_range().contains(&5) {
            bytes_to_store.4 = next_byte;
            number_of_bytes_altered += 1;
            next_byte = get_next_byte(number_of_bytes_altered, register_contents.bytes);
        }
        if field_specifier.inclusive_range().contains(&4) {
            bytes_to_store.3 = next_byte;
            number_of_bytes_altered += 1;
            next_byte = get_next_byte(number_of_bytes_altered, register_contents.bytes);
        }
        if field_specifier.inclusive_range().contains(&3) {
            bytes_to_store.2 = next_byte;
            number_of_bytes_altered += 1;
            next_byte = get_next_byte(number_of_bytes_altered, register_contents.bytes);
        }
        if field_specifier.inclusive_range().contains(&2) {
            bytes_to_store.1 = next_byte;
            number_of_bytes_altered += 1;
            next_byte = get_next_byte(number_of_bytes_altered, register_contents.bytes);
        }
        if field_specifier.inclusive_range().contains(&1) {
            bytes_to_store.0 = next_byte;
        }
        bytes_to_store
    }

    fn sta(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        let original_sign = self
            .memory
            .get(self.modified_address(instruction))
            .unwrap()
            .sign;

        self.memory
            .set(
                self.modified_address(instruction),
                Word {
                    sign: Self::sign_to_load_or_store(&field_specifier, self.r_a, original_sign),
                    bytes: Self::bytes_to_store(contents, self.r_a, &field_specifier),
                },
            )
            .unwrap();
    }

    fn st1(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        let original_sign = self
            .memory
            .get(self.modified_address(instruction))
            .unwrap()
            .sign;
        let word_from_index = Word::from_i32(self.r_i1.to_i32()).unwrap();

        self.memory
            .set(
                self.modified_address(instruction),
                Word {
                    sign: Self::sign_to_load_or_store(
                        &field_specifier,
                        word_from_index,
                        original_sign,
                    ),
                    bytes: Self::bytes_to_store(contents, word_from_index, &field_specifier),
                },
            )
            .unwrap();
    }

    fn st2(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        let original_sign = self
            .memory
            .get(self.modified_address(instruction))
            .unwrap()
            .sign;
        let word_from_index = Word::from_i32(self.r_i2.to_i32()).unwrap();

        self.memory
            .set(
                self.modified_address(instruction),
                Word {
                    sign: Self::sign_to_load_or_store(
                        &field_specifier,
                        word_from_index,
                        original_sign,
                    ),
                    bytes: Self::bytes_to_store(contents, word_from_index, &field_specifier),
                },
            )
            .unwrap();
    }

    fn st3(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        let original_sign = self
            .memory
            .get(self.modified_address(instruction))
            .unwrap()
            .sign;
        let word_from_index = Word::from_i32(self.r_i3.to_i32()).unwrap();

        self.memory
            .set(
                self.modified_address(instruction),
                Word {
                    sign: Self::sign_to_load_or_store(
                        &field_specifier,
                        word_from_index,
                        original_sign,
                    ),
                    bytes: Self::bytes_to_store(contents, word_from_index, &field_specifier),
                },
            )
            .unwrap();
    }

    fn st4(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        let original_sign = self
            .memory
            .get(self.modified_address(instruction))
            .unwrap()
            .sign;
        let word_from_index = Word::from_i32(self.r_i4.to_i32()).unwrap();

        self.memory
            .set(
                self.modified_address(instruction),
                Word {
                    sign: Self::sign_to_load_or_store(
                        &field_specifier,
                        word_from_index,
                        original_sign,
                    ),
                    bytes: Self::bytes_to_store(contents, word_from_index, &field_specifier),
                },
            )
            .unwrap();
    }

    fn st5(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        let original_sign = self
            .memory
            .get(self.modified_address(instruction))
            .unwrap()
            .sign;
        let word_from_index = Word::from_i32(self.r_i5.to_i32()).unwrap();

        self.memory
            .set(
                self.modified_address(instruction),
                Word {
                    sign: Self::sign_to_load_or_store(
                        &field_specifier,
                        word_from_index,
                        original_sign,
                    ),
                    bytes: Self::bytes_to_store(contents, word_from_index, &field_specifier),
                },
            )
            .unwrap();
    }

    fn st6(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        let original_sign = self
            .memory
            .get(self.modified_address(instruction))
            .unwrap()
            .sign;
        let word_from_index = Word::from_i32(self.r_i6.to_i32()).unwrap();

        self.memory
            .set(
                self.modified_address(instruction),
                Word {
                    sign: Self::sign_to_load_or_store(
                        &field_specifier,
                        word_from_index,
                        original_sign,
                    ),
                    bytes: Self::bytes_to_store(contents, word_from_index, &field_specifier),
                },
            )
            .unwrap();
    }

    fn stx(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        let original_sign = self
            .memory
            .get(self.modified_address(instruction))
            .unwrap()
            .sign;

        self.memory
            .set(
                self.modified_address(instruction),
                Word {
                    sign: Self::sign_to_load_or_store(&field_specifier, self.r_x, original_sign),
                    bytes: Self::bytes_to_store(contents, self.r_x, &field_specifier),
                },
            )
            .unwrap();
    }

    fn stj(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        let original_sign = self
            .memory
            .get(self.modified_address(instruction))
            .unwrap()
            .sign;
        let word_from_jump_address = Word::from_i32(self.r_j.to_i32()).unwrap();

        self.memory
            .set(
                self.modified_address(instruction),
                Word {
                    sign: Self::sign_to_load_or_store(
                        &field_specifier,
                        word_from_jump_address,
                        original_sign,
                    ),
                    bytes: Self::bytes_to_store(contents, word_from_jump_address, &field_specifier),
                },
            )
            .unwrap();
    }

    fn stz(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        let original_sign = self
            .memory
            .get(self.modified_address(instruction))
            .unwrap()
            .sign;

        self.memory
            .set(
                self.modified_address(instruction),
                Word {
                    sign: Self::sign_to_load_or_store(&field_specifier, Word::ZERO, original_sign),
                    bytes: Self::bytes_to_store(contents, Word::ZERO, &field_specifier),
                },
            )
            .unwrap();
    }

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

    fn handle_48(&mut self, instruction: Word) {
        match instruction.field().value() {
            2 => self.enta(instruction),
            _ => panic!("Illegal field in code 48 instruction"),
        };
    }

    fn word_to_enter(&self, instruction: Word) -> Word {
        let mut m = Word::from_i32(self.modified_address(instruction)).unwrap();
        if m.to_i32() == 0 {
            m = m.with_sign(instruction.sign);
        }
        m
    }

    fn enta(&mut self, instruction: Word) {
        self.r_a = self.word_to_enter(instruction);
    }

    fn handle_49(&mut self, instruction: Word) {}

    fn handle_50(&mut self, instruction: Word) {}

    fn handle_51(&mut self, instruction: Word) {}

    fn handle_52(&mut self, instruction: Word) {}

    fn handle_53(&mut self, instruction: Word) {}

    fn handle_54(&mut self, instruction: Word) {}

    fn handle_55(&mut self, instruction: Word) {
        self.r_x = self.word_to_enter(instruction);
    }

    fn cmpa(&mut self, instruction: Word) {}

    fn cmp1(&mut self, instruction: Word) {}

    fn cmp2(&mut self, instruction: Word) {}

    fn cmp3(&mut self, instruction: Word) {}

    fn cmp4(&mut self, instruction: Word) {}

    fn cmp5(&mut self, instruction: Word) {}

    fn cmp6(&mut self, instruction: Word) {}

    fn cmpx(&mut self, instruction: Word) {}
}
