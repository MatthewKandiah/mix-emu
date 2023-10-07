use super::byte::Byte;

pub struct IndexRegister {
    values: [Byte; 3],
}

impl IndexRegister {
    pub fn zero() -> Self {
        Self {
            values: [Byte::zero(); 3],
        }
    }

    pub fn sign(&self) -> i8 {
        match self.values[0].value {
            0 => -1,
            _ => 1,
        }
    }

    pub fn address(&self) -> i16 {
        let sign = self.sign() as i16;
        let first_byte = self.values[1].value as i16;
        let second_byte = self.values[2].value as i16;
        sign * (first_byte * i16::pow(2, 6) + second_byte)
    }
}

