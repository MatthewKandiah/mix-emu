use crate::data_types::{Word, Index, JumpAddress};
use crate::memory::Memory;

pub enum ComparisonIndicatorState {
    EQUAL,
    GREATER,
    LESS,
}

pub struct Computer {
    current_instruction_address: i32,
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
            current_instruction_address: 0,
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

    pub fn handle_next_instruction(&mut self) {
        let current_instruction = self.memory.get(self.current_instruction_address).unwrap();
        self.current_instruction_address += 1;
        self.handle_instruction(current_instruction);
    }

    fn handle_instruction(&mut self, instruction: Word) {
        match instruction.code() {
            0 => (),
            1 => self.add(instruction),
            2 => self.sub(instruction),
            3 => self.mul(instruction),
            4 => self.div(instruction),
            5 => self.handle_5(instruction),
            6 => self.handle_6(instruction),
            7 => self.mov(instruction),
            8 => self.lda(instruction),
            9 => self.ld1(instruction),
            10 => self.ld2(instruction),
            11 => self.ld3(instruction),
            12 => self.ld4(instruction),
            13 => self.ld5(instruction),
            14 => self.ld6(instruction),
            15 => self.ldx(instruction),
            16 => self.ldan(instruction),
            17 => self.ld1n(instruction),
            18 => self.ld2n(instruction),
            19 => self.ld3n(instruction),
            20 => self.ld4n(instruction),
            21 => self.ld5n(instruction),
            22 => self.ld6n(instruction),
            23 => self.ldxn(instruction),
            24 => self.sta(instruction),
            25 => self.st1(instruction),
            26 => self.st2(instruction),
            27 => self.st3(instruction),
            28 => self.st4(instruction),
            29 => self.st5(instruction),
            30 => self.st6(instruction),
            31 => self.stx(instruction),
            32 => self.stj(instruction),
            33 => self.stz(instruction),
            34 => self.jbus(instruction),
            35 => self.ioc(instruction),
            36 => self.input(instruction),
            37 => self.output(instruction),
            38 => self.jred(instruction),
            39 => self.handle_39(instruction),
            40 => self.handle_40(instruction),
            41 => self.handle_41(instruction),
            42 => self.handle_42(instruction),
            43 => self.handle_43(instruction),
            44 => self.handle_44(instruction),
            45 => self.handle_45(instruction),
            46 => self.handle_46(instruction),
            47 => self.handle_47(instruction),
            48 => self.handle_48(instruction),
            49 => self.handle_49(instruction),
            50 => self.handle_50(instruction),
            51 => self.handle_51(instruction),
            52 => self.handle_52(instruction),
            53 => self.handle_53(instruction),
            54 => self.handle_54(instruction),
            55 => self.handle_55(instruction),
            56 => self.cmpa(instruction),
            57 => self.cmp1(instruction),
            58 => self.cmp2(instruction),
            59 => self.cmp3(instruction),
            60 => self.cmp4(instruction),
            61 => self.cmp5(instruction),
            62 => self.cmp6(instruction),
            63 => self.cmpx(instruction),
            _ => panic!("Invalid instruction code"),
        }
    }

    fn add(&mut self, instruction: Word) {}

    fn sub(&mut self, instruction: Word) {}

    fn mul(&mut self, instruction: Word) {}

    fn div(&mut self, instruction: Word) {}

    fn handle_5(&mut self, instruction: Word) {}

    fn handle_6(&mut self, instruction: Word) {}

    fn mov(&mut self, instruction: Word) {}

    fn lda(&mut self, instruction: Word) {}

    fn ld1(&mut self, instruction: Word) {}

    fn ld2(&mut self, instruction: Word) {}

    fn ld3(&mut self, instruction: Word) {}

    fn ld4(&mut self, instruction: Word) {}

    fn ld5(&mut self, instruction: Word) {}

    fn ld6(&mut self, instruction: Word) {}

    fn ldx(&mut self, instruction: Word) {}

    fn ldan(&mut self, instruction: Word) {}

    fn ld1n(&mut self, instruction: Word) {}

    fn ld2n(&mut self, instruction: Word) {}

    fn ld3n(&mut self, instruction: Word) {}

    fn ld4n(&mut self, instruction: Word) {}

    fn ld5n(&mut self, instruction: Word) {}

    fn ld6n(&mut self, instruction: Word) {}

    fn ldxn(&mut self, instruction: Word) {}

    fn sta(&mut self, instruction: Word) {}

    fn st1(&mut self, instruction: Word) {}

    fn st2(&mut self, instruction: Word) {}

    fn st3(&mut self, instruction: Word) {}

    fn st4(&mut self, instruction: Word) {}

    fn st5(&mut self, instruction: Word) {}

    fn st6(&mut self, instruction: Word) {}

    fn stx(&mut self, instruction: Word) {}

    fn stj(&mut self, instruction: Word) {}

    fn stz(&mut self, instruction: Word) {}

    fn jbus(&mut self, instruction: Word) {}

    fn ioc(&mut self, instruction: Word) {}

    fn input(&mut self, instruction: Word) {}

    fn output(&mut self, instruction: Word) {}

    fn jred(&mut self, instruction: Word) {}

    fn handle_39(&mut self, instruction: Word) {}

    fn handle_40(&mut self, instruction: Word) {}

    fn handle_41(&mut self, instruction: Word) {}

    fn handle_42(&mut self, instruction: Word) {}

    fn handle_43(&mut self, instruction: Word) {}

    fn handle_44(&mut self, instruction: Word) {}

    fn handle_45(&mut self, instruction: Word) {}

    fn handle_46(&mut self, instruction: Word) {}

    fn handle_47(&mut self, instruction: Word) {}

    fn handle_48(&mut self, instruction: Word) {}

    fn handle_49(&mut self, instruction: Word) {}

    fn handle_50(&mut self, instruction: Word) {}

    fn handle_51(&mut self, instruction: Word) {}

    fn handle_52(&mut self, instruction: Word) {}

    fn handle_53(&mut self, instruction: Word) {}

    fn handle_54(&mut self, instruction: Word) {}

    fn handle_55(&mut self, instruction: Word) {}

    fn cmpa(&mut self, instruction: Word) {}

    fn cmp1(&mut self, instruction: Word) {}

    fn cmp2(&mut self, instruction: Word) {}

    fn cmp3(&mut self, instruction: Word) {}

    fn cmp4(&mut self, instruction: Word) {}

    fn cmp5(&mut self, instruction: Word) {}

    fn cmp6(&mut self, instruction: Word) {}

    fn cmpx(&mut self, instruction: Word) {}
}
