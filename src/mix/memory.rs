use super::{byte::Byte, word::Word};

#[derive(Clone, Copy)]
pub struct Memory {
    pub value: [Word; 4000],
}

#[derive(Debug, PartialEq, Eq)]
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

    pub fn write(
        &mut self,
        address: u16,
        to_write: [Option<Byte>; 6],
    ) -> Result<(), AddressOutOfRange> {
        if address >= 4000 {
            return Err(AddressOutOfRange { value: address });
        };

        for (i, b) in to_write.iter().enumerate() {
            match b {
                Some(byte) => self.value[address as usize].values[i] = *byte,
                None => (),
            };
        }

        return Ok(());
    }
}

#[test]
fn it_should_write_all_bytes() {
    let mut memory = Memory::zero();
    let to_write = [
        Some(Byte::from_u8(1).unwrap()),
        Some(Byte::from_u8(2).unwrap()),
        Some(Byte::from_u8(3).unwrap()),
        Some(Byte::from_u8(4).unwrap()),
        Some(Byte::from_u8(5).unwrap()),
        Some(Byte::from_u8(6).unwrap()),
    ];

    let _ = memory.write(101, to_write);

    assert_eq!(memory.value[101], Word::from_u8s([1, 2, 3, 4, 5, 6]).unwrap());
}

#[test]
fn it_should_write_all_bytes_except_sign() {
    let mut memory = Memory::zero();
    let to_write = [
        None,
        Some(Byte::from_u8(2).unwrap()),
        Some(Byte::from_u8(3).unwrap()),
        Some(Byte::from_u8(4).unwrap()),
        Some(Byte::from_u8(5).unwrap()),
        Some(Byte::from_u8(6).unwrap()),
    ];

    let _ = memory.write(101, to_write);

    assert_eq!(memory.value[101], Word::from_u8s([0, 2, 3, 4, 5, 6]).unwrap());

}

#[test]
fn it_should_write_just_specified_bytes() {
    let mut memory = Memory::zero();
    let to_write = [
        None,
        Some(Byte::from_u8(2).unwrap()),
        None,
        Some(Byte::from_u8(4).unwrap()),
        None,
        Some(Byte::from_u8(6).unwrap()),
    ];

    let _ = memory.write(101, to_write);

    assert_eq!(memory.value[101], Word::from_u8s([0, 2, 0, 4, 0, 6]).unwrap());
}

#[test]
fn it_should_return_error_if_address_too_high() {
    let mut memory = Memory::zero();
    let to_write = [
        None,
        Some(Byte::from_u8(2).unwrap()),
        None,
        Some(Byte::from_u8(4).unwrap()),
        None,
        Some(Byte::from_u8(6).unwrap()),
    ];

    let result = memory.write(10100, to_write);

    assert_eq!(result.unwrap_err(), AddressOutOfRange{ value: 10100 });
}
