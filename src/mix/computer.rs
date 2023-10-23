use super::{
    byte::Byte, index_register::IndexRegister, jump_register::JumpRegister, memory::Memory,
    word::Word,
};

#[derive(Clone, Copy)]
pub enum ComparisonIndicatorState {
    Less,
    Equal,
    Greater,
    Off,
}

#[derive(Clone, Copy)]
pub struct Computer {
    // main register
    r_a: Word,
    // extension register
    r_x: Word,
    // index registers, used primarily for counting and referencing variable memory addresses
    r_i1: IndexRegister,
    r_i2: IndexRegister,
    r_i3: IndexRegister,
    r_i4: IndexRegister,
    r_i5: IndexRegister,
    r_i6: IndexRegister,
    // jump register, always hold the address of the instruction following the most recent jump operation
    r_j: JumpRegister,
    overflow_toggle: bool,
    comparison_indicator: ComparisonIndicatorState,
    memory: Memory,
}

impl Computer {
    pub fn new() -> Self {
        Computer {
            r_a: Word::zero(),
            r_x: Word::zero(),
            r_i1: IndexRegister::zero(),
            r_i2: IndexRegister::zero(),
            r_i3: IndexRegister::zero(),
            r_i4: IndexRegister::zero(),
            r_i5: IndexRegister::zero(),
            r_i6: IndexRegister::zero(),
            r_j: JumpRegister::zero(),
            overflow_toggle: false,
            comparison_indicator: ComparisonIndicatorState::Off,
            memory: Memory::zero(),
        }
    }

    pub fn handle_instruction(&mut self, instruction: Word) -> () {
        match instruction.op_code() {
            1 => Self::add(self, instruction),
            2 => Self::sub(self, instruction),
            3 => Self::mul(self, instruction),
            4 => Self::div(self, instruction),
            8 => Self::lda(self, instruction),
            9 => Self::ld1(self, instruction),
            10 => Self::ld2(self, instruction),
            11 => Self::ld3(self, instruction),
            12 => Self::ld4(self, instruction),
            13 => Self::ld5(self, instruction),
            14 => Self::ld6(self, instruction),
            15 => Self::ldx(self, instruction),
            16 => Self::ldan(self, instruction),
            17 => Self::ld1n(self, instruction),
            18 => Self::ld2n(self, instruction),
            19 => Self::ld3n(self, instruction),
            20 => Self::ld4n(self, instruction),
            21 => Self::ld5n(self, instruction),
            22 => Self::ld6n(self, instruction),
            23 => Self::ldxn(self, instruction),
            24 => Self::sta(self, instruction),
            25 => Self::st1(self, instruction),
            26 => Self::st2(self, instruction),
            27 => Self::st3(self, instruction),
            28 => Self::st4(self, instruction),
            29 => Self::st5(self, instruction),
            30 => Self::st6(self, instruction),
            31 => Self::stx(self, instruction),
            32 => Self::stj(self, instruction),
            33 => Self::stz(self, instruction),
            48 => Self::handle_48(self, instruction),
            49 => Self::handle_49(self, instruction),
            50 => Self::handle_50(self, instruction),
            51 => Self::handle_51(self, instruction),
            52 => Self::handle_52(self, instruction),
            53 => Self::handle_53(self, instruction),
            54 => Self::handle_54(self, instruction),
            55 => Self::handle_55(self, instruction),
            _ => panic!("Illegal op code"),
        }
    }

    fn index_value(&self, index: u8) -> i16 {
        match index {
            0 => 0,
            1 => self.r_i1.address(),
            2 => self.r_i2.address(),
            3 => self.r_i3.address(),
            4 => self.r_i4.address(),
            5 => self.r_i5.address(),
            6 => self.r_i6.address(),
            _ => panic!("Illegal index register value"),
        }
    }

    fn extract_bytes_from_word(full_contents: Word, modifier: u8) -> Word {
        let left_limit = modifier / 8;
        let right_limit = modifier % 8;
        if right_limit < left_limit {
            panic!("Illegal modifier range");
        }
        let mut buffer: Vec<Byte> = Vec::new();
        for i in 0..6 {
            if i >= left_limit && i <= right_limit {
                buffer.push(full_contents.values[i as usize])
            }
        }
        let mut final_value = [Byte::zero(); 6];
        let prepended_zero_count = 6 - buffer.len();
        for i in 0..buffer.len() {
            final_value[prepended_zero_count + i] = buffer[i];
        }
        return Word {
            values: final_value,
        };
    }

    fn modified_address(&self, word: Word) -> u16 {
        (word.address() + self.index_value(word.index())) as u16
    }

    fn word_to_load(&self, instruction_word: Word) -> Word {
        let full_word = self
            .memory
            .contents(self.modified_address(instruction_word))
            .unwrap();
        return Self::extract_bytes_from_word(full_word, instruction_word.modifier());
    }

    fn index_register_to_load(&self, instruction_word: Word) -> IndexRegister {
        let full_word = self.word_to_load(instruction_word);
        let sign = full_word.values[0];
        let first_byte = full_word.values[4];
        let second_byte = full_word.values[5];
        return IndexRegister {
            values: [sign, first_byte, second_byte],
        };
    }

    fn lda(&mut self, instruction_word: Word) -> () {
        self.r_a = self.word_to_load(instruction_word);
    }

    fn ldx(&mut self, instruction_word: Word) -> () {
        self.r_x = self.word_to_load(instruction_word);
    }

    fn ld1(&mut self, instruction_word: Word) -> () {
        self.r_i1 = self.index_register_to_load(instruction_word);
    }

    fn ld2(&mut self, instruction_word: Word) -> () {
        self.r_i2 = self.index_register_to_load(instruction_word);
    }

    fn ld3(&mut self, instruction_word: Word) -> () {
        self.r_i3 = self.index_register_to_load(instruction_word);
    }

    fn ld4(&mut self, instruction_word: Word) -> () {
        self.r_i4 = self.index_register_to_load(instruction_word);
    }

    fn ld5(&mut self, instruction_word: Word) -> () {
        self.r_i5 = self.index_register_to_load(instruction_word);
    }

    fn ld6(&mut self, instruction_word: Word) -> () {
        self.r_i6 = self.index_register_to_load(instruction_word);
    }

    fn negated_word_to_load(&self, instruction_word: Word) -> Word {
        let word = self.word_to_load(instruction_word);
        let negated_sign_byte = match word.values[0].value {
            0 => Byte::from_u8(1).unwrap(),
            _ => Byte::zero(),
        };
        return Word {
            values: [
                negated_sign_byte,
                word.values[1],
                word.values[2],
                word.values[3],
                word.values[4],
                word.values[5],
            ],
        };
    }

