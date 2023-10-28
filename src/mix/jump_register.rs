use super::{byte::Byte, byte_overflow::ByteOverflow};

#[derive(Clone, Copy)]
pub struct JumpRegister {
    pub values: [Byte; 2],
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
