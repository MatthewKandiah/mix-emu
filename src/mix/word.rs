use std::num::TryFromIntError;

use super::byte::Byte;
use super::byte_overflow::ByteOverflow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Word {
    pub values: [Byte; 6],
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
        return Ok(Self { values: result });
    }

    pub fn from_i32(value: i32) -> Result<Self, TryFromIntError> {
        let mut abs = value.abs();
        let sign_byte_value = match value {
            0.. => 1,
            _ => 0,
        };
        let first_byte_value = abs / i32::pow(64, 4);
        abs %= i32::pow(64, 4);
        let second_byte_value = abs / i32::pow(64, 3);
        abs %= i32::pow(64, 3);
        let third_byte_value = abs / i32::pow(64, 2);
        abs %= i32::pow(64, 2);
        let fourth_byte_value = abs / 64;
        abs %= 64;
        let fifth_byte_value = abs;

        let sign_byte = Byte::from_u8(sign_byte_value.try_into()?).unwrap();
        let first_byte = Byte::from_u8(first_byte_value.try_into()?).unwrap();
        let second_byte = Byte::from_u8(second_byte_value.try_into()?).unwrap();
        let third_byte = Byte::from_u8(third_byte_value.try_into()?).unwrap();
        let fourth_byte = Byte::from_u8(fourth_byte_value.try_into()?).unwrap();
        let fifth_byte = Byte::from_u8(fifth_byte_value.try_into()?).unwrap();

        Ok(Word {
            values: [
                sign_byte,
                first_byte,
                second_byte,
                third_byte,
                fourth_byte,
                fifth_byte,
            ],
        })
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

    pub fn as_integer(&self) -> i32 {
        let sign = self.sign() as i32;
        let first_byte = self.values[1].value as i32;
        let second_byte = self.values[2].value as i32;
        let third_byte = self.values[3].value as i32;
        let fourth_byte = self.values[4].value as i32;
        let fifth_byte = self.values[5].value as i32;
        sign * (first_byte * i32::pow(64, 4)
            + second_byte * i32::pow(64, 3)
            + third_byte * i32::pow(64, 2)
            + fourth_byte * i32::pow(64, 1)
            + fifth_byte * i32::pow(64, 0))
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

#[test]
fn it_should_convert_to_an_integer() {
    assert_eq!(Word::zero().as_integer(), 0);
    assert_eq!(Word::from_u8s([1, 0, 0, 0, 0, 1]).unwrap().as_integer(), 1);
    assert_eq!(Word::from_u8s([0, 0, 0, 0, 0, 1]).unwrap().as_integer(), -1);
    assert_eq!(Word::from_u8s([1, 0, 0, 0, 1, 2]).unwrap().as_integer(), 66);
    assert_eq!(
        Word::from_u8s([1, 0, 0, 1, 2, 3]).unwrap().as_integer(),
        4227
    );
    assert_eq!(
        Word::from_u8s([1, 0, 1, 2, 3, 4]).unwrap().as_integer(),
        270532
    );
    assert_eq!(
        Word::from_u8s([1, 1, 2, 3, 4, 5]).unwrap().as_integer(),
        17314053
    );
}