    fn negated_index_register_to_load(&self, instruction_word: Word) -> IndexRegister {
        let index_register = self.index_register_to_load(instruction_word);
        let negated_sign_byte = match index_register.values[0].value {
            0 => Byte::from_u8(1).unwrap(),
            _ => Byte::zero(),
        };
        return IndexRegister {
            values: [
                negated_sign_byte,
                index_register.values[1],
                index_register.values[2],
            ],
        };
    }

    fn ldan(&mut self, instruction_word: Word) -> () {
        self.r_a = self.negated_word_to_load(instruction_word);
    }

    fn ldxn(&mut self, instruction_word: Word) -> () {
        self.r_x = self.negated_word_to_load(instruction_word);
    }

    fn ld1n(&mut self, instruction_word: Word) -> () {
        self.r_i1 = self.negated_index_register_to_load(instruction_word);
    }

    fn ld2n(&mut self, instruction_word: Word) -> () {
        self.r_i2 = self.negated_index_register_to_load(instruction_word);
    }

    fn ld3n(&mut self, instruction_word: Word) -> () {
        self.r_i3 = self.negated_index_register_to_load(instruction_word);
    }

    fn ld4n(&mut self, instruction_word: Word) -> () {
        self.r_i4 = self.negated_index_register_to_load(instruction_word);
    }

    fn ld5n(&mut self, instruction_word: Word) -> () {
        self.r_i5 = self.negated_index_register_to_load(instruction_word);
    }

    fn ld6n(&mut self, instruction_word: Word) -> () {
        self.r_i6 = self.negated_index_register_to_load(instruction_word);
    }

    fn bytes_to_store(full_word: Word, instruction_word: Word) -> [Option<Byte>; 6] {
        let left_limit = instruction_word.modifier() / 8;
        let right_limit = instruction_word.modifier() % 8;
        if right_limit < left_limit {
            panic!("Illegal modifier, right limit less than left limit");
        }
        let mut bytes_to_write: [Option<Byte>; 6] = [None; 6];
        for i in 0..6 {
            if i >= left_limit && i <= right_limit {
                bytes_to_write[i as usize] = Some(full_word.values[i as usize]);
            }
        }
        return bytes_to_write;
    }

    fn bytes_to_store_from_index_register(
        index_register: IndexRegister,
        instruction_word: Word,
    ) -> [Option<Byte>; 6] {
        let equivalent_word = Word {
            values: [
                index_register.values[0],
                Byte::zero(),
                Byte::zero(),
                Byte::zero(),
                index_register.values[1],
                index_register.values[2],
            ],
        };
        return Self::bytes_to_store(equivalent_word, instruction_word);
    }

    fn bytes_to_store_from_jump_register(
        jump_register: JumpRegister,
        instruction_word: Word,
    ) -> [Option<Byte>; 6] {
        let equivalent_index_register = IndexRegister {
            values: [
                Byte::from_u8(1).unwrap(),
                jump_register.values[0],
                jump_register.values[1],
            ],
        };
        return Self::bytes_to_store_from_index_register(
            equivalent_index_register,
            instruction_word,
        );
    }

    fn sta(&mut self, instruction_word: Word) -> () {
        let bytes_to_write = Self::bytes_to_store(self.r_a, instruction_word);
        let _ = self
            .memory
            .write(self.modified_address(instruction_word), bytes_to_write);
    }

    fn stx(&mut self, instruction_word: Word) -> () {
        let bytes_to_write = Self::bytes_to_store(self.r_x, instruction_word);
        let _ = self
            .memory
            .write(self.modified_address(instruction_word), bytes_to_write);
    }

    fn st1(&mut self, instruction_word: Word) -> () {
        let bytes_to_write = Self::bytes_to_store_from_index_register(self.r_i1, instruction_word);
        let _ = self
            .memory
            .write(self.modified_address(instruction_word), bytes_to_write);
    }

    fn st2(&mut self, instruction_word: Word) -> () {
        let bytes_to_write = Self::bytes_to_store_from_index_register(self.r_i2, instruction_word);
        let _ = self
            .memory
            .write(self.modified_address(instruction_word), bytes_to_write);
    }

    fn st3(&mut self, instruction_word: Word) -> () {
        let bytes_to_write = Self::bytes_to_store_from_index_register(self.r_i3, instruction_word);
        let _ = self
            .memory
            .write(self.modified_address(instruction_word), bytes_to_write);
    }

    fn st4(&mut self, instruction_word: Word) -> () {
        let bytes_to_write = Self::bytes_to_store_from_index_register(self.r_i4, instruction_word);
        let _ = self
            .memory
            .write(self.modified_address(instruction_word), bytes_to_write);
    }

    fn st5(&mut self, instruction_word: Word) -> () {
        let bytes_to_write = Self::bytes_to_store_from_index_register(self.r_i5, instruction_word);
        let _ = self
            .memory
            .write(self.modified_address(instruction_word), bytes_to_write);
    }

    fn st6(&mut self, instruction_word: Word) -> () {
        let bytes_to_write = Self::bytes_to_store_from_index_register(self.r_i6, instruction_word);
        let _ = self
            .memory
            .write(self.modified_address(instruction_word), bytes_to_write);
    }

    fn stj(&mut self, instruction_word: Word) -> () {
        let bytes_to_write = Self::bytes_to_store_from_jump_register(self.r_j, instruction_word);
        let _ = self
            .memory
            .write(self.modified_address(instruction_word), bytes_to_write);
    }

    fn stz(&mut self, instruction_word: Word) -> () {
        let bytes_to_write = Self::bytes_to_store(Word::zero(), instruction_word);
        let _ = self
            .memory
            .write(self.modified_address(instruction_word), bytes_to_write);
    }

    fn add(&mut self, instruction_word: Word) -> () {
        let bytes_to_add = self.word_to_load(instruction_word);
        if bytes_to_add.as_integer() == 0 {
            return;
        }
        let sum = bytes_to_add.as_integer() + self.r_a.as_integer();
        let overflow = sum / 1_073_741_824;
        let remainder = sum % 1_073_741_824;
        self.overflow_toggle = overflow != 0;
        self.r_a = Word::from_i32(remainder).unwrap();
    }

