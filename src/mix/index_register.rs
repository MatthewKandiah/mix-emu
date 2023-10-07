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
