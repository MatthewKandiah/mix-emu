use crate::data_types::{Word, Index, JumpAddress};
use crate::memory::Memory;

pub enum ComparisonIndicatorState {
    EQUAL,
    GREATER,
    LESS,
}

pub struct Computer {
    current_instruction_address: Option<i32>,
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
            current_instruction_address: None,
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
}