    fn sub(&mut self, instruction_word: Word) -> () {
        let bytes_to_sub = self.word_to_load(instruction_word);
        if bytes_to_sub.as_integer() == 0 {
            return;
        }
        let diff = self.r_a.as_integer() - bytes_to_sub.as_integer();
        let overflow = diff / 1_073_741_824;
        let remainder = diff % 1_073_741_824;
        self.overflow_toggle = overflow != 0;
        self.r_a = Word::from_i32(remainder).unwrap();
    }

    fn mul(&mut self, instruction_word: Word) -> () {
        let bytes_to_mul = self.word_to_load(instruction_word);
        let product: i64 = (bytes_to_mul.as_integer() as i64) * (self.r_a.as_integer() as i64);
        let sign = product / product.abs();
        let a_val = product.abs() / i64::pow(64, 5);
        let x_val = product.abs() % i64::pow(64, 5);
        self.r_a = Word::from_i32((sign * a_val).try_into().unwrap()).unwrap();
        self.r_x = Word::from_i32((sign * x_val).try_into().unwrap()).unwrap();
    }

    fn div(&mut self, instruction_word: Word) -> () {
        let bytes_to_div = self.word_to_load(instruction_word);
        if bytes_to_div.as_integer() == 0 {
            self.r_a = Word::zero();
            self.r_x = Word::zero();
            self.overflow_toggle = true;
        }
        let original_a_val = self.r_a.as_integer().abs();
        let original_x_val = self.r_x.as_integer().abs();
        let original_value =
            (self.r_a.sign() as i32) * original_a_val * i32::pow(64, 5) + original_x_val;
        let result = original_value / bytes_to_div.as_integer();
        if result >= i32::pow(64, 5) {
            self.r_a = Word::zero();
            self.r_x = Word::zero();
            self.overflow_toggle = true;
        }
        self.r_x =
            Word::from_i32((self.r_a.sign() as i32) * (original_value % bytes_to_div.as_integer()))
                .unwrap();
        self.r_a = Word::from_i32(result).unwrap();
    }

    fn handle_48(&mut self, instruction_word: Word) -> () {
        match instruction_word.modifier() {
            2 => self.enta(instruction_word),
            _ => panic!("Illegal modifier"),
        }
    }

    fn handle_49(&mut self, instruction_word: Word) -> () {
        match instruction_word.modifier() {
            2 => self.ent1(instruction_word),
            _ => panic!("Illegal modifier"),
        }
    }

    fn handle_50(&mut self, instruction_word: Word) -> () {
        match instruction_word.modifier() {
            2 => self.ent2(instruction_word),
            _ => panic!("Illegal modifier"),
        }
    }

    fn handle_51(&mut self, instruction_word: Word) -> () {
        match instruction_word.modifier() {
            2 => self.ent3(instruction_word),
            _ => panic!("Illegal modifier"),
        }
    }

    fn handle_52(&mut self, instruction_word: Word) -> () {
        match instruction_word.modifier() {
            2 => self.ent4(instruction_word),
            _ => panic!("Illegal modifier"),
        }
    }

    fn handle_53(&mut self, instruction_word: Word) -> () {
        match instruction_word.modifier() {
            2 => self.ent5(instruction_word),
            _ => panic!("Illegal modifier"),
        }
    }

    fn handle_54(&mut self, instruction_word: Word) -> () {
        match instruction_word.modifier() {
            2 => self.ent6(instruction_word),
            _ => panic!("Illegal modifier"),
        }
    }

    fn handle_55(&mut self, instruction_word: Word) -> () {
        match instruction_word.modifier() {
            2 => self.entx(instruction_word),
            _ => panic!("Illegal modifier"),
        }
    }

    fn value_to_enter(&self, instruction_word: Word) -> i32 {
        let m = instruction_word.address();
        if m != 0 {
            return m.into();
        }
        let index_value = self.index_value(instruction_word.index());
        if index_value != 0 {
            return index_value.into();
        }
        return 0;
    }

    fn word_to_enter(&self, instruction_word: Word) -> Word {
        let value = self.value_to_enter(instruction_word);
        if value != 0 {
            return Word::from_i32(value).unwrap();
        }
        return match instruction_word.sign() {
            1 => Word::from_u8s([1,0,0,0,0,0]).unwrap(),
            -1 => Word::from_u8s([0,0,0,0,0,0]).unwrap(),
            _ => panic!("Illegal sign"),
        }
    }

    fn index_register_to_enter(&self, instruction_word: Word) -> IndexRegister {
        let value = self.value_to_enter(instruction_word);
        if value != 0 {
            return IndexRegister::from_i32(value).unwrap();
        }
        return match instruction_word.sign() {
            1 => IndexRegister::from_u8s([1, 0, 0]).unwrap(),
            -1 => IndexRegister::from_u8s([0, 0, 0]).unwrap(),
            _ => panic!("Illegal sign"),
        }
    }

    fn enta(&mut self, instruction_word: Word) -> () {
        self.r_a = self.word_to_enter(instruction_word);
    }

    fn entx(&mut self, instruction_word: Word) -> () {
        self.r_x = self.word_to_enter(instruction_word);
    }

    fn ent1(&mut self, instruction_word: Word) -> () {
        self.r_i1 = self.index_register_to_enter(instruction_word);
    }

    fn ent2(&mut self, instruction_word: Word) -> () {
        self.r_i2 = self.index_register_to_enter(instruction_word);
    }

    fn ent3(&mut self, instruction_word: Word) -> () {
        self.r_i3 = self.index_register_to_enter(instruction_word);
    }

    fn ent4(&mut self, instruction_word: Word) -> () {
        self.r_i4 = self.index_register_to_enter(instruction_word);
    }

    fn ent5(&mut self, instruction_word: Word) -> () {
        self.r_i5 = self.index_register_to_enter(instruction_word);
    }

    fn ent6(&mut self, instruction_word: Word) -> () {
        self.r_i6 = self.index_register_to_enter(instruction_word);
    }
}

