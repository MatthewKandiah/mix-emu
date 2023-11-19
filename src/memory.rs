use crate::data_types::Word;

pub const SIZE: usize = 4000;

pub struct Memory {
    content: [Word; SIZE],
}

impl Memory {
    pub const ZERO: Self = Self {
        content: [Word::ZERO; SIZE],
    };
}
