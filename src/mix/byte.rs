use super::byte_overflow::ByteOverflow;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Byte {
    pub value: u8,
}

impl Byte {
    pub fn from_u8(value: u8) -> Result<Self, ByteOverflow> {
        match value {
            0..=63 => Ok(Self { value }),
            _ => Err(ByteOverflow { value }),
        }
    }

    pub fn zero() -> Self {
        Self { value: 0 }
    }
}

#[test]
fn it_should_make_a_byte_from_a_u8() {
    let x = Byte::from_u8(7);
    assert_eq!(x.unwrap(), Byte { value: 7 });
}

#[test]
fn it_should_return_overflow_error_if_u8_too_big() {
    let x = Byte::from_u8(64);
    assert_eq!(x.unwrap_err(), ByteOverflow { value: 64 });
}

#[test]
fn it_should_make_a_zero_byte() {
    assert_eq!(Byte::zero(), Byte { value: 0 });
}