#[test]
fn should_handle_lda_instruction_no_index_no_modifier() {
    let mut computer = Computer::new();
    computer.memory.value[1] = Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap();
    assert_eq!(computer.r_a, Word::zero());
    computer.handle_instruction(Word::from_u8s([1, 0, 1, 0, 5, 8]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap());
}

#[test]
fn should_handle_lda_instruction_with_positive_index_no_modifier() {
    let mut computer = Computer::new();
    computer.memory.value[1] = Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap();
    computer.memory.value[101] = Word::from_u8s([7, 8, 9, 10, 11, 12]).unwrap();
    computer.r_i3 = IndexRegister::from_u8s([1, 1, 36]).unwrap();
    assert_eq!(computer.r_a, Word::zero());
    computer.handle_instruction(Word::from_u8s([1, 0, 1, 3, 5, 8]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([7, 8, 9, 10, 11, 12]).unwrap());
}

#[test]
fn should_handle_lda_instruction_with_negative_index_no_modifier() {
    let mut computer = Computer::new();
    computer.memory.value[1] = Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap();
    computer.memory.value[101] = Word::from_u8s([7, 8, 9, 10, 11, 12]).unwrap();
    computer.r_i3 = IndexRegister::from_u8s([0, 1, 36]).unwrap();
    assert_eq!(computer.r_a, Word::zero());
    computer.handle_instruction(Word::from_u8s([1, 1, 37, 3, 5, 8]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap());
}

#[test]
fn should_handle_lda_instruction_no_index_with_modifier() {
    let mut computer = Computer::new();
    computer.memory.value[1] = Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap();
    assert_eq!(computer.r_a, Word::zero());

    computer.handle_instruction(Word::from_u8s([1, 0, 1, 0, 1, 8]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([0, 0, 0, 0, 1, 2]).unwrap());

    computer.handle_instruction(Word::from_u8s([1, 0, 1, 0, 3, 8]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([0, 0, 1, 2, 3, 4]).unwrap());

    computer.handle_instruction(Word::from_u8s([1, 0, 1, 0, 20, 8]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([0, 0, 0, 3, 4, 5]).unwrap());

    computer.handle_instruction(Word::from_u8s([1, 0, 1, 0, 45, 8]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([0, 0, 0, 0, 0, 6]).unwrap());

    computer.handle_instruction(Word::from_u8s([1, 0, 1, 0, 0, 8]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([0, 0, 0, 0, 0, 1]).unwrap());
}

#[test]
fn should_handle_lda_instruction_with_index_with_modifier() {
    let mut computer = Computer::new();
    computer.memory.value[7] = Word::from_u8s([7, 8, 9, 10, 11, 12]).unwrap();
    computer.memory.value[2007] = Word::from_u8s([13, 14, 15, 16, 17, 18]).unwrap();
    computer.r_i5 = IndexRegister::from_u8s([2, 31, 16]).unwrap();
    computer.r_i6 = IndexRegister::from_u8s([0, 31, 16]).unwrap();
    assert_eq!(computer.r_a, Word::zero());

    computer.handle_instruction(Word::from_u8s([1, 0, 7, 5, 13, 8]).unwrap());
    assert_eq!(
        computer.r_a,
        Word::from_u8s([0, 14, 15, 16, 17, 18]).unwrap()
    );

    computer.handle_instruction(Word::from_u8s([1, 31, 23, 6, 2, 8]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([0, 0, 0, 7, 8, 9]).unwrap());
}

#[test]
fn should_handle_ldx_instruction_no_index_no_modifier() {
    let mut computer = Computer::new();
    computer.memory.value[1] = Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap();
    assert_eq!(computer.r_x, Word::zero());
    computer.handle_instruction(Word::from_u8s([1, 0, 1, 0, 5, 15]).unwrap());
    assert_eq!(computer.r_x, Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap());
}

#[test]
fn should_handle_ldx_instruction_with_positive_index_no_modifier() {
    let mut computer = Computer::new();
    computer.memory.value[1] = Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap();
    computer.memory.value[101] = Word::from_u8s([7, 8, 9, 10, 11, 12]).unwrap();
    computer.r_i3 = IndexRegister::from_u8s([1, 1, 36]).unwrap();
    assert_eq!(computer.r_x, Word::zero());
    computer.handle_instruction(Word::from_u8s([1, 0, 1, 3, 5, 15]).unwrap());
    assert_eq!(computer.r_x, Word::from_u8s([7, 8, 9, 10, 11, 12]).unwrap());
}

#[test]
fn should_handle_ldx_instruction_with_negative_index_no_modifier() {
    let mut computer = Computer::new();
    computer.memory.value[1] = Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap();
    computer.memory.value[101] = Word::from_u8s([7, 8, 9, 10, 11, 12]).unwrap();
    computer.r_i3 = IndexRegister::from_u8s([0, 1, 36]).unwrap();
    assert_eq!(computer.r_x, Word::zero());
    computer.handle_instruction(Word::from_u8s([1, 1, 37, 3, 5, 15]).unwrap());
    assert_eq!(computer.r_x, Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap());
}

#[test]
fn should_handle_ldx_instruction_no_index_with_modifier() {
    let mut computer = Computer::new();
    computer.memory.value[1] = Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap();
    assert_eq!(computer.r_x, Word::zero());

    computer.handle_instruction(Word::from_u8s([1, 0, 1, 0, 1, 15]).unwrap());
    assert_eq!(computer.r_x, Word::from_u8s([0, 0, 0, 0, 1, 2]).unwrap());

    computer.handle_instruction(Word::from_u8s([1, 0, 1, 0, 3, 15]).unwrap());
    assert_eq!(computer.r_x, Word::from_u8s([0, 0, 1, 2, 3, 4]).unwrap());

    computer.handle_instruction(Word::from_u8s([1, 0, 1, 0, 20, 15]).unwrap());
    assert_eq!(computer.r_x, Word::from_u8s([0, 0, 0, 3, 4, 5]).unwrap());

    computer.handle_instruction(Word::from_u8s([1, 0, 1, 0, 45, 15]).unwrap());
    assert_eq!(computer.r_x, Word::from_u8s([0, 0, 0, 0, 0, 6]).unwrap());

    computer.handle_instruction(Word::from_u8s([1, 0, 1, 0, 0, 15]).unwrap());
    assert_eq!(computer.r_x, Word::from_u8s([0, 0, 0, 0, 0, 1]).unwrap());
}

#[test]
fn should_handle_ldx_instruction_with_index_with_modifier() {
    let mut computer = Computer::new();
    computer.memory.value[7] = Word::from_u8s([7, 8, 9, 10, 11, 12]).unwrap();
    computer.memory.value[2007] = Word::from_u8s([13, 14, 15, 16, 17, 18]).unwrap();
    computer.r_i5 = IndexRegister::from_u8s([2, 31, 16]).unwrap();
    computer.r_i6 = IndexRegister::from_u8s([0, 31, 16]).unwrap();
    assert_eq!(computer.r_x, Word::zero());

    computer.handle_instruction(Word::from_u8s([1, 0, 7, 5, 13, 15]).unwrap());
    assert_eq!(
        computer.r_x,
        Word::from_u8s([0, 14, 15, 16, 17, 18]).unwrap()
    );

    computer.handle_instruction(Word::from_u8s([1, 31, 23, 6, 2, 15]).unwrap());
    assert_eq!(computer.r_x, Word::from_u8s([0, 0, 0, 7, 8, 9]).unwrap());
}

#[test]
fn should_handle_ldi_instruction_no_index_no_modifier() {
    let mut computer = Computer::new();
    computer.memory.value[1] = Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap();
    assert_eq!(computer.r_i1, IndexRegister::zero());
    computer.handle_instruction(Word::from_u8s([1, 0, 1, 0, 5, 9]).unwrap());
    assert_eq!(computer.r_i1, IndexRegister::from_u8s([1, 5, 6]).unwrap());
}

#[test]
fn should_handle_ldi_instruction_with_positive_index_no_modifier() {
    let mut computer = Computer::new();
    computer.memory.value[1] = Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap();
    computer.memory.value[101] = Word::from_u8s([7, 8, 9, 10, 11, 12]).unwrap();
    computer.r_i3 = IndexRegister::from_u8s([1, 1, 36]).unwrap();
    assert_eq!(computer.r_i2, IndexRegister::zero());
    computer.handle_instruction(Word::from_u8s([1, 0, 1, 3, 5, 10]).unwrap());
    assert_eq!(computer.r_i2, IndexRegister::from_u8s([7, 11, 12]).unwrap());
}

#[test]
fn should_handle_ldi_instruction_with_negative_index_no_modifier() {
    let mut computer = Computer::new();
    computer.memory.value[1] = Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap();
    computer.memory.value[101] = Word::from_u8s([7, 8, 9, 10, 11, 12]).unwrap();
    computer.r_i4 = IndexRegister::from_u8s([0, 1, 36]).unwrap();
    assert_eq!(computer.r_i3, IndexRegister::zero());
    computer.handle_instruction(Word::from_u8s([1, 1, 37, 4, 5, 11]).unwrap());
    assert_eq!(computer.r_i3, IndexRegister::from_u8s([1, 5, 6]).unwrap());
}

#[test]
fn should_handle_ldi_instruction_no_index_with_modifier() {
    let mut computer = Computer::new();
    computer.memory.value[1] = Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap();
    assert_eq!(computer.r_i4, IndexRegister::zero());

    computer.handle_instruction(Word::from_u8s([1, 0, 1, 0, 1, 12]).unwrap());
    assert_eq!(computer.r_i4, IndexRegister::from_u8s([0, 1, 2]).unwrap());

    computer.handle_instruction(Word::from_u8s([1, 0, 1, 0, 3, 12]).unwrap());
    assert_eq!(computer.r_i4, IndexRegister::from_u8s([0, 3, 4]).unwrap());

    computer.handle_instruction(Word::from_u8s([1, 0, 1, 0, 20, 12]).unwrap());
    assert_eq!(computer.r_i4, IndexRegister::from_u8s([0, 4, 5]).unwrap());

    computer.handle_instruction(Word::from_u8s([1, 0, 1, 0, 45, 12]).unwrap());
    assert_eq!(computer.r_i4, IndexRegister::from_u8s([0, 0, 6]).unwrap());

    computer.handle_instruction(Word::from_u8s([1, 0, 1, 0, 0, 12]).unwrap());
    assert_eq!(computer.r_i4, IndexRegister::from_u8s([0, 0, 1]).unwrap());
}

#[test]
fn should_handle_ldi_instruction_with_index_with_modifier() {
    let mut computer = Computer::new();
    computer.memory.value[7] = Word::from_u8s([7, 8, 9, 10, 11, 12]).unwrap();
    computer.memory.value[2007] = Word::from_u8s([13, 14, 15, 16, 17, 18]).unwrap();
    computer.r_i1 = IndexRegister::from_u8s([2, 31, 16]).unwrap();
    computer.r_i2 = IndexRegister::from_u8s([0, 31, 16]).unwrap();
    assert_eq!(computer.r_i5, IndexRegister::zero());

    computer.handle_instruction(Word::from_u8s([1, 0, 7, 1, 13, 13]).unwrap());
    assert_eq!(computer.r_i5, IndexRegister::from_u8s([0, 17, 18]).unwrap());

    computer.handle_instruction(Word::from_u8s([1, 31, 23, 2, 2, 14]).unwrap());
    assert_eq!(computer.r_i6, IndexRegister::from_u8s([0, 8, 9]).unwrap());
}

#[test]
fn should_handle_ldan_instruction_no_index_no_modifier() {
    let mut computer = Computer::new();
    computer.memory.value[1] = Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap();
    assert_eq!(computer.r_a, Word::zero());
    computer.handle_instruction(Word::from_u8s([1, 0, 1, 0, 5, 16]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([0, 2, 3, 4, 5, 6]).unwrap());
}

#[test]
fn should_handle_ldan_instruction_with_positive_index_no_modifier() {
    let mut computer = Computer::new();
    computer.memory.value[1] = Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap();
    computer.memory.value[101] = Word::from_u8s([7, 8, 9, 10, 11, 12]).unwrap();
    computer.r_i3 = IndexRegister::from_u8s([1, 1, 36]).unwrap();
    assert_eq!(computer.r_a, Word::zero());
    computer.handle_instruction(Word::from_u8s([1, 0, 1, 3, 5, 16]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([0, 8, 9, 10, 11, 12]).unwrap());
}

#[test]
fn should_handle_ldan_instruction_with_negative_index_no_modifier() {
    let mut computer = Computer::new();
    computer.memory.value[1] = Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap();
    computer.memory.value[101] = Word::from_u8s([7, 8, 9, 10, 11, 12]).unwrap();
    computer.r_i3 = IndexRegister::from_u8s([0, 1, 36]).unwrap();
    assert_eq!(computer.r_a, Word::zero());
    computer.handle_instruction(Word::from_u8s([1, 1, 37, 3, 5, 16]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([0, 2, 3, 4, 5, 6]).unwrap());
}

#[test]
fn should_handle_ldan_instruction_no_index_with_modifier() {
    let mut computer = Computer::new();
    computer.memory.value[1] = Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap();
    assert_eq!(computer.r_a, Word::zero());

    computer.handle_instruction(Word::from_u8s([1, 0, 1, 0, 1, 16]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([1, 0, 0, 0, 1, 2]).unwrap());

    computer.handle_instruction(Word::from_u8s([1, 0, 1, 0, 3, 16]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([1, 0, 1, 2, 3, 4]).unwrap());

    computer.handle_instruction(Word::from_u8s([1, 0, 1, 0, 20, 16]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([1, 0, 0, 3, 4, 5]).unwrap());

    computer.handle_instruction(Word::from_u8s([1, 0, 1, 0, 45, 16]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([1, 0, 0, 0, 0, 6]).unwrap());

    computer.handle_instruction(Word::from_u8s([1, 0, 1, 0, 0, 16]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([1, 0, 0, 0, 0, 1]).unwrap());
}

#[test]
fn should_handle_ldan_instruction_with_index_with_modifier() {
    let mut computer = Computer::new();
    computer.memory.value[7] = Word::from_u8s([7, 8, 9, 10, 11, 12]).unwrap();
    computer.memory.value[2007] = Word::from_u8s([13, 14, 15, 16, 17, 18]).unwrap();
    computer.r_i5 = IndexRegister::from_u8s([2, 31, 16]).unwrap();
    computer.r_i6 = IndexRegister::from_u8s([0, 31, 16]).unwrap();
    assert_eq!(computer.r_a, Word::zero());

    computer.handle_instruction(Word::from_u8s([1, 0, 7, 5, 13, 16]).unwrap());
    assert_eq!(
        computer.r_a,
        Word::from_u8s([1, 14, 15, 16, 17, 18]).unwrap()
    );

    computer.handle_instruction(Word::from_u8s([1, 31, 23, 6, 2, 16]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([1, 0, 0, 7, 8, 9]).unwrap());
}

#[test]
fn should_handle_ldxn_instruction_no_index_no_modifier() {
    let mut computer = Computer::new();
    computer.memory.value[1] = Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap();
    assert_eq!(computer.r_x, Word::zero());
    computer.handle_instruction(Word::from_u8s([1, 0, 1, 0, 5, 23]).unwrap());
    assert_eq!(computer.r_x, Word::from_u8s([0, 2, 3, 4, 5, 6]).unwrap());
}

#[test]
fn should_handle_ldxn_instruction_with_positive_index_no_modifier() {
    let mut computer = Computer::new();
    computer.memory.value[1] = Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap();
    computer.memory.value[101] = Word::from_u8s([7, 8, 9, 10, 11, 12]).unwrap();
    computer.r_i3 = IndexRegister::from_u8s([1, 1, 36]).unwrap();
    assert_eq!(computer.r_x, Word::zero());
    computer.handle_instruction(Word::from_u8s([1, 0, 1, 3, 5, 23]).unwrap());
    assert_eq!(computer.r_x, Word::from_u8s([0, 8, 9, 10, 11, 12]).unwrap());
}

#[test]
fn should_handle_ldxn_instruction_with_negative_index_no_modifier() {
    let mut computer = Computer::new();
    computer.memory.value[1] = Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap();
    computer.memory.value[101] = Word::from_u8s([7, 8, 9, 10, 11, 12]).unwrap();
    computer.r_i3 = IndexRegister::from_u8s([0, 1, 36]).unwrap();
    assert_eq!(computer.r_x, Word::zero());
    computer.handle_instruction(Word::from_u8s([1, 1, 37, 3, 5, 23]).unwrap());
    assert_eq!(computer.r_x, Word::from_u8s([0, 2, 3, 4, 5, 6]).unwrap());
}

#[test]
fn should_handle_ldxn_instruction_no_index_with_modifier() {
    let mut computer = Computer::new();
    computer.memory.value[1] = Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap();
    assert_eq!(computer.r_x, Word::zero());

    computer.handle_instruction(Word::from_u8s([1, 0, 1, 0, 1, 23]).unwrap());
    assert_eq!(computer.r_x, Word::from_u8s([1, 0, 0, 0, 1, 2]).unwrap());

    computer.handle_instruction(Word::from_u8s([1, 0, 1, 0, 3, 23]).unwrap());
    assert_eq!(computer.r_x, Word::from_u8s([1, 0, 1, 2, 3, 4]).unwrap());

    computer.handle_instruction(Word::from_u8s([1, 0, 1, 0, 20, 23]).unwrap());
    assert_eq!(computer.r_x, Word::from_u8s([1, 0, 0, 3, 4, 5]).unwrap());

    computer.handle_instruction(Word::from_u8s([1, 0, 1, 0, 45, 23]).unwrap());
    assert_eq!(computer.r_x, Word::from_u8s([1, 0, 0, 0, 0, 6]).unwrap());

    computer.handle_instruction(Word::from_u8s([1, 0, 1, 0, 0, 23]).unwrap());
    assert_eq!(computer.r_x, Word::from_u8s([1, 0, 0, 0, 0, 1]).unwrap());
}

#[test]
fn should_handle_ldxn_instruction_with_index_with_modifier() {
    let mut computer = Computer::new();
    computer.memory.value[7] = Word::from_u8s([7, 8, 9, 10, 11, 12]).unwrap();
    computer.memory.value[2007] = Word::from_u8s([13, 14, 15, 16, 17, 18]).unwrap();
    computer.r_i5 = IndexRegister::from_u8s([2, 31, 16]).unwrap();
    computer.r_i6 = IndexRegister::from_u8s([0, 31, 16]).unwrap();
    assert_eq!(computer.r_x, Word::zero());

    computer.handle_instruction(Word::from_u8s([1, 0, 7, 5, 13, 23]).unwrap());
    assert_eq!(
        computer.r_x,
        Word::from_u8s([1, 14, 15, 16, 17, 18]).unwrap()
    );

    computer.handle_instruction(Word::from_u8s([1, 31, 23, 6, 2, 23]).unwrap());
    assert_eq!(computer.r_x, Word::from_u8s([1, 0, 0, 7, 8, 9]).unwrap());
}

#[test]
fn should_handle_ldin_instruction_no_index_no_modifier() {
    let mut computer = Computer::new();
    computer.memory.value[1] = Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap();
    assert_eq!(computer.r_i1, IndexRegister::zero());
    computer.handle_instruction(Word::from_u8s([1, 0, 1, 0, 5, 17]).unwrap());
    assert_eq!(computer.r_i1, IndexRegister::from_u8s([0, 5, 6]).unwrap());
}

#[test]
fn should_handle_ldin_instruction_with_positive_index_no_modifier() {
    let mut computer = Computer::new();
    computer.memory.value[1] = Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap();
    computer.memory.value[101] = Word::from_u8s([7, 8, 9, 10, 11, 12]).unwrap();
    computer.r_i3 = IndexRegister::from_u8s([1, 1, 36]).unwrap();
    assert_eq!(computer.r_i2, IndexRegister::zero());
    computer.handle_instruction(Word::from_u8s([1, 0, 1, 3, 5, 18]).unwrap());
    assert_eq!(computer.r_i2, IndexRegister::from_u8s([0, 11, 12]).unwrap());
}

#[test]
fn should_handle_ldin_instruction_with_negative_index_no_modifier() {
    let mut computer = Computer::new();
    computer.memory.value[1] = Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap();
    computer.memory.value[101] = Word::from_u8s([7, 8, 9, 10, 11, 12]).unwrap();
    computer.r_i4 = IndexRegister::from_u8s([0, 1, 36]).unwrap();
    assert_eq!(computer.r_i3, IndexRegister::zero());
    computer.handle_instruction(Word::from_u8s([1, 1, 37, 4, 5, 19]).unwrap());
    assert_eq!(computer.r_i3, IndexRegister::from_u8s([0, 5, 6]).unwrap());
}

#[test]
fn should_handle_ldin_instruction_no_index_with_modifier() {
    let mut computer = Computer::new();
    computer.memory.value[1] = Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap();
    assert_eq!(computer.r_i4, IndexRegister::zero());

    computer.handle_instruction(Word::from_u8s([1, 0, 1, 0, 1, 20]).unwrap());
    assert_eq!(computer.r_i4, IndexRegister::from_u8s([1, 1, 2]).unwrap());

    computer.handle_instruction(Word::from_u8s([1, 0, 1, 0, 3, 20]).unwrap());
    assert_eq!(computer.r_i4, IndexRegister::from_u8s([1, 3, 4]).unwrap());

    computer.handle_instruction(Word::from_u8s([1, 0, 1, 0, 20, 20]).unwrap());
    assert_eq!(computer.r_i4, IndexRegister::from_u8s([1, 4, 5]).unwrap());

    computer.handle_instruction(Word::from_u8s([1, 0, 1, 0, 45, 20]).unwrap());
    assert_eq!(computer.r_i4, IndexRegister::from_u8s([1, 0, 6]).unwrap());

    computer.handle_instruction(Word::from_u8s([1, 0, 1, 0, 0, 20]).unwrap());
    assert_eq!(computer.r_i4, IndexRegister::from_u8s([1, 0, 1]).unwrap());
}

#[test]
fn should_handle_ldin_instruction_with_index_with_modifier() {
    let mut computer = Computer::new();
    computer.memory.value[7] = Word::from_u8s([7, 8, 9, 10, 11, 12]).unwrap();
    computer.memory.value[2007] = Word::from_u8s([13, 14, 15, 16, 17, 18]).unwrap();
    computer.r_i1 = IndexRegister::from_u8s([2, 31, 16]).unwrap();
    computer.r_i2 = IndexRegister::from_u8s([0, 31, 16]).unwrap();
    assert_eq!(computer.r_i5, IndexRegister::zero());

    computer.handle_instruction(Word::from_u8s([1, 0, 7, 1, 13, 21]).unwrap());
    assert_eq!(computer.r_i5, IndexRegister::from_u8s([1, 17, 18]).unwrap());

    computer.handle_instruction(Word::from_u8s([1, 31, 23, 2, 2, 22]).unwrap());
    assert_eq!(computer.r_i6, IndexRegister::from_u8s([1, 8, 9]).unwrap());
}

#[test]
fn should_handle_sta_instruction() {
    let mut computer = Computer::new();
    computer.memory.value[100] = Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap();
    computer.memory.value[101] = Word::from_u8s([2, 3, 4, 5, 6, 7]).unwrap();
    computer.r_a = Word::from_u8s([11, 12, 13, 14, 15, 16]).unwrap();
    computer.r_i1 = IndexRegister::from_u8s([1, 0, 1]).unwrap();

    computer.handle_instruction(Word::from_u8s([1, 1, 36, 0, 5, 24]).unwrap());
    assert_eq!(
        computer.memory.contents(100).unwrap(),
        Word::from_u8s([11, 12, 13, 14, 15, 16]).unwrap()
    );

    computer.handle_instruction(Word::from_u8s([1, 1, 36, 1, 28, 24]).unwrap());
    assert_eq!(
        computer.memory.contents(101).unwrap(),
        Word::from_u8s([2, 3, 4, 14, 15, 7]).unwrap()
    );
}

#[test]
fn should_handle_stx_instruction() {
    let mut computer = Computer::new();
    computer.memory.value[100] = Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap();
    computer.memory.value[101] = Word::from_u8s([2, 3, 4, 5, 6, 7]).unwrap();
    computer.r_x = Word::from_u8s([11, 12, 13, 14, 15, 16]).unwrap();
    computer.r_i1 = IndexRegister::from_u8s([1, 0, 1]).unwrap();

    computer.handle_instruction(Word::from_u8s([1, 1, 36, 0, 5, 31]).unwrap());
    assert_eq!(
        computer.memory.contents(100).unwrap(),
        Word::from_u8s([11, 12, 13, 14, 15, 16]).unwrap()
    );

    computer.handle_instruction(Word::from_u8s([1, 1, 36, 1, 28, 31]).unwrap());
    assert_eq!(
        computer.memory.contents(101).unwrap(),
        Word::from_u8s([2, 3, 4, 14, 15, 7]).unwrap()
    );
}

#[test]
fn should_handle_sti_instruction() {
    let mut computer = Computer::new();
    computer.memory.value[100] = Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap();
    computer.memory.value[101] = Word::from_u8s([2, 3, 4, 5, 6, 7]).unwrap();
    computer.r_i2 = IndexRegister::from_u8s([11, 15, 16]).unwrap();
    computer.r_i1 = IndexRegister::from_u8s([1, 0, 1]).unwrap();

    computer.handle_instruction(Word::from_u8s([1, 1, 36, 0, 5, 26]).unwrap());
    assert_eq!(
        computer.memory.contents(100).unwrap(),
        Word::from_u8s([11, 0, 0, 0, 15, 16]).unwrap()
    );

    computer.handle_instruction(Word::from_u8s([1, 1, 36, 1, 28, 26]).unwrap());
    assert_eq!(
        computer.memory.contents(101).unwrap(),
        Word::from_u8s([2, 3, 4, 0, 15, 7]).unwrap()
    );
}

#[test]
fn should_handle_stj_instruction() {
    let mut computer = Computer::new();
    computer.memory.value[100] = Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap();
    computer.memory.value[101] = Word::from_u8s([2, 3, 4, 5, 6, 7]).unwrap();
    computer.r_j = JumpRegister::from_u8s([15, 16]).unwrap();
    computer.r_i1 = IndexRegister::from_u8s([1, 0, 1]).unwrap();

    computer.handle_instruction(Word::from_u8s([1, 1, 36, 0, 5, 32]).unwrap());
    assert_eq!(
        computer.memory.contents(100).unwrap(),
        Word::from_u8s([1, 0, 0, 0, 15, 16]).unwrap()
    );

    computer.handle_instruction(Word::from_u8s([1, 1, 36, 1, 28, 32]).unwrap());
    assert_eq!(
        computer.memory.contents(101).unwrap(),
        Word::from_u8s([2, 3, 4, 0, 15, 7]).unwrap()
    );
}

#[test]
fn should_handle_stz_instruction() {
    let mut computer = Computer::new();
    computer.memory.value[100] = Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap();
    computer.memory.value[101] = Word::from_u8s([2, 3, 4, 5, 6, 7]).unwrap();
    computer.r_a = Word::from_u8s([11, 12, 13, 14, 15, 16]).unwrap();
    computer.r_i1 = IndexRegister::from_u8s([1, 0, 1]).unwrap();

    computer.handle_instruction(Word::from_u8s([1, 1, 36, 0, 5, 24]).unwrap());
    assert_eq!(
        computer.memory.contents(100).unwrap(),
        Word::from_u8s([11, 12, 13, 14, 15, 16]).unwrap()
    );
    computer.handle_instruction(Word::from_u8s([1, 1, 36, 0, 5, 33]).unwrap());
    assert_eq!(computer.memory.contents(100).unwrap(), Word::zero(),);
}

#[test]
fn should_handle_add_instruction() {
    let mut computer = Computer::new();
    computer.r_a = Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap();
    computer.memory.value[10] = Word::from_u8s([1, 0, 0, 0, 0, 1]).unwrap();
    computer.memory.value[11] = Word::from_u8s([0, 0, 0, 0, 0, 1]).unwrap();

    computer.handle_instruction(Word::from_u8s([1, 0, 10, 0, 5, 1]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([1, 2, 3, 4, 5, 7]).unwrap());

    computer.handle_instruction(Word::from_u8s([1, 0, 11, 0, 5, 1]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap());
}

#[test]
fn should_handle_sub_instruction() {
    let mut computer = Computer::new();
    computer.r_a = Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap();
    computer.memory.value[10] = Word::from_u8s([1, 0, 0, 0, 0, 1]).unwrap();
    computer.memory.value[11] = Word::from_u8s([0, 0, 0, 0, 0, 1]).unwrap();

    computer.handle_instruction(Word::from_u8s([1, 0, 10, 0, 5, 2]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([1, 2, 3, 4, 5, 5]).unwrap());

    computer.handle_instruction(Word::from_u8s([1, 0, 11, 0, 5, 2]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap());
}

#[test]
fn should_not_change_sign_when_adding_zero() {
    let mut computer = Computer::new();
    computer.r_a = Word::from_u8s([0, 0, 0, 0, 0, 1]).unwrap();
    computer.memory.value[0] = Word::zero();

    computer.handle_instruction(Word::from_u8s([1, 0, 0, 0, 5, 1]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([0, 0, 0, 0, 0, 1]).unwrap());

    computer.handle_instruction(Word::from_u8s([0, 0, 0, 0, 5, 1]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([0, 0, 0, 0, 0, 1]).unwrap());
}

#[test]
fn should_not_change_sign_when_subbing_zero() {
    let mut computer = Computer::new();
    computer.r_a = Word::from_u8s([0, 0, 0, 0, 0, 1]).unwrap();
    computer.memory.value[0] = Word::zero();

    computer.handle_instruction(Word::from_u8s([1, 0, 0, 0, 5, 2]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([0, 0, 0, 0, 0, 1]).unwrap());

    computer.handle_instruction(Word::from_u8s([0, 0, 0, 0, 5, 2]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([0, 0, 0, 0, 0, 1]).unwrap());
}

#[test]
fn should_handle_mul_instruction() {
    let mut computer = Computer::new();
    computer.r_a = Word::from_u8s([1, 1, 1, 1, 1, 1]).unwrap();
    computer.memory.value[1000] = Word::from_u8s([1, 1, 1, 1, 1, 1]).unwrap();

    computer.handle_instruction(Word::from_u8s([1, 15, 40, 0, 5, 3]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([1, 0, 1, 2, 3, 4]).unwrap());
    assert_eq!(computer.r_x, Word::from_u8s([1, 5, 4, 3, 2, 1]).unwrap());
}

#[test]
fn should_handle_div_instruction() {
    let mut computer = Computer::new();
    computer.r_a = Word::from_u8s([1, 0, 0, 0, 0, 0]).unwrap();
    computer.r_x = Word::from_u8s([0, 0, 0, 0, 0, 17]).unwrap();
    computer.memory.value[1000] = Word::from_u8s([1, 0, 0, 0, 0, 3]).unwrap();

    computer.handle_instruction(Word::from_u8s([1, 15, 40, 0, 5, 4]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([1, 0, 0, 0, 0, 5]).unwrap());
    assert_eq!(computer.r_x, Word::from_u8s([1, 0, 0, 0, 0, 2]).unwrap());
}

#[test]
fn should_handle_enta_instruction() {
    let mut computer = Computer::new();
    computer.handle_instruction(Word::from_u8s([1, 2, 3, 0, 2, 48]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([1, 0, 0, 0, 2, 3]).unwrap());
}

#[test]
fn should_handle_enta_with_positive_zero() {
    let mut computer = Computer::new();
    computer.r_i3 = IndexRegister::from_u8s([0, 0, 0]).unwrap();
    computer.handle_instruction(Word::from_u8s([1, 0, 0, 3, 2, 48]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([1, 0, 0, 0, 0, 0]).unwrap());
}

#[test]
fn should_handle_enta_with_negative_zero() {
    let mut computer = Computer::new();
    computer.r_i3 = IndexRegister::from_u8s([1, 0, 0]).unwrap();
    computer.handle_instruction(Word::from_u8s([0, 0, 0, 3, 2, 48]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([0, 0, 0, 0, 0, 0]).unwrap());
}

#[test]
fn should_handle_enta_loading_index_register() {
    let mut computer = Computer::new();
    computer.r_i3 = IndexRegister::from_u8s([0, 7, 9]).unwrap();
    computer.handle_instruction(Word::from_u8s([1, 0, 0, 3, 2, 48]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([0, 0, 0, 0, 7, 9]).unwrap());
}
