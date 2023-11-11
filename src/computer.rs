#[derive(Debug, PartialEq, Eq)]
pub enum Bit {
    ZERO,
    ONE,
}

impl Bit {
    pub fn value(&self) -> i32 {
        match self {
            Self::ZERO => 0,
            Self::ONE => 1,
        }
    }

    pub fn from_bool(value: bool) -> Self {
        match value {
            true => Bit::ONE,
            false => Bit::ZERO,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Byte (Bit, Bit, Bit, Bit, Bit, Bit);

#[derive(Debug, PartialEq, Eq)]
pub enum ByteValueError {
    Overflow (i32),
    Underflow (i32),
}

impl Byte {
    pub const ZERO: Self = Self (Bit::ZERO, Bit::ZERO, Bit::ZERO, Bit::ZERO, Bit::ZERO, Bit::ZERO);

    pub const MAX: Self = Self (Bit::ONE, Bit::ONE, Bit::ONE, Bit::ONE, Bit::ONE, Bit::ONE);

    pub fn to_i32(&self) -> i32 {
        self.0.value() * 32 + self.1.value() * 16 + self.2.value() * 8 + self.3.value() * 4 + self.4.value() * 2 + self.5.value()
    }


    pub fn from_i32(value: i32) -> Result<Self, ByteValueError> {
        if value > 63 {
            return Err(ByteValueError::Overflow(value));
        }
        if value < 0 {
            return Err(ByteValueError::Underflow(value));
        }
        let mut x = value;
        let first_bit = Bit::from_bool(x >= 32);
        x = x % 32;
        let second_bit = Bit::from_bool(x >= 16);
        x = x % 16;
        let third_bit = Bit::from_bool(x >= 8);
        x = x % 8;
        let fourth_bit = Bit::from_bool(x >= 4);
        x = x % 4;
        let fifth_bit = Bit::from_bool(x >= 2);
        x = x % 2;
        let sixth_bit = Bit::from_bool(x == 1);
        Ok(Self (first_bit, second_bit, third_bit, fourth_bit, fifth_bit, sixth_bit))
    }
}

#[test]
fn should_return_byte_value_as_i32() {
    assert_eq!(Byte (Bit::ZERO, Bit::ZERO, Bit::ZERO, Bit::ZERO, Bit::ZERO, Bit::ZERO).to_i32(), 0);
    assert_eq!(Byte (Bit::ONE, Bit::ONE, Bit::ONE, Bit::ONE, Bit::ONE, Bit::ONE).to_i32(), 63);
    assert_eq!(Byte (Bit::ONE, Bit::ZERO, Bit::ONE, Bit::ZERO, Bit::ONE, Bit::ZERO).to_i32(), 42);
    assert_eq!(Byte (Bit::ZERO, Bit::ONE, Bit::ZERO, Bit::ONE, Bit::ZERO, Bit::ONE).to_i32(), 21);
}

#[test]
fn should_make_correct_byte_for_i32_value() {
    assert_eq!(Byte::from_i32(0), Ok(Byte::ZERO));
    assert_eq!(Byte::from_i32(63), Ok(Byte::MAX));
    assert_eq!(Byte::from_i32(42), Ok(Byte (Bit::ONE, Bit::ZERO, Bit::ONE, Bit::ZERO, Bit::ONE, Bit::ZERO)));
    assert_eq!(Byte::from_i32(21), Ok(Byte (Bit::ZERO, Bit::ONE, Bit::ZERO, Bit::ONE, Bit::ZERO, Bit::ONE)));
    assert_eq!(Byte::from_i32(64), Err(ByteValueError::Overflow(64)));
    assert_eq!(Byte::from_i32(-1), Err(ByteValueError::Underflow(-1)));
}

pub enum Sign {
    PLUS,
    MINUS,
}

impl Sign {
    pub fn value(&self) -> i32 {
        match self {
            Self::PLUS => 1,
            Self::MINUS => -1,
        }
    }
}

pub struct Word (Sign, Byte, Byte, Byte, Byte, Byte);

impl Word {
    pub const ZERO: Self = Self (Sign::PLUS, Byte::ZERO, Byte::ZERO, Byte::ZERO, Byte::ZERO, Byte::ZERO);
}

pub struct Index (Sign, Byte, Byte);

impl Index {
    pub const ZERO: Self = Self (Sign::PLUS, Byte::ZERO, Byte::ZERO);
}

pub struct JumpAddress (Byte, Byte);

impl JumpAddress {
    pub const ZERO: Self = Self (Byte::ZERO, Byte::ZERO);
}

pub enum ComparisonIndicatorState {
    EQUAL,
    GREATER,
    LESS,
}

pub struct Memory ([Word; 4000]);

impl Memory {
    pub const ZERO: Self = Self([Word::ZERO; 4000]);
}

pub struct Computer {
    loaded_instruction: Option<Word>,
    r_a: Word,
    r_x: Word,
    r_i1: Index,
    r_i2: Index,
    r_i3: Index,
    r_i4: Index,
    r_i5: Index,
    r_i6: Index,
    r_j: JumpAddress,
    overflow: bool,
    comparison_indicator: Option<ComparisonIndicatorState>,
    memory: Memory,
}

impl Computer {
    pub fn new() -> Self {
        Self {
            loaded_instruction: None,
            r_a: Word::ZERO,
            r_x: Word::ZERO,
            r_i1: Index::ZERO,
            r_i2: Index::ZERO,
            r_i3: Index::ZERO,
            r_i4: Index::ZERO,
            r_i5: Index::ZERO,
            r_i6: Index::ZERO,
            r_j: JumpAddress::ZERO,
            overflow: false,
            comparison_indicator: None,
            memory: Memory::ZERO,
        }
    }
}
