use crate::data_types::Word;

pub struct Memory([Word; 4000]);

impl Memory {
    pub const ZERO: Self = Self([Word::ZERO; 4000]);
}

