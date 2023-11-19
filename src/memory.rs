use crate::data_types::Word;

pub const SIZE: usize = 4000;

pub struct Memory {
    content: [Word; SIZE],
}

pub enum AccessError {
    InvalidAddress(i32),
}

pub enum SetError {
    InvalidAddress(i32),
}

impl Memory {
    pub const ZERO: Self = Self {
        content: [Word::ZERO; SIZE],
    };

    pub fn get(&self, address: i32) -> Result<Word, AccessError> {
        let usize_address: Result<usize, _> = address.try_into();
        match usize_address {
            Err(_) => Err(AccessError::InvalidAddress(address)),
            Ok(x) => match x {
                SIZE.. => Err(AccessError::InvalidAddress(address)),
                _ => Ok(self.content[x]),
            },
        }
    }

    pub fn set(&mut self, address: i32, value: Word) -> Result<(), SetError> {
        let usize_address: Result<usize, _> = address.try_into();
        match usize_address {
            Err(_) => Err(SetError::InvalidAddress(address)),
            Ok(x) => match x {
                SIZE.. => Err(SetError::InvalidAddress(address)),
                _ => Ok(self.content[x] = value),
            },
        }
    }
}
