#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Byte {
    value: u8,
}

#[derive(Debug, PartialEq, Eq)]
struct ByteOverflow {
    value: u8,
}

impl Byte {
    fn from_u8(value: u8) -> Result<Self, ByteOverflow> {
        match value {
            0..=63 => Ok(Self { value }),
            _ => Err(ByteOverflow { value }),
        }
    }

    fn zero() -> Self {
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

#[derive(Debug, PartialEq, Eq)]
struct Word {
    values: [Byte; 6],
}

impl Word {
    fn from_u8s(values: [u8; 6]) -> Result<Self, ByteOverflow> {
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
        Word::from_u8s([61,62,63,64,65,66,]).unwrap_err(),
        ByteOverflow {value: 64},
        );
}

type IndexRegister = [Byte; 3];

type JumpRegister = [Byte; 2];

enum ComparisonIndicatorState {
    Less,
    Equal,
    Greater,
    Off,
}

struct MixComputer {
    // main register
    r_a: Word,
    // extension register
    r_x: Word,
    // index registers, used primarily for counting and referencing variable memory addresses
    r_i1: IndexRegister,
    r_i2: IndexRegister,
    r_i3: IndexRegister,
    r_i4: IndexRegister,
    r_i5: IndexRegister,
    r_i6: IndexRegister,
    // jump register, always hold the address of the instruction following the most recent jump operation
    r_j: JumpRegister,
    overflow_toggle: bool,
    comparison_indicator: ComparisonIndicatorState,
    memory: [Word; 4000],
}

fn main() {
    let max = Byte::from_u8(63);
    let min = Byte::from_u8(0);
    let example = Byte::from_u8(47);

    println!("{:06b}", max.unwrap().value);
    println!("{:06b}", min.unwrap().value);
    println!("{:06b}", example.unwrap().value);

    let word = Word::from_u8s([1, 2, 3, 4, 5, 6]);
    println!("{:?}", word);
    println!("{:?}", Word::from_u8s([100, 2, 3, 4, 5, 66]))
}
