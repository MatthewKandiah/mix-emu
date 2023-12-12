use crate::data_types::{Index, JumpAddress, Word};

pub struct Registers {
    pub a: Word,
    pub x: Word,
    pub i1: Index,
    pub i2: Index,
    pub i3: Index,
    pub i4: Index,
    pub i5: Index,
    pub i6: Index,
    pub j: JumpAddress,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: Word::ZERO,
            x: Word::ZERO,
            i1: Index::ZERO,
            i2: Index::ZERO,
            i3: Index::ZERO,
            i4: Index::ZERO,
            i5: Index::ZERO,
            i6: Index::ZERO,
            j: JumpAddress::ZERO,
        }
    }
}
