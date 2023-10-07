use super::byte::Byte;

pub struct JumpRegister {
    values: [Byte; 2],
}

impl JumpRegister {
    pub fn zero() -> Self {
        Self {
            values: [Byte::zero(); 2],
        }
    }

    pub fn address(&self) -> i16 {
        let first_byte = self.values[0].value as i16;
        let second_byte = self.values[1].value as i16;
        first_byte * i16::pow(2, 6) + second_byte
    }
}
