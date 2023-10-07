use super::{
    index_register::IndexRegister, jump_register::JumpRegister, memory::Memory, word::Word,
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
}
