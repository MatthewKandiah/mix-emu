use super::word::Word;

pub struct Memory {
    pub value: [Word; 4000],
}

#[derive(Debug)]
pub struct AddressOutOfRange {
    value: u16,
}

impl Memory {
    pub fn zero() -> Self {
        Self {
            value: [Word::zero(); 4000],
        }
    }

    pub fn contents(&self, address: u16) -> Result<Word, AddressOutOfRange> {
        match address >= 4000 {
            true => Err(AddressOutOfRange { value: address }),
            false => Ok(self.value[address as usize]),
        }
    }
}
