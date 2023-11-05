use super::{byte::Byte, byte_overflow::ByteOverflow};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JumpRegister {
    pub values: [Byte; 2],
}

#[derive(Debug)]
pub enum InvalidInputError {
    UnexpectedNegative,
    ByteOverflow,
}

impl JumpRegister {
    pub fn zero() -> Self {
        Self {
            values: [Byte::zero(); 2],
        }
    }

    pub fn from_u8s(values: [u8; 2]) -> Result<Self, ByteOverflow> {
        let first = Byte::from_u8(values[0]);
        let second = Byte::from_u8(values[1]);

        let mut result: [Byte; 2] = [Byte::zero(); 2];
        match first {
            Ok(x) => result[0] = x,
            Err(x) => return Err(ByteOverflow { value: x.value }),
        }
        match second {
            Ok(x) => result[1] = x,
            Err(x) => return Err(ByteOverflow { value: x.value }),
        }
        return Ok(Self { values: result });
    }


    pub fn from_i32(value: i32) -> Result<Self, InvalidInputError> {
        if value < 0 {return Err(InvalidInputError::UnexpectedNegative)};
        let first_byte_value = value / 64;
        let second_byte_value = value % 64;

        let first_byte = match Byte::from_u8(first_byte_value.try_into().unwrap()) {
            Ok(x) => x,
            Err(_) => return Err(InvalidInputError::ByteOverflow),
        };
        let second_byte = match Byte::from_u8(second_byte_value.try_into().unwrap()) {
            Ok(x) => x,
            Err(_) => return Err(InvalidInputError::ByteOverflow),
        };

        Ok(Self {
            values: [
                first_byte,
                second_byte,
            ],
        })
    }

    pub fn as_integer(&self) -> i16 {
        let first_byte = self.values[0].value as i16;
        let second_byte = self.values[1].value as i16;
        first_byte * i16::pow(2, 6) + second_byte
    }
}

#[test]
fn it_should_return_correct_integer() {
    assert_eq!(JumpRegister::from_u8s([1, 2]).unwrap().as_integer(), 66);
    assert_eq!(JumpRegister::from_u8s([2, 3]).unwrap().as_integer(), 131);
    assert_eq!(JumpRegister::from_u8s([0, 0]).unwrap().as_integer(), 0);
    assert_eq!(JumpRegister::from_u8s([63, 63]).unwrap().as_integer(), 4095);
}

#[test]
fn it_should_return_correct_jump_register()  {
    assert_eq!(JumpRegister::from_i32(1).unwrap(), JumpRegister::from_u8s([0,1]).unwrap());
    assert_eq!(JumpRegister::from_i32(100).unwrap(), JumpRegister::from_u8s([1,36]).unwrap());
    assert_eq!(JumpRegister::from_i32(0).unwrap(), JumpRegister::from_u8s([0,0]).unwrap());
    assert_eq!(JumpRegister::from_i32(64 * 64 - 1).unwrap(), JumpRegister::from_u8s([63,63]).unwrap());
}
