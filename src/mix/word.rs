use super::byte::Byte;
use super::byte_overflow::ByteOverflow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Word {
    values: [Byte; 6],
}

impl Word {
    pub fn from_u8s(values: [u8; 6]) -> Result<Self, ByteOverflow> {
        let first = Byte::from_u8(values[0]);
        let second = Byte::from_u8(values[1]);
        let third = Byte::from_u8(values[2]);
        let fourth = Byte::from_u8(values[3]);
        let fifth = Byte::from_u8(values[4]);
        let sixth = Byte::from_u8(values[5]);

        let mut result: [Byte; 6] = [Byte::zero(); 6];
        match first {
            Ok(x) => result[0] = x,
            Err(x) => return Err(ByteOverflow { value: x.value }),
        }
        match second {
            Ok(x) => result[1] = x,
            Err(x) => return Err(ByteOverflow { value: x.value }),
        }
        match third {
            Ok(x) => result[2] = x,
            Err(x) => return Err(ByteOverflow { value: x.value }),
        }
        match fourth {
            Ok(x) => result[3] = x,
            Err(x) => return Err(ByteOverflow { value: x.value }),
        }
        match fifth {
            Ok(x) => result[4] = x,
            Err(x) => return Err(ByteOverflow { value: x.value }),
        }
        match sixth {
            Ok(x) => result[5] = x,
            Err(x) => return Err(ByteOverflow { value: x.value }),
        }
        return Ok(Word { values: result });
    }

    pub fn zero() -> Self {
        Self::from_u8s([0, 0, 0, 0, 0, 0]).unwrap()
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

    pub fn index(&self) -> u8 {
        self.values[3].value
    }

    pub fn modifier(&self) -> u8 {
        self.values[4].value
    }

    pub fn op_code(&self) -> u8 {
        self.values[5].value
    }
}

#[test]
fn it_should_make_a_word_from_u8_array() {
    assert_eq!(
        Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap(),
        Word {
            values: [
                Byte { value: 1 },
                Byte { value: 2 },
                Byte { value: 3 },
                Byte { value: 4 },
                Byte { value: 5 },
                Byte { value: 6 },
            ]
        }
    );
}

#[test]
fn it_should_return_overflow_error_containing_first_value_that_is_too_big() {
    assert_eq!(
        Word::from_u8s([61, 62, 63, 64, 65, 66,]).unwrap_err(),
        ByteOverflow { value: 64 },
    );
}

#[test]
fn sign_should_be_positive_if_sign_byte_non_zero() {
    assert_eq!(Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap().sign(), 1)
}

#[test]
fn sign_should_be_negative_if_sign_byte_zero() {
    assert_eq!(Word::from_u8s([0, 2, 3, 4, 5, 6]).unwrap().sign(), -1)
}

#[test]
fn it_should_calculate_address_from_the_first_three_bytes() {
    assert_eq!(Word::from_u8s([0, 1, 2, 3, 4, 5]).unwrap().address(), -66);
    assert_eq!(Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap().address(), 131);
    assert_eq!(Word::from_u8s([0, 0, 0, 0, 0, 0]).unwrap().address(), 0);
    assert_eq!(
        Word::from_u8s([63, 63, 63, 63, 63, 63]).unwrap().address(),
        4095
    );
    assert_eq!(
        Word::from_u8s([0, 63, 63, 63, 63, 63]).unwrap().address(),
        -4095
    );
}

