use super::{
    byte::Byte, index_register::IndexRegister, jump_register::JumpRegister, memory::Memory,
    word::Word,
};

pub enum ComparisonIndicatorState {
    Less,
    Equal,
    Greater,
    Off,
}

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
            _ => panic!("Illegal op code"),
        }
    }

    fn lda(&mut self, word: Word) -> () {
        let index_value = match word.index() {
            0 => 0,
            1 => self.r_i1.address(),
            2 => self.r_i2.address(),
            3 => self.r_i3.address(),
            4 => self.r_i4.address(),
            5 => self.r_i5.address(),
            6 => self.r_i6.address(),
            _ => panic!("Illegal index register value"),
        };
        let modified_address = word.address() + index_value;
        let full_contents = self.memory.contents(modified_address as u16).unwrap();
        let left_limit = word.modifier() / 8;
        let right_limit = word.modifier() % 8;
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
        self.r_a = Word {
            values: final_value,
        };
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
    computer.memory.value[1] = Word::from_u8s([1,2,3,4,5,6]).unwrap();
    computer.memory.value[101] = Word::from_u8s([7,8,9,10,11,12]).unwrap();
    computer.r_i3 = IndexRegister::from_u8s([1,1,36]).unwrap();
    assert_eq!(computer.r_a, Word::zero());
    computer.handle_instruction(Word::from_u8s([1,0,1,3,5,8]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([7,8,9,10,11,12]).unwrap());
}

#[test]
fn should_handle_lda_instruction_with_negative_index_no_modifier() {
    let mut computer = Computer::new();
    computer.memory.value[1] = Word::from_u8s([1,2,3,4,5,6]).unwrap();
    computer.memory.value[101] = Word::from_u8s([7,8,9,10,11,12]).unwrap();
    computer.r_i3 = IndexRegister::from_u8s([0,1,36]).unwrap();
    assert_eq!(computer.r_a, Word::zero());
    computer.handle_instruction(Word::from_u8s([1,1,37,3,5,8]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([1,2,3,4,5,6]).unwrap());
}

#[test]
fn should_handle_lda_instruction_no_index_with_modifier() {
    let mut computer = Computer::new();
    computer.memory.value[1] = Word::from_u8s([1,2,3,4,5,6]).unwrap();
    assert_eq!(computer.r_a, Word::zero());

    computer.handle_instruction(Word::from_u8s([1,0,1,0,1,8]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([0,0,0,0,1,2]).unwrap());

    computer.handle_instruction(Word::from_u8s([1,0,1,0,3,8]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([0,0,1,2,3,4]).unwrap());

    computer.handle_instruction(Word::from_u8s([1,0,1,0,20,8]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([0,0,0,3,4,5]).unwrap());

    computer.handle_instruction(Word::from_u8s([1,0,1,0,45,8]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([0,0,0,0,0,6]).unwrap());

    computer.handle_instruction(Word::from_u8s([1,0,1,0,0,8]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([0,0,0,0,0,1]).unwrap());
}

#[test]
fn should_handle_lda_instruction_with_index_with_modifier() {
    let mut computer = Computer::new();
    computer.memory.value[7] = Word::from_u8s([7,8,9,10,11,12]).unwrap();
    computer.memory.value[2007] = Word::from_u8s([13,14,15,16,17,18]).unwrap();
    computer.r_i5 = IndexRegister::from_u8s([2,31, 16]).unwrap();
    computer.r_i6 = IndexRegister::from_u8s([0, 31, 16]).unwrap();
    assert_eq!(computer.r_a, Word::zero());

    computer.handle_instruction(Word::from_u8s([1, 0, 7, 5, 13, 8]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([0,14,15,16,17,18]).unwrap());

    computer.handle_instruction(Word::from_u8s([1, 31, 23, 6, 2, 8]).unwrap());
    assert_eq!(computer.r_a, Word::from_u8s([0,0,0,7,8,9]).unwrap());
}
