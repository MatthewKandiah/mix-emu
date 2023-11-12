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

#[derive(Debug, PartialEq, Eq)]
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

    pub fn from_i32(value: i32) -> Self {
        match value >= 0 {
            true => Self::PLUS,
            false => Self::MINUS,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Word (Sign, Byte, Byte, Byte, Byte, Byte);

#[derive(Debug, PartialEq, Eq)]
pub enum WordValueError {
    Overflow (i32),
    Underflow (i32),
}

impl Word {
    pub const ZERO: Self = Self (Sign::PLUS, Byte::ZERO, Byte::ZERO, Byte::ZERO, Byte::ZERO, Byte::ZERO);

    pub const MAX: Self = Self (Sign::PLUS, Byte::MAX, Byte::MAX, Byte::MAX, Byte::MAX, Byte::MAX);

    pub const MIN: Self = Self (Sign::MINUS, Byte::MAX, Byte::MAX, Byte::MAX, Byte::MAX, Byte::MAX);

    pub fn to_i32(&self) -> i32 {
        self.0.value() * (64_i32.pow(4) * self.1.to_i32() + 64_i32.pow(3) * self.2.to_i32() + 64_i32.pow(2) * self.3.to_i32() + 64 * self.4.to_i32() + self.5.to_i32())
    }

    pub fn from_i32(value: i32) -> Result<Self, WordValueError> {
        if value > 1_073_741_823 {
            return Err(WordValueError::Overflow(value));
        }
        if value < -1_073_741_823 {
            return Err(WordValueError::Underflow(value));
        }
        let sign = Sign::from_i32(value);
        let mut x = value.abs();
        let first_byte = Byte::from_i32(x / 64_i32.pow(4)).unwrap();
        x = x % 64_i32.pow(4);
        let second_byte = Byte::from_i32(x / 64_i32.pow(3)).unwrap();
        x = x % 64_i32.pow(3);
        let third_byte = Byte::from_i32(x / 64_i32.pow(2)).unwrap();
        x = x % 64_i32.pow(2);
        let fourth_byte = Byte::from_i32(x / 64).unwrap();
        x = x % 64;
        let fifth_byte = Byte::from_i32(x).unwrap();
        Ok(Word (sign, first_byte, second_byte, third_byte, fourth_byte, fifth_byte))
    }
}

#[test]
fn should_return_word_value_as_i32() {
    assert_eq!(Word::ZERO.to_i32(), 0);
    assert_eq!(Word::MAX.to_i32(), 1_073_741_823);
    assert_eq!(Word::MIN.to_i32(), -1_073_741_823);
    assert_eq!(Word (Sign::PLUS, Byte::from_i32(1).unwrap(), Byte::from_i32(2).unwrap(), Byte::from_i32(3).unwrap(), Byte::from_i32(4).unwrap(), Byte::from_i32(5).unwrap()).to_i32(), 17_314_053);
}

#[test]
fn should_make_correct_word_for_i32_value() {
    assert_eq!(Word::from_i32(0), Ok(Word::ZERO));
    assert_eq!(Word::from_i32(1_073_741_823), Ok(Word::MAX));
    assert_eq!(Word::from_i32(-1_073_741_823), Ok(Word::MIN));
    assert_eq!(Word::from_i32(17_314_053), Ok(Word (Sign::PLUS, Byte::from_i32(1).unwrap(), Byte::from_i32(2).unwrap(), Byte::from_i32(3).unwrap(), Byte::from_i32(4).unwrap(), Byte::from_i32(5).unwrap())));
    assert_eq!(Word::from_i32(1_073_741_824), Err(WordValueError::Overflow(1_073_741_824)));
    assert_eq!(Word::from_i32(-1_073_741_824), Err(WordValueError::Underflow(-1_073_741_824)));
}

#[derive(Debug, PartialEq, Eq)]
pub struct Index (Sign, Byte, Byte);

#[derive(Debug, PartialEq, Eq)]
pub enum IndexValueError {
    Overflow (i32),
    Underflow (i32),
}

impl Index {
    pub const ZERO: Self = Self (Sign::PLUS, Byte::ZERO, Byte::ZERO);

    pub const MAX: Self = Self (Sign::PLUS, Byte::MAX, Byte::MAX);

    pub const MIN: Self = Self (Sign::MINUS, Byte::MAX, Byte::MAX);

    pub fn to_i32(&self) -> i32 {
        self.0.value() * (self.1.to_i32() * 64 + self.2.to_i32())
    }

    pub fn from_i32(value: i32) -> Result<Self, IndexValueError> {
        if value > 4095 {
            return Err(IndexValueError::Overflow(value));
        }
        if value < -4095 {
            return Err(IndexValueError::Underflow(value));
        }
        let sign = Sign::from_i32(value);
        let first_byte = Byte::from_i32(value.abs() / 64).unwrap();
        let second_byte = Byte::from_i32(value.abs() % 64).unwrap();
        Ok(Self (sign, first_byte, second_byte))

    }
}

#[test]
fn should_return_index_value_as_i32() {
    assert_eq!(Index (Sign::PLUS, Byte::ZERO, Byte::ZERO).to_i32(), 0);
    assert_eq!(Index (Sign::PLUS, Byte::MAX, Byte::MAX).to_i32(), 4095);
    assert_eq!(Index (Sign::MINUS, Byte::MAX, Byte::MAX).to_i32(), -4095);
    assert_eq!(Index (Sign::MINUS, Byte::ZERO, Byte::MAX).to_i32(), -63);
    assert_eq!(Index (Sign::PLUS, Byte::from_i32(2).unwrap(), Byte::from_i32(3).unwrap()).to_i32(), 131);
}

#[test]
fn should_make_correct_index_for_i32_value() {
    assert_eq!(Index::from_i32(0), Ok(Index::ZERO));
    assert_eq!(Index::from_i32(4095), Ok(Index::MAX));
    assert_eq!(Index::from_i32(-4095), Ok(Index::MIN));
    assert_eq!(Index::from_i32(-63), Ok(Index (Sign::MINUS, Byte::ZERO, Byte::MAX)));
    assert_eq!(Index::from_i32(131), Ok(Index (Sign::PLUS, Byte::from_i32(2).unwrap(), Byte::from_i32(3).unwrap())));
    assert_eq!(Index::from_i32(4096), Err(IndexValueError::Overflow(4096)));
    assert_eq!(Index::from_i32(-4096), Err(IndexValueError::Underflow(-4096)));
}

pub struct JumpAddress (Byte, Byte);

impl JumpAddress {
    pub const ZERO: Self = Self (Byte::ZERO, Byte::ZERO);

    pub const MAX: Self = Self (Byte::MAX, Byte::MAX);
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