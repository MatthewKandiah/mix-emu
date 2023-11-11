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

pub struct Index (Sign, Byte, Byte);

pub struct JumpAddress (Byte, Byte);

pub enum ComparisonIndicatorState {
    EQUAL,
    GREATER,
    LESS,
}

pub struct Memory ([Word; 4000]);

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
    comparison_indicator: ComparisonIndicatorState,
    memory: Memory,
}
