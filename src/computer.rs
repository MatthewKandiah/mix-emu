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
}

pub struct Byte (Bit, Bit, Bit, Bit, Bit, Bit);

impl Byte {
    pub const ZERO: Self = Self (Bit::ZERO, Bit::ZERO, Bit::ZERO, Bit::ZERO, Bit::ZERO, Bit::ZERO);
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
