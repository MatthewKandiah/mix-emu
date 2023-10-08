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
            31 => Self::stx(self, instruction),
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
