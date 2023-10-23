use std::num::TryFromIntError;

use super::{byte::Byte, byte_overflow::ByteOverflow};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IndexRegister {
    pub values: [Byte; 3],
}

impl IndexRegister {
    pub fn from_u8s(values: [u8; 3]) -> Result<Self, ByteOverflow> {
        let first = Byte::from_u8(values[0]);
        let second = Byte::from_u8(values[1]);
        let third = Byte::from_u8(values[2]);

        let mut result: [Byte; 3] = [Byte::zero(); 3];
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

        let sign_byte = Byte::from_u8(sign_byte_value.try_into()?).unwrap();
        let first_byte = Byte::from_u8(first_byte_value.try_into()?).unwrap();
        let second_byte = Byte::from_u8(second_byte_value.try_into()?).unwrap();

        Ok(Self {
            values: [
                sign_byte,
                first_byte,
                second_byte,
            ],
        })
    }

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

    pub fn negate_sign(&self) -> Self {
        let mut result_values = self.values;
        result_values[0] = match self.sign() {
            1 => Byte { value: 0 },
            -1 => Byte { value: 1 },
            _ => panic!("Failed to negate sign"),
        };
        return Self {
            values: result_values,
        };
    }
}

#[test]
fn it_should_return_correct_sign() {
    assert_eq!(IndexRegister::from_u8s([0, 1, 2]).unwrap().sign(), -1);
    assert_eq!(IndexRegister::from_u8s([1, 2, 3]).unwrap().sign(), 1);
}

#[test]
fn it_should_return_correct_address() {
    assert_eq!(IndexRegister::from_u8s([0, 1, 2]).unwrap().address(), -66);
    assert_eq!(IndexRegister::from_u8s([1, 2, 3]).unwrap().address(), 131);
    assert_eq!(IndexRegister::from_u8s([0, 0, 0]).unwrap().address(), 0);
    assert_eq!(
        IndexRegister::from_u8s([63, 63, 63]).unwrap().address(),
        4095
    );
}
