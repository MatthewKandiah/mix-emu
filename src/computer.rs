pub mod memory;
pub mod registers;

use crate::data_types::{Byte, FieldSpecification, Index, JumpAddress, Sign, Word};

use self::{memory::Memory, registers::Registers};

#[derive(Debug, PartialEq, Eq)]
pub enum ComparisonIndicatorState {
    EQUAL,
    GREATER,
    LESS,
}

pub struct TapeUnit {
    pub ready: bool,
    pub capacity: usize,
    pub data: Vec<[Word; 100]>,
}

impl TapeUnit {
    pub fn block_size(&self) -> usize {
        100
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    fn is_position_valid(&self, pos: usize) -> bool {
        pos < self.capacity()
    }

    pub fn new(capacity: usize) -> Self {
        Self {
            ready: true,
            capacity,
            data: vec![[Word::ZERO; 100]; capacity],
        }
    }

    pub fn read(&self, position: i32) -> [Word; 100] {
        let position: usize = position.try_into().unwrap();
        if !self.is_position_valid(position) {
            panic!("invalid read position");
        }
        self.data[position]
    }

    pub fn write(&mut self, position: i32, words: [Word; 100]) {
        let position: usize = position.try_into().unwrap();
        if !self.is_position_valid(position) {
            panic!("invalid write position");
        }
        self.data[position] = words;
    }
}

pub struct LinePrinter {}

impl LinePrinter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn write(&self, words: [Word; 24]) {
        let mut line = String::new();
        for word in words {
            line.push(word.bytes.0.to_char().unwrap());
            line.push(word.bytes.1.to_char().unwrap());
            line.push(word.bytes.2.to_char().unwrap());
            line.push(word.bytes.3.to_char().unwrap());
            line.push(word.bytes.4.to_char().unwrap());
        }
        println!("{}", line);
    }
}

pub struct Computer {
    pub current_instruction_address: i32,
    pub registers: Registers,
    pub memory: Memory,
    pub overflow: bool,
    pub comparison_indicator: Option<ComparisonIndicatorState>,
    pub running: bool,
    pub tape_unit: TapeUnit,
    pub line_printer: LinePrinter,
}

impl Computer {
    pub fn new() -> Self {
        Self {
            current_instruction_address: 0,
            registers: Registers::new(),
            overflow: false,
            comparison_indicator: None,
            memory: Memory::ZERO,
            running: false,
            tape_unit: TapeUnit::new(1000),
            line_printer: LinePrinter::new(),
        }
    }

    pub fn start(&mut self) {
        if self.running {
            panic!("cannot start when already running, consider using restart");
        }
        self.running = true;
        self.handle_next_instruction();
    }

    pub fn handle_next_instruction(&mut self) {
        if !self.running {
            return;
        }
        let current_instruction = self.memory.get(self.current_instruction_address).unwrap();
        self.current_instruction_address += 1;
        self.handle_instruction(current_instruction);
    }

    pub fn handle_instruction(&mut self, instruction: Word) {
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

    fn modified_address(&self, instruction: Word) -> i32 {
        let index_modifier = match instruction.index() {
            0 => 0,
            1 => self.registers.i1.to_i32(),
            2 => self.registers.i2.to_i32(),
            3 => self.registers.i3.to_i32(),
            4 => self.registers.i4.to_i32(),
            5 => self.registers.i5.to_i32(),
            6 => self.registers.i6.to_i32(),
            _ => panic!("Invalid index"),
        };
        instruction.address() + index_modifier
    }

    fn sign_to_load_or_store(
        field_specifier: &FieldSpecification,
        contents: Word,
        original_value: Sign,
    ) -> Sign {
        match field_specifier.inclusive_range().contains(&0) {
            true => contents.sign,
            false => original_value,
        }
    }

    fn bytes_to_load_word(
        field_specifier: &FieldSpecification,
        contents: Word,
    ) -> (Byte, Byte, Byte, Byte, Byte) {
        let mut bytes_to_load = Vec::<Byte>::new();
        match field_specifier.inclusive_range().contains(&1) {
            true => bytes_to_load.push(contents.bytes.0),
            false => bytes_to_load.insert(0, Byte::ZERO),
        };
        match field_specifier.inclusive_range().contains(&2) {
            true => bytes_to_load.push(contents.bytes.1),
            false => bytes_to_load.insert(0, Byte::ZERO),
        };
        match field_specifier.inclusive_range().contains(&3) {
            true => bytes_to_load.push(contents.bytes.2),
            false => bytes_to_load.insert(0, Byte::ZERO),
        };
        match field_specifier.inclusive_range().contains(&4) {
            true => bytes_to_load.push(contents.bytes.3),
            false => bytes_to_load.insert(0, Byte::ZERO),
        };
        match field_specifier.inclusive_range().contains(&5) {
            true => bytes_to_load.push(contents.bytes.4),
            false => bytes_to_load.insert(0, Byte::ZERO),
        };
        (
            bytes_to_load[0],
            bytes_to_load[1],
            bytes_to_load[2],
            bytes_to_load[3],
            bytes_to_load[4],
        )
    }

    fn bytes_to_load_index(field_specifier: &FieldSpecification, contents: Word) -> (Byte, Byte) {
        let bytes_for_word = Self::bytes_to_load_word(field_specifier, contents);
        if bytes_for_word.0 != Byte::ZERO
            || bytes_for_word.1 != Byte::ZERO
            || bytes_for_word.2 != Byte::ZERO
        {
            panic!("Attempting to load too many bytes into index register");
        }
        (bytes_for_word.3, bytes_for_word.4)
    }

    fn field_specifier_and_contents(&self, instruction: Word) -> (FieldSpecification, Word) {
        let address = self.modified_address(instruction);
        let contents = self.memory.get(address).unwrap();
        let field_specifier = instruction.field();
        if !field_specifier.is_valid() {
            panic!("illegal field specifier: {:?}", field_specifier);
        }
        (field_specifier, contents)
    }

    fn get_v(&self, instruction: Word) -> i32 {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        let needed_bytes = Self::bytes_to_load_word(&field_specifier, contents);
        contents.sign.value()
            * (needed_bytes.0.to_i32() * 64_i32.pow(4)
                + needed_bytes.1.to_i32() * 64_i32.pow(3)
                + needed_bytes.2.to_i32() * 64_i32.pow(2)
                + needed_bytes.3.to_i32() * 64
                + needed_bytes.4.to_i32())
    }

    fn add(&mut self, instruction: Word) {
        let result = self.registers.a.to_i32() + self.get_v(instruction);
        if result == 0 {
            self.registers.a.bytes = (Byte::ZERO, Byte::ZERO, Byte::ZERO, Byte::ZERO, Byte::ZERO);
            return;
        }
        if result / 64_i32.pow(5) != 0 {
            self.overflow = true;
        }
        self.registers.a = Word::from_i32(result % 64_i32.pow(5)).unwrap();
    }

    fn sub(&mut self, instruction: Word) {
        let result = self.registers.a.to_i32() - self.get_v(instruction);
        if result == 0 {
            self.registers.a.bytes = (Byte::ZERO, Byte::ZERO, Byte::ZERO, Byte::ZERO, Byte::ZERO);
            return;
        }
        if result / 64_i32.pow(5) != 0 {
            self.overflow = true;
        }
        self.registers.a = Word::from_i32(result % 64_i32.pow(5)).unwrap();
    }

    fn mul(&mut self, instruction: Word) {
        // Word::MAX * Word::MAX too large to store in i32
        let result: i64 = <i32 as Into<i64>>::into(self.registers.a.to_i32())
            * <i32 as Into<i64>>::into(self.get_v(instruction));
        let (_, contents) = self.field_specifier_and_contents(instruction);
        let result_sign = match self.registers.a.sign == contents.sign {
            true => Sign::PLUS,
            false => Sign::MINUS,
        };
        let x_value = result.abs() % 64_i64.pow(5);
        let full_a_value = result.abs() / 64_i64.pow(5);
        let a_value = full_a_value % 64_i64.pow(5);
        // multiplication cannot overflow, consider the largest possible absolute value
        // (64^5-1)*(64^5-1) < 64^10-1
        // so result can always fit in 10 bytes!
        self.registers.a = Word::from_i32(a_value.try_into().unwrap())
            .unwrap()
            .with_sign(result_sign);
        self.registers.x = Word::from_i32(x_value.try_into().unwrap())
            .unwrap()
            .with_sign(result_sign);
    }

    fn div(&mut self, instruction: Word) {
        let numerator: i64 = <i32 as Into<i64>>::into(self.registers.a.to_i32()) * 64_i64.pow(5)
            + <i32 as Into<i64>>::into(self.registers.x.to_i32());
        let v = self.get_v(instruction);
        if v == 0 {
            self.overflow = true;
            return;
        }

        let x_sign = self.registers.a.sign;
        let (_, contents) = self.field_specifier_and_contents(instruction);
        let a_sign = match self.registers.a.sign == contents.sign {
            true => Sign::PLUS,
            false => Sign::MINUS,
        };

        let a_result_value: i32 = match (numerator / <i32 as Into<i64>>::into(v)).try_into() {
            Ok(x) => x,
            Err(_) => {
                self.overflow = true;
                return;
            }
        };
        let a_result_word = match Word::from_i32(a_result_value) {
            Ok(x) => x,
            Err(_) => {
                self.overflow = true;
                return;
            }
        }
        .with_sign(a_sign);

        let x_result_value: i32 = match (numerator % <i32 as Into<i64>>::into(v)).try_into() {
            Ok(x) => x,
            Err(_) => {
                self.overflow = true;
                return;
            }
        };
        let x_result_word = match Word::from_i32(x_result_value) {
            Ok(x) => x,
            Err(_) => {
                self.overflow = true;
                return;
            }
        }
        .with_sign(x_sign);

        self.registers.x = x_result_word;
        self.registers.a = a_result_word;
    }

    fn handle_5(&mut self, instruction: Word) {
        match instruction.field().value() {
            0 => self.num(),
            1 => self.char(),
            2 => self.hlt(),
            _ => panic!("illegal field in code 5 instruction"),
        }
    }

    fn num(&mut self) {
        let sign = self.registers.a.sign;
        let bytes: [Byte; 10] = [
            self.registers.a.bytes.0,
            self.registers.a.bytes.1,
            self.registers.a.bytes.2,
            self.registers.a.bytes.3,
            self.registers.a.bytes.4,
            self.registers.x.bytes.0,
            self.registers.x.bytes.1,
            self.registers.x.bytes.2,
            self.registers.x.bytes.3,
            self.registers.x.bytes.4,
        ];
        let mut value = 0;
        for byte in bytes {
            value = value * 10 + byte.to_i32() % 10;
        }
        self.registers.a = Word::from_i32(value % 64_i32.pow(5))
            .unwrap()
            .with_sign(sign);
    }

    fn char(&mut self) {
        let mut value: i64 = self.registers.a.to_i32().abs().into();
        self.registers.x.bytes.4 = Byte::from_i32((value % 10 + 30).try_into().unwrap()).unwrap();
        value = value / 10;
        self.registers.x.bytes.3 = Byte::from_i32((value % 10 + 30).try_into().unwrap()).unwrap();
        value = value / 10;
        self.registers.x.bytes.2 = Byte::from_i32((value % 10 + 30).try_into().unwrap()).unwrap();
        value = value / 10;
        self.registers.x.bytes.1 = Byte::from_i32((value % 10 + 30).try_into().unwrap()).unwrap();
        value = value / 10;
        self.registers.x.bytes.0 = Byte::from_i32((value % 10 + 30).try_into().unwrap()).unwrap();
        value = value / 10;
        self.registers.a.bytes.4 = Byte::from_i32((value % 10 + 30).try_into().unwrap()).unwrap();
        value = value / 10;
        self.registers.a.bytes.3 = Byte::from_i32((value % 10 + 30).try_into().unwrap()).unwrap();
        value = value / 10;
        self.registers.a.bytes.2 = Byte::from_i32((value % 10 + 30).try_into().unwrap()).unwrap();
        value = value / 10;
        self.registers.a.bytes.1 = Byte::from_i32((value % 10 + 30).try_into().unwrap()).unwrap();
        value = value / 10;
        self.registers.a.bytes.0 = Byte::from_i32((value % 10 + 30).try_into().unwrap()).unwrap();
    }

    fn hlt(&mut self) {
        self.running = false;
    }

    fn handle_6(&mut self, instruction: Word) {
        match instruction.field().value() {
            0 => self.sla(instruction),
            1 => self.sra(instruction),
            2 => self.slax(instruction),
            3 => self.srax(instruction),
            4 => self.slc(instruction),
            5 => self.src(instruction),
            _ => panic!("illegal field in code 6 instruction"),
        }
    }

    fn sla(&mut self, instruction: Word) {
        let shift_count = i32::min(self.modified_address(instruction), 5);
        if shift_count < 0 {
            panic!("illegal - negative shift count");
        }
        let mut register_bytes = self.registers.a.bytes;
        for _ in 0..shift_count {
            register_bytes.0 = register_bytes.1;
            register_bytes.1 = register_bytes.2;
            register_bytes.2 = register_bytes.3;
            register_bytes.3 = register_bytes.4;
            register_bytes.4 = Byte::ZERO;
        }
        self.registers.a.bytes = register_bytes;
    }

    fn sra(&mut self, instruction: Word) {
        let shift_count = i32::min(self.modified_address(instruction), 5);
        if shift_count < 0 {
            panic!("illegal - negative shift count");
        }
        let mut register_bytes = self.registers.a.bytes;
        for _ in 0..shift_count {
            register_bytes.4 = register_bytes.3;
            register_bytes.3 = register_bytes.2;
            register_bytes.2 = register_bytes.1;
            register_bytes.1 = register_bytes.0;
            register_bytes.0 = Byte::ZERO;
        }
        self.registers.a.bytes = register_bytes;
    }

    fn slax(&mut self, instruction: Word) {
        let shift_count = i32::min(self.modified_address(instruction), 10);
        if shift_count < 0 {
            panic!("illegal - negative shift count");
        }
        let mut a_bytes = self.registers.a.bytes;
        let mut x_bytes = self.registers.x.bytes;
        for _ in 0..shift_count {
            a_bytes.0 = a_bytes.1;
            a_bytes.1 = a_bytes.2;
            a_bytes.2 = a_bytes.3;
            a_bytes.3 = a_bytes.4;
            a_bytes.4 = x_bytes.0;
            x_bytes.0 = x_bytes.1;
            x_bytes.1 = x_bytes.2;
            x_bytes.2 = x_bytes.3;
            x_bytes.3 = x_bytes.4;
            x_bytes.4 = Byte::ZERO;
        }
        self.registers.a.bytes = a_bytes;
        self.registers.x.bytes = x_bytes;
    }

    fn srax(&mut self, instruction: Word) {
        let shift_count = i32::min(self.modified_address(instruction), 10);
        if shift_count < 0 {
            panic!("illegal - negative shift count");
        }
        let mut a_bytes = self.registers.a.bytes;
        let mut x_bytes = self.registers.x.bytes;
        for _ in 0..shift_count {
            x_bytes.4 = x_bytes.3;
            x_bytes.3 = x_bytes.2;
            x_bytes.2 = x_bytes.1;
            x_bytes.1 = x_bytes.0;
            x_bytes.0 = a_bytes.4;
            a_bytes.4 = a_bytes.3;
            a_bytes.3 = a_bytes.2;
            a_bytes.2 = a_bytes.1;
            a_bytes.1 = a_bytes.0;
            a_bytes.0 = Byte::ZERO;
        }
        self.registers.a.bytes = a_bytes;
        self.registers.x.bytes = x_bytes;
    }

    fn slc(&mut self, instruction: Word) {
        let shift_count = self.modified_address(instruction) % 10;
        if shift_count < 0 {
            panic!("illegal - negative shift count");
        }
        let mut a_bytes = self.registers.a.bytes;
        let mut x_bytes = self.registers.x.bytes;
        for _ in 0..shift_count {
            let temp = a_bytes.0;
            a_bytes.0 = a_bytes.1;
            a_bytes.1 = a_bytes.2;
            a_bytes.2 = a_bytes.3;
            a_bytes.3 = a_bytes.4;
            a_bytes.4 = x_bytes.0;
            x_bytes.0 = x_bytes.1;
            x_bytes.1 = x_bytes.2;
            x_bytes.2 = x_bytes.3;
            x_bytes.3 = x_bytes.4;
            x_bytes.4 = temp;
        }
        self.registers.a.bytes = a_bytes;
        self.registers.x.bytes = x_bytes;
    }

    fn src(&mut self, instruction: Word) {
        let shift_count = self.modified_address(instruction) % 10;
        if shift_count < 0 {
            panic!("illegal - negative shift count");
        }
        let mut a_bytes = self.registers.a.bytes;
        let mut x_bytes = self.registers.x.bytes;
        for _ in 0..shift_count {
            let temp = x_bytes.4;
            x_bytes.4 = x_bytes.3;
            x_bytes.3 = x_bytes.2;
            x_bytes.2 = x_bytes.1;
            x_bytes.1 = x_bytes.0;
            x_bytes.0 = a_bytes.4;
            a_bytes.4 = a_bytes.3;
            a_bytes.3 = a_bytes.2;
            a_bytes.2 = a_bytes.1;
            a_bytes.1 = a_bytes.0;
            a_bytes.0 = temp;
        }
        self.registers.a.bytes = a_bytes;
        self.registers.x.bytes = x_bytes;
    }

    fn mov(&mut self, instruction: Word) {
        let copy_number = instruction.field().value();
        if copy_number == 0 {
            return;
        };
        let from_address = self.modified_address(instruction);
        let to_address = self.registers.i1.to_i32();
        for idx in 0..copy_number {
            let value = self.memory.get(from_address + idx).unwrap();
            self.memory.set(to_address + idx, value).unwrap();
        }
        self.registers.i1 = Index::from_i32(to_address + copy_number).unwrap();
    }

    fn lda(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.registers.a = Word {
            sign: Self::sign_to_load_or_store(&field_specifier, contents, self.registers.a.sign),
            bytes: Self::bytes_to_load_word(&field_specifier, contents),
        };
    }

    fn ld1(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.registers.i1 = Index {
            sign: Self::sign_to_load_or_store(&field_specifier, contents, self.registers.i1.sign),
            bytes: Self::bytes_to_load_index(&field_specifier, contents),
        };
    }

    fn ld2(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.registers.i2 = Index {
            sign: Self::sign_to_load_or_store(&field_specifier, contents, self.registers.i2.sign),
            bytes: Self::bytes_to_load_index(&field_specifier, contents),
        };
    }

    fn ld3(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.registers.i3 = Index {
            sign: Self::sign_to_load_or_store(&field_specifier, contents, self.registers.i3.sign),
            bytes: Self::bytes_to_load_index(&field_specifier, contents),
        };
    }

    fn ld4(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.registers.i4 = Index {
            sign: Self::sign_to_load_or_store(&field_specifier, contents, self.registers.i4.sign),
            bytes: Self::bytes_to_load_index(&field_specifier, contents),
        };
    }

    fn ld5(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.registers.i5 = Index {
            sign: Self::sign_to_load_or_store(&field_specifier, contents, self.registers.i5.sign),
            bytes: Self::bytes_to_load_index(&field_specifier, contents),
        };
    }

    fn ld6(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.registers.i6 = Index {
            sign: Self::sign_to_load_or_store(&field_specifier, contents, self.registers.i6.sign),
            bytes: Self::bytes_to_load_index(&field_specifier, contents),
        };
    }

    fn ldx(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.registers.x = Word {
            sign: Self::sign_to_load_or_store(&field_specifier, contents, self.registers.x.sign),
            bytes: Self::bytes_to_load_word(&field_specifier, contents),
        };
    }

    fn ldan(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.registers.a = Word {
            sign: Self::sign_to_load_or_store(&field_specifier, contents, self.registers.a.sign)
                .opposite(),
            bytes: Self::bytes_to_load_word(&field_specifier, contents),
        };
    }

    fn ld1n(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.registers.i1 = Index {
            sign: Self::sign_to_load_or_store(&field_specifier, contents, self.registers.i1.sign)
                .opposite(),
            bytes: Self::bytes_to_load_index(&field_specifier, contents),
        };
    }

    fn ld2n(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.registers.i2 = Index {
            sign: Self::sign_to_load_or_store(&field_specifier, contents, self.registers.i2.sign)
                .opposite(),
            bytes: Self::bytes_to_load_index(&field_specifier, contents),
        };
    }

    fn ld3n(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.registers.i3 = Index {
            sign: Self::sign_to_load_or_store(&field_specifier, contents, self.registers.i3.sign)
                .opposite(),
            bytes: Self::bytes_to_load_index(&field_specifier, contents),
        };
    }

    fn ld4n(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.registers.i4 = Index {
            sign: Self::sign_to_load_or_store(&field_specifier, contents, self.registers.i4.sign)
                .opposite(),
            bytes: Self::bytes_to_load_index(&field_specifier, contents),
        };
    }

    fn ld5n(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.registers.i5 = Index {
            sign: Self::sign_to_load_or_store(&field_specifier, contents, self.registers.i5.sign)
                .opposite(),
            bytes: Self::bytes_to_load_index(&field_specifier, contents),
        };
    }

    fn ld6n(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.registers.i6 = Index {
            sign: Self::sign_to_load_or_store(&field_specifier, contents, self.registers.i6.sign)
                .opposite(),
            bytes: Self::bytes_to_load_index(&field_specifier, contents),
        };
    }

    fn ldxn(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        self.registers.x = Word {
            sign: Self::sign_to_load_or_store(&field_specifier, contents, self.registers.x.sign)
                .opposite(),
            bytes: Self::bytes_to_load_word(&field_specifier, contents),
        };
    }

    fn bytes_to_store(
        memory_contents: Word,
        register_contents: Word,
        field_specifier: &FieldSpecification,
    ) -> (Byte, Byte, Byte, Byte, Byte) {
        let mut bytes_to_store = memory_contents.bytes;
        let mut number_of_bytes_altered = 0;
        let get_next_byte =
            |number_of_bytes_altered: i32, bytes: (Byte, Byte, Byte, Byte, Byte)| -> Byte {
                match number_of_bytes_altered {
                    0 => bytes.4,
                    1 => bytes.3,
                    2 => bytes.2,
                    3 => bytes.1,
                    4 => bytes.0,
                    _ => panic!("invalid byte access"),
                }
            };
        let mut next_byte = get_next_byte(number_of_bytes_altered, register_contents.bytes);

        if field_specifier.inclusive_range().contains(&5) {
            bytes_to_store.4 = next_byte;
            number_of_bytes_altered += 1;
            next_byte = get_next_byte(number_of_bytes_altered, register_contents.bytes);
        }
        if field_specifier.inclusive_range().contains(&4) {
            bytes_to_store.3 = next_byte;
            number_of_bytes_altered += 1;
            next_byte = get_next_byte(number_of_bytes_altered, register_contents.bytes);
        }
        if field_specifier.inclusive_range().contains(&3) {
            bytes_to_store.2 = next_byte;
            number_of_bytes_altered += 1;
            next_byte = get_next_byte(number_of_bytes_altered, register_contents.bytes);
        }
        if field_specifier.inclusive_range().contains(&2) {
            bytes_to_store.1 = next_byte;
            number_of_bytes_altered += 1;
            next_byte = get_next_byte(number_of_bytes_altered, register_contents.bytes);
        }
        if field_specifier.inclusive_range().contains(&1) {
            bytes_to_store.0 = next_byte;
        }
        bytes_to_store
    }

    fn sta(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        let original_sign = self
            .memory
            .get(self.modified_address(instruction))
            .unwrap()
            .sign;

        self.memory
            .set(
                self.modified_address(instruction),
                Word {
                    sign: Self::sign_to_load_or_store(
                        &field_specifier,
                        self.registers.a,
                        original_sign,
                    ),
                    bytes: Self::bytes_to_store(contents, self.registers.a, &field_specifier),
                },
            )
            .unwrap();
    }

    fn st1(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        let original_sign = self
            .memory
            .get(self.modified_address(instruction))
            .unwrap()
            .sign;
        let word_from_index = Word::from_i32(self.registers.i1.to_i32()).unwrap();

        self.memory
            .set(
                self.modified_address(instruction),
                Word {
                    sign: Self::sign_to_load_or_store(
                        &field_specifier,
                        word_from_index,
                        original_sign,
                    ),
                    bytes: Self::bytes_to_store(contents, word_from_index, &field_specifier),
                },
            )
            .unwrap();
    }

    fn st2(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        let original_sign = self
            .memory
            .get(self.modified_address(instruction))
            .unwrap()
            .sign;
        let word_from_index = Word::from_i32(self.registers.i2.to_i32()).unwrap();

        self.memory
            .set(
                self.modified_address(instruction),
                Word {
                    sign: Self::sign_to_load_or_store(
                        &field_specifier,
                        word_from_index,
                        original_sign,
                    ),
                    bytes: Self::bytes_to_store(contents, word_from_index, &field_specifier),
                },
            )
            .unwrap();
    }

    fn st3(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        let original_sign = self
            .memory
            .get(self.modified_address(instruction))
            .unwrap()
            .sign;
        let word_from_index = Word::from_i32(self.registers.i3.to_i32()).unwrap();

        self.memory
            .set(
                self.modified_address(instruction),
                Word {
                    sign: Self::sign_to_load_or_store(
                        &field_specifier,
                        word_from_index,
                        original_sign,
                    ),
                    bytes: Self::bytes_to_store(contents, word_from_index, &field_specifier),
                },
            )
            .unwrap();
    }

    fn st4(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        let original_sign = self
            .memory
            .get(self.modified_address(instruction))
            .unwrap()
            .sign;
        let word_from_index = Word::from_i32(self.registers.i4.to_i32()).unwrap();

        self.memory
            .set(
                self.modified_address(instruction),
                Word {
                    sign: Self::sign_to_load_or_store(
                        &field_specifier,
                        word_from_index,
                        original_sign,
                    ),
                    bytes: Self::bytes_to_store(contents, word_from_index, &field_specifier),
                },
            )
            .unwrap();
    }

    fn st5(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        let original_sign = self
            .memory
            .get(self.modified_address(instruction))
            .unwrap()
            .sign;
        let word_from_index = Word::from_i32(self.registers.i5.to_i32()).unwrap();

        self.memory
            .set(
                self.modified_address(instruction),
                Word {
                    sign: Self::sign_to_load_or_store(
                        &field_specifier,
                        word_from_index,
                        original_sign,
                    ),
                    bytes: Self::bytes_to_store(contents, word_from_index, &field_specifier),
                },
            )
            .unwrap();
    }

    fn st6(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        let original_sign = self
            .memory
            .get(self.modified_address(instruction))
            .unwrap()
            .sign;
        let word_from_index = Word::from_i32(self.registers.i6.to_i32()).unwrap();

        self.memory
            .set(
                self.modified_address(instruction),
                Word {
                    sign: Self::sign_to_load_or_store(
                        &field_specifier,
                        word_from_index,
                        original_sign,
                    ),
                    bytes: Self::bytes_to_store(contents, word_from_index, &field_specifier),
                },
            )
            .unwrap();
    }

    fn stx(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        let original_sign = self
            .memory
            .get(self.modified_address(instruction))
            .unwrap()
            .sign;

        self.memory
            .set(
                self.modified_address(instruction),
                Word {
                    sign: Self::sign_to_load_or_store(
                        &field_specifier,
                        self.registers.x,
                        original_sign,
                    ),
                    bytes: Self::bytes_to_store(contents, self.registers.x, &field_specifier),
                },
            )
            .unwrap();
    }

    fn stj(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        let original_sign = self
            .memory
            .get(self.modified_address(instruction))
            .unwrap()
            .sign;
        let word_from_jump_address = Word::from_i32(self.registers.j.to_i32()).unwrap();

        self.memory
            .set(
                self.modified_address(instruction),
                Word {
                    sign: Self::sign_to_load_or_store(
                        &field_specifier,
                        word_from_jump_address,
                        original_sign,
                    ),
                    bytes: Self::bytes_to_store(contents, word_from_jump_address, &field_specifier),
                },
            )
            .unwrap();
    }

    fn stz(&mut self, instruction: Word) {
        let (field_specifier, contents) = self.field_specifier_and_contents(instruction);
        let original_sign = self
            .memory
            .get(self.modified_address(instruction))
            .unwrap()
            .sign;

        self.memory
            .set(
                self.modified_address(instruction),
                Word {
                    sign: Self::sign_to_load_or_store(&field_specifier, Word::ZERO, original_sign),
                    bytes: Self::bytes_to_store(contents, Word::ZERO, &field_specifier),
                },
            )
            .unwrap();
    }

    fn jbus(&mut self, instruction: Word) {
        if !instruction.field().value() != 0 {
            panic!("illegal field in jbus instruction");
        }
        if self.tape_unit.ready {
            self.jmp(instruction);
        }
    }

    fn ioc(&mut self, _instruction: Word) {
        // do nothing - let's worry about these device specifics later
    }

    fn input(&mut self, instruction: Word) {
        let unit = instruction.field().value();
        if unit != 0 {
            panic!("illegal field value in IN instruction - I haven't implemented all the input devices yet!");
        }
        let target_device = &self.tape_unit;
        let read_words = target_device.read(self.registers.x.to_i32());
        for (idx, word) in read_words.iter().enumerate() {
            let idx: i32 = idx.try_into().unwrap();
            self.memory
                .set(self.modified_address(instruction) + idx, *word)
                .unwrap();
        }
    }

    fn output(&mut self, instruction: Word) {
        let unit = instruction.field().value();
        if unit != 0 && unit != 18 {
            panic!("illegal field value in IN instruction - I haven't implemented all the input devices yet!");
        }
        let modified_address = self.modified_address(instruction);
        if unit == 0 {
            let target_device = &mut self.tape_unit;
            let mut write_words: [Word; 100] = [Word::ZERO; 100];
            for i in 0..100 {
                let idx: i32 = i.try_into().unwrap();
                write_words[i] = self.memory.get(modified_address + idx).unwrap();
            }
            target_device.write(self.registers.x.to_i32(), write_words);
        } else if unit == 18 {
            let target_device = &self.line_printer;
            let mut write_words: [Word; 24] = [Word::ZERO; 24];
            for i in 0..24 {
                let idx: i32 = i.try_into().unwrap();
                write_words[i] = self.memory.get(modified_address + idx).unwrap()
            }
            target_device.write(write_words);
        }
    }

    fn jred(&mut self, instruction: Word) {
        if instruction.field().value() != 0 {
            panic!("illegal field value in jred");
        }
        if self.tape_unit.ready {
            self.jmp(instruction);
        }
    }

    fn handle_39(&mut self, instruction: Word) {
        match instruction.field().value() {
            0 => self.jmp(instruction),
            1 => self.jsj(instruction),
            2 => self.jov(instruction),
            3 => self.jnov(instruction),
            4 => self.jl(instruction),
            5 => self.je(instruction),
            6 => self.jg(instruction),
            7 => self.jge(instruction),
            8 => self.jne(instruction),
            9 => self.jle(instruction),
            _ => panic!("Illegal field in code 39 instruction"),
        };
    }

    fn jmp(&mut self, instruction: Word) {
        self.registers.j = JumpAddress::from_i32(self.current_instruction_address).unwrap();
        self.current_instruction_address = self.modified_address(instruction);
    }

    fn jsj(&mut self, instruction: Word) {
        self.current_instruction_address = self.modified_address(instruction);
    }

    fn jov(&mut self, instruction: Word) {
        if self.overflow {
            self.overflow = false;
            self.jmp(instruction);
        }
    }

    fn jnov(&mut self, instruction: Word) {
        if !self.overflow {
            self.jmp(instruction);
        } else {
            self.overflow = false;
        }
    }

    fn jl(&mut self, instruction: Word) {
        if self.comparison_indicator == Some(ComparisonIndicatorState::LESS) {
            self.jmp(instruction);
        }
    }

    fn je(&mut self, instruction: Word) {
        if self.comparison_indicator == Some(ComparisonIndicatorState::EQUAL) {
            self.jmp(instruction);
        }
    }

    fn jg(&mut self, instruction: Word) {
        if self.comparison_indicator == Some(ComparisonIndicatorState::GREATER) {
            self.jmp(instruction);
        }
    }

    fn jge(&mut self, instruction: Word) {
        if self.comparison_indicator == Some(ComparisonIndicatorState::GREATER)
            || self.comparison_indicator == Some(ComparisonIndicatorState::EQUAL)
        {
            self.jmp(instruction);
        }
    }

    fn jne(&mut self, instruction: Word) {
        if self.comparison_indicator == Some(ComparisonIndicatorState::GREATER)
            || self.comparison_indicator == Some(ComparisonIndicatorState::LESS)
        {
            self.jmp(instruction);
        }
    }

    fn jle(&mut self, instruction: Word) {
        if self.comparison_indicator == Some(ComparisonIndicatorState::LESS)
            || self.comparison_indicator == Some(ComparisonIndicatorState::EQUAL)
        {
            self.jmp(instruction);
        }
    }

    fn handle_40(&mut self, instruction: Word) {
        match instruction.field().value() {
            0 => self.jan(instruction),
            1 => self.jaz(instruction),
            2 => self.jap(instruction),
            3 => self.jann(instruction),
            4 => self.janz(instruction),
            5 => self.janp(instruction),
            _ => panic!("Illegal field for code 40 instruction)"),
        };
    }

    fn jan(&mut self, instruction: Word) {
        if self.registers.a.to_i32() < 0 {
            self.jmp(instruction);
        }
    }

    fn jaz(&mut self, instruction: Word) {
        if self.registers.a.to_i32() == 0 {
            self.jmp(instruction);
        }
    }

    fn jap(&mut self, instruction: Word) {
        if self.registers.a.to_i32() > 0 {
            self.jmp(instruction);
        }
    }

    fn jann(&mut self, instruction: Word) {
        if self.registers.a.to_i32() >= 0 {
            self.jmp(instruction);
        }
    }

    fn janz(&mut self, instruction: Word) {
        if self.registers.a.to_i32() != 0 {
            self.jmp(instruction);
        }
    }

    fn janp(&mut self, instruction: Word) {
        if self.registers.a.to_i32() <= 0 {
            self.jmp(instruction);
        }
    }

    fn handle_41(&mut self, instruction: Word) {
        match instruction.field().value() {
            0 => self.j1n(instruction),
            1 => self.j1z(instruction),
            2 => self.j1p(instruction),
            3 => self.j1nn(instruction),
            4 => self.j1nz(instruction),
            5 => self.j1np(instruction),
            _ => panic!("illegal field in code 41 instruction"),
        }
    }

    fn j1n(&mut self, instruction: Word) {
        if self.registers.i1.to_i32() < 0 {
            self.jmp(instruction);
        }
    }

    fn j1z(&mut self, instruction: Word) {
        if self.registers.i1.to_i32() == 0 {
            self.jmp(instruction);
        }
    }

    fn j1p(&mut self, instruction: Word) {
        if self.registers.i1.to_i32() > 0 {
            self.jmp(instruction);
        }
    }

    fn j1nn(&mut self, instruction: Word) {
        if self.registers.i1.to_i32() >= 0 {
            self.jmp(instruction);
        }
    }

    fn j1nz(&mut self, instruction: Word) {
        if self.registers.i1.to_i32() != 0 {
            self.jmp(instruction);
        }
    }

    fn j1np(&mut self, instruction: Word) {
        if self.registers.i1.to_i32() <= 0 {
            self.jmp(instruction);
        }
    }

    fn handle_42(&mut self, instruction: Word) {
        match instruction.field().value() {
            0 => self.j2n(instruction),
            1 => self.j2z(instruction),
            2 => self.j2p(instruction),
            3 => self.j2nn(instruction),
            4 => self.j2nz(instruction),
            5 => self.j2np(instruction),
            _ => panic!("illegal field in code 42 instruction"),
        }
    }

    fn j2n(&mut self, instruction: Word) {
        if self.registers.i2.to_i32() < 0 {
            self.jmp(instruction);
        }
    }

    fn j2z(&mut self, instruction: Word) {
        if self.registers.i2.to_i32() == 0 {
            self.jmp(instruction);
        }
    }

    fn j2p(&mut self, instruction: Word) {
        if self.registers.i2.to_i32() > 0 {
            self.jmp(instruction);
        }
    }

    fn j2nn(&mut self, instruction: Word) {
        if self.registers.i2.to_i32() >= 0 {
            self.jmp(instruction);
        }
    }

    fn j2nz(&mut self, instruction: Word) {
        if self.registers.i2.to_i32() != 0 {
            self.jmp(instruction);
        }
    }

    fn j2np(&mut self, instruction: Word) {
        if self.registers.i2.to_i32() <= 0 {
            self.jmp(instruction);
        }
    }

    fn handle_43(&mut self, instruction: Word) {
        match instruction.field().value() {
            0 => self.j3n(instruction),
            1 => self.j3z(instruction),
            2 => self.j3p(instruction),
            3 => self.j3nn(instruction),
            4 => self.j3nz(instruction),
            5 => self.j3np(instruction),
            _ => panic!("illegal field in code 43 instruction"),
        }
    }

    fn j3n(&mut self, instruction: Word) {
        if self.registers.i3.to_i32() < 0 {
            self.jmp(instruction);
        }
    }

    fn j3z(&mut self, instruction: Word) {
        if self.registers.i3.to_i32() == 0 {
            self.jmp(instruction);
        }
    }

    fn j3p(&mut self, instruction: Word) {
        if self.registers.i3.to_i32() > 0 {
            self.jmp(instruction);
        }
    }

    fn j3nn(&mut self, instruction: Word) {
        if self.registers.i3.to_i32() >= 0 {
            self.jmp(instruction);
        }
    }

    fn j3nz(&mut self, instruction: Word) {
        if self.registers.i3.to_i32() != 0 {
            self.jmp(instruction);
        }
    }

    fn j3np(&mut self, instruction: Word) {
        if self.registers.i3.to_i32() <= 0 {
            self.jmp(instruction);
        }
    }

    fn handle_44(&mut self, instruction: Word) {
        match instruction.field().value() {
            0 => self.j4n(instruction),
            1 => self.j4z(instruction),
            2 => self.j4p(instruction),
            3 => self.j4nn(instruction),
            4 => self.j4nz(instruction),
            5 => self.j4np(instruction),
            _ => panic!("illegal field in code 44 instruction"),
        }
    }

    fn j4n(&mut self, instruction: Word) {
        if self.registers.i4.to_i32() < 0 {
            self.jmp(instruction);
        }
    }

    fn j4z(&mut self, instruction: Word) {
        if self.registers.i4.to_i32() == 0 {
            self.jmp(instruction);
        }
    }

    fn j4p(&mut self, instruction: Word) {
        if self.registers.i4.to_i32() > 0 {
            self.jmp(instruction);
        }
    }

    fn j4nn(&mut self, instruction: Word) {
        if self.registers.i4.to_i32() >= 0 {
            self.jmp(instruction);
        }
    }

    fn j4nz(&mut self, instruction: Word) {
        if self.registers.i4.to_i32() != 0 {
            self.jmp(instruction);
        }
    }

    fn j4np(&mut self, instruction: Word) {
        if self.registers.i4.to_i32() <= 0 {
            self.jmp(instruction);
        }
    }

    fn handle_45(&mut self, instruction: Word) {
        match instruction.field().value() {
            0 => self.j5n(instruction),
            1 => self.j5z(instruction),
            2 => self.j5p(instruction),
            3 => self.j5nn(instruction),
            4 => self.j5nz(instruction),
            5 => self.j5np(instruction),
            _ => panic!("illegal field in code 45 instruction"),
        }
    }

    fn j5n(&mut self, instruction: Word) {
        if self.registers.i5.to_i32() < 0 {
            self.jmp(instruction);
        }
    }

    fn j5z(&mut self, instruction: Word) {
        if self.registers.i5.to_i32() == 0 {
            self.jmp(instruction);
        }
    }

    fn j5p(&mut self, instruction: Word) {
        if self.registers.i5.to_i32() > 0 {
            self.jmp(instruction);
        }
    }

    fn j5nn(&mut self, instruction: Word) {
        if self.registers.i5.to_i32() >= 0 {
            self.jmp(instruction);
        }
    }

    fn j5nz(&mut self, instruction: Word) {
        if self.registers.i5.to_i32() != 0 {
            self.jmp(instruction);
        }
    }

    fn j5np(&mut self, instruction: Word) {
        if self.registers.i5.to_i32() <= 0 {
            self.jmp(instruction);
        }
    }

    fn handle_46(&mut self, instruction: Word) {
        match instruction.field().value() {
            0 => self.j6n(instruction),
            1 => self.j6z(instruction),
            2 => self.j6p(instruction),
            3 => self.j6nn(instruction),
            4 => self.j6nz(instruction),
            5 => self.j6np(instruction),
            _ => panic!("illegal field in code 46 instruction"),
        }
    }

    fn j6n(&mut self, instruction: Word) {
        if self.registers.i6.to_i32() < 0 {
            self.jmp(instruction);
        }
    }

    fn j6z(&mut self, instruction: Word) {
        if self.registers.i6.to_i32() == 0 {
            self.jmp(instruction);
        }
    }

    fn j6p(&mut self, instruction: Word) {
        if self.registers.i6.to_i32() > 0 {
            self.jmp(instruction);
        }
    }

    fn j6nn(&mut self, instruction: Word) {
        if self.registers.i6.to_i32() >= 0 {
            self.jmp(instruction);
        }
    }

    fn j6nz(&mut self, instruction: Word) {
        if self.registers.i6.to_i32() != 0 {
            self.jmp(instruction);
        }
    }

    fn j6np(&mut self, instruction: Word) {
        if self.registers.i6.to_i32() <= 0 {
            self.jmp(instruction);
        }
    }

    fn handle_47(&mut self, instruction: Word) {
        match instruction.field().value() {
            0 => self.jxn(instruction),
            1 => self.jxz(instruction),
            2 => self.jxp(instruction),
            3 => self.jxnn(instruction),
            4 => self.jxnz(instruction),
            5 => self.jxnp(instruction),
            _ => panic!("illegal field in code 47 instruction"),
        }
    }

    fn jxn(&mut self, instruction: Word) {
        if self.registers.x.to_i32() < 0 {
            self.jmp(instruction);
        }
    }

    fn jxz(&mut self, instruction: Word) {
        if self.registers.x.to_i32() == 0 {
            self.jmp(instruction);
        }
    }

    fn jxp(&mut self, instruction: Word) {
        if self.registers.x.to_i32() > 0 {
            self.jmp(instruction);
        }
    }

    fn jxnn(&mut self, instruction: Word) {
        if self.registers.x.to_i32() >= 0 {
            self.jmp(instruction);
        }
    }

    fn jxnz(&mut self, instruction: Word) {
        if self.registers.x.to_i32() != 0 {
            self.jmp(instruction);
        }
    }

    fn jxnp(&mut self, instruction: Word) {
        if self.registers.x.to_i32() <= 0 {
            self.jmp(instruction);
        }
    }

    fn word_to_enter(&self, instruction: Word) -> Word {
        let mut m = Word::from_i32(self.modified_address(instruction)).unwrap();
        if m.to_i32() == 0 {
            m = m.with_sign(instruction.sign);
        }
        m
    }

    fn index_to_enter(&self, instruction: Word) -> Index {
        let mut m = Index::from_i32(self.modified_address(instruction)).unwrap();
        if m.to_i32() == 0 {
            m = m.with_sign(instruction.sign);
        }
        m
    }

    fn handle_48(&mut self, instruction: Word) {
        match instruction.field().value() {
            0 => self.inca(instruction),
            1 => self.deca(instruction),
            2 => self.enta(instruction),
            3 => self.enna(instruction),
            _ => panic!("Illegal field in code 48 instruction"),
        };
    }

    fn inca(&mut self, instruction: Word) {
        let result = self.registers.a.to_i32() + self.modified_address(instruction);
        if result == 0 {
            self.registers.a.bytes = (Byte::ZERO, Byte::ZERO, Byte::ZERO, Byte::ZERO, Byte::ZERO);
            return;
        }
        if result / 64_i32.pow(5) != 0 {
            self.overflow = true;
        }
        self.registers.a = Word::from_i32(result % 64_i32.pow(5)).unwrap();
    }

    fn deca(&mut self, instruction: Word) {
        self.inca(instruction.with_opposite_sign());
    }

    fn enta(&mut self, instruction: Word) {
        self.registers.a = self.word_to_enter(instruction);
    }

    fn enna(&mut self, instruction: Word) {
        self.registers.a = self.word_to_enter(instruction).with_opposite_sign();
    }

    fn handle_49(&mut self, instruction: Word) {
        match instruction.field().value() {
            0 => self.inc1(instruction),
            1 => self.dec1(instruction),
            2 => self.ent1(instruction),
            3 => self.enn1(instruction),
            _ => panic!("Illegal field in code 49 instruction"),
        }
    }

    fn inc1(&mut self, instruction: Word) {
        let result = self.registers.i1.to_i32() + self.modified_address(instruction);
        if result == 0 {
            self.registers.i1.bytes = (Byte::ZERO, Byte::ZERO);
            return;
        }
        self.registers.i1 = Index::from_i32(result).unwrap();
    }

    fn dec1(&mut self, instruction: Word) {
        self.inc1(instruction.with_opposite_sign());
    }

    fn ent1(&mut self, instruction: Word) {
        self.registers.i1 = self.index_to_enter(instruction);
    }

    fn enn1(&mut self, instruction: Word) {
        self.registers.i1 = self.index_to_enter(instruction).with_opposite_sign();
    }

    fn handle_50(&mut self, instruction: Word) {
        match instruction.field().value() {
            0 => self.inc2(instruction),
            1 => self.dec2(instruction),
            2 => self.ent2(instruction),
            3 => self.enn2(instruction),
            _ => panic!("Illegal field in code 50 instruction"),
        };
    }

    fn inc2(&mut self, instruction: Word) {
        let result = self.registers.i2.to_i32() + self.modified_address(instruction);
        if result == 0 {
            self.registers.i2.bytes = (Byte::ZERO, Byte::ZERO);
            return;
        }
        self.registers.i2 = Index::from_i32(result).unwrap();
    }

    fn dec2(&mut self, instruction: Word) {
        self.inc2(instruction.with_opposite_sign());
    }

    fn ent2(&mut self, instruction: Word) {
        self.registers.i2 = self.index_to_enter(instruction);
    }

    fn enn2(&mut self, instruction: Word) {
        self.registers.i2 = self.index_to_enter(instruction).with_opposite_sign();
    }

    fn handle_51(&mut self, instruction: Word) {
        match instruction.field().value() {
            0 => self.inc3(instruction),
            1 => self.dec3(instruction),
            2 => self.ent3(instruction),
            3 => self.enn3(instruction),
            _ => panic!("Illegal field in code 51 instruction"),
        };
    }

    fn inc3(&mut self, instruction: Word) {
        let result = self.registers.i3.to_i32() + self.modified_address(instruction);
        if result == 0 {
            self.registers.i3.bytes = (Byte::ZERO, Byte::ZERO);
            return;
        }
        self.registers.i3 = Index::from_i32(result).unwrap();
    }

    fn dec3(&mut self, instruction: Word) {
        self.inc3(instruction.with_opposite_sign());
    }

    fn ent3(&mut self, instruction: Word) {
        self.registers.i3 = self.index_to_enter(instruction);
    }

    fn enn3(&mut self, instruction: Word) {
        self.registers.i3 = self.index_to_enter(instruction).with_opposite_sign();
    }

    fn handle_52(&mut self, instruction: Word) {
        match instruction.field().value() {
            0 => self.inc4(instruction),
            1 => self.dec4(instruction),
            2 => self.ent4(instruction),
            3 => self.enn4(instruction),
            _ => panic!("Illegal field in code 52 instruction"),
        };
    }

    fn inc4(&mut self, instruction: Word) {
        let result = self.registers.i4.to_i32() + self.modified_address(instruction);
        if result == 0 {
            self.registers.i4.bytes = (Byte::ZERO, Byte::ZERO);
            return;
        }
        self.registers.i4 = Index::from_i32(result).unwrap();
    }

    fn dec4(&mut self, instruction: Word) {
        self.inc4(instruction.with_opposite_sign());
    }

    fn ent4(&mut self, instruction: Word) {
        self.registers.i4 = self.index_to_enter(instruction);
    }

    fn enn4(&mut self, instruction: Word) {
        self.registers.i4 = self.index_to_enter(instruction).with_opposite_sign();
    }

    fn handle_53(&mut self, instruction: Word) {
        match instruction.field().value() {
            0 => self.inc5(instruction),
            1 => self.dec5(instruction),
            2 => self.ent5(instruction),
            3 => self.enn5(instruction),
            _ => panic!("Illegal field in code 53 instruction"),
        };
    }

    fn inc5(&mut self, instruction: Word) {
        let result = self.registers.i5.to_i32() + self.modified_address(instruction);
        if result == 0 {
            self.registers.i5.bytes = (Byte::ZERO, Byte::ZERO);
            return;
        }
        self.registers.i5 = Index::from_i32(result).unwrap();
    }

    fn dec5(&mut self, instruction: Word) {
        self.inc5(instruction.with_opposite_sign());
    }

    fn ent5(&mut self, instruction: Word) {
        self.registers.i5 = self.index_to_enter(instruction);
    }

    fn enn5(&mut self, instruction: Word) {
        self.registers.i5 = self.index_to_enter(instruction).with_opposite_sign();
    }

    fn handle_54(&mut self, instruction: Word) {
        match instruction.field().value() {
            0 => self.inc6(instruction),
            1 => self.dec6(instruction),
            2 => self.ent6(instruction),
            3 => self.enn6(instruction),
            _ => panic!("Illegal field in code 54 instruction"),
        };
    }

    fn inc6(&mut self, instruction: Word) {
        let result = self.registers.i6.to_i32() + self.modified_address(instruction);
        if result == 0 {
            self.registers.i6.bytes = (Byte::ZERO, Byte::ZERO);
            return;
        }
        self.registers.i6 = Index::from_i32(result).unwrap();
    }

    fn dec6(&mut self, instruction: Word) {
        self.inc6(instruction.with_opposite_sign());
    }

    fn ent6(&mut self, instruction: Word) {
        self.registers.i6 = self.index_to_enter(instruction);
    }

    fn enn6(&mut self, instruction: Word) {
        self.registers.i6 = self.index_to_enter(instruction).with_opposite_sign();
    }

    fn handle_55(&mut self, instruction: Word) {
        match instruction.field().value() {
            0 => self.incx(instruction),
            1 => self.decx(instruction),
            2 => self.entx(instruction),
            3 => self.ennx(instruction),
            _ => panic!("Illegal field in code 55 instruction"),
        }
    }

    fn incx(&mut self, instruction: Word) {
        let result = self.registers.x.to_i32() + self.modified_address(instruction);
        if result == 0 {
            self.registers.x.bytes = (Byte::ZERO, Byte::ZERO, Byte::ZERO, Byte::ZERO, Byte::ZERO);
            return;
        }
        if result / 64_i32.pow(5) != 0 {
            self.overflow = true;
        }
        self.registers.x = Word::from_i32(result % 64_i32.pow(5)).unwrap();
    }

    fn decx(&mut self, instruction: Word) {
        self.incx(instruction.with_opposite_sign());
    }

    fn entx(&mut self, instruction: Word) {
        self.registers.x = self.word_to_enter(instruction);
    }

    fn ennx(&mut self, instruction: Word) {
        self.registers.x = self.word_to_enter(instruction).with_opposite_sign();
    }

    fn do_comparison(&mut self, register_word: Word, memory_word: Word) {
        self.comparison_indicator = Some(match register_word.to_i32() - memory_word.to_i32() {
            ..=-1 => ComparisonIndicatorState::LESS,
            0 => ComparisonIndicatorState::EQUAL,
            1.. => ComparisonIndicatorState::GREATER,
        });
    }

    fn cmpa(&mut self, instruction: Word) {
        let (field_specifier, memory_contents) = self.field_specifier_and_contents(instruction);
        let register_word = Word {
            sign: Self::sign_to_load_or_store(&field_specifier, self.registers.a, Sign::PLUS),
            bytes: Self::bytes_to_load_word(&field_specifier, self.registers.a),
        };
        let memory_word = Word {
            sign: Self::sign_to_load_or_store(&field_specifier, memory_contents, Sign::PLUS),
            bytes: Self::bytes_to_load_word(&field_specifier, memory_contents),
        };
        self.do_comparison(register_word, memory_word);
    }

    fn cmp1(&mut self, instruction: Word) {
        let (field_specifier, memory_contents) = self.field_specifier_and_contents(instruction);
        let memory_word = Word {
            sign: Self::sign_to_load_or_store(&field_specifier, memory_contents, Sign::PLUS),
            bytes: Self::bytes_to_load_word(&field_specifier, memory_contents),
        };
        let index_value_as_word = Word::from_i32(self.registers.i1.to_i32()).unwrap();
        let register_word = Word {
            sign: Self::sign_to_load_or_store(&field_specifier, index_value_as_word, Sign::PLUS),
            bytes: Self::bytes_to_load_word(&field_specifier, index_value_as_word),
        };
        self.do_comparison(register_word, memory_word);
    }

    fn cmp2(&mut self, instruction: Word) {
        let (field_specifier, memory_contents) = self.field_specifier_and_contents(instruction);
        let memory_word = Word {
            sign: Self::sign_to_load_or_store(&field_specifier, memory_contents, Sign::PLUS),
            bytes: Self::bytes_to_load_word(&field_specifier, memory_contents),
        };
        let index_value_as_word = Word::from_i32(self.registers.i2.to_i32()).unwrap();
        let register_word = Word {
            sign: Self::sign_to_load_or_store(&field_specifier, index_value_as_word, Sign::PLUS),
            bytes: Self::bytes_to_load_word(&field_specifier, index_value_as_word),
        };
        self.do_comparison(register_word, memory_word);
    }

    fn cmp3(&mut self, instruction: Word) {
        let (field_specifier, memory_contents) = self.field_specifier_and_contents(instruction);
        let memory_word = Word {
            sign: Self::sign_to_load_or_store(&field_specifier, memory_contents, Sign::PLUS),
            bytes: Self::bytes_to_load_word(&field_specifier, memory_contents),
        };
        let index_value_as_word = Word::from_i32(self.registers.i3.to_i32()).unwrap();
        let register_word = Word {
            sign: Self::sign_to_load_or_store(&field_specifier, index_value_as_word, Sign::PLUS),
            bytes: Self::bytes_to_load_word(&field_specifier, index_value_as_word),
        };
        self.do_comparison(register_word, memory_word);
    }

    fn cmp4(&mut self, instruction: Word) {
        let (field_specifier, memory_contents) = self.field_specifier_and_contents(instruction);
        let memory_word = Word {
            sign: Self::sign_to_load_or_store(&field_specifier, memory_contents, Sign::PLUS),
            bytes: Self::bytes_to_load_word(&field_specifier, memory_contents),
        };
        let index_value_as_word = Word::from_i32(self.registers.i4.to_i32()).unwrap();
        let register_word = Word {
            sign: Self::sign_to_load_or_store(&field_specifier, index_value_as_word, Sign::PLUS),
            bytes: Self::bytes_to_load_word(&field_specifier, index_value_as_word),
        };
        self.do_comparison(register_word, memory_word);
    }

    fn cmp5(&mut self, instruction: Word) {
        let (field_specifier, memory_contents) = self.field_specifier_and_contents(instruction);
        let memory_word = Word {
            sign: Self::sign_to_load_or_store(&field_specifier, memory_contents, Sign::PLUS),
            bytes: Self::bytes_to_load_word(&field_specifier, memory_contents),
        };
        let index_value_as_word = Word::from_i32(self.registers.i5.to_i32()).unwrap();
        let register_word = Word {
            sign: Self::sign_to_load_or_store(&field_specifier, index_value_as_word, Sign::PLUS),
            bytes: Self::bytes_to_load_word(&field_specifier, index_value_as_word),
        };
        self.do_comparison(register_word, memory_word);
    }

    fn cmp6(&mut self, instruction: Word) {
        let (field_specifier, memory_contents) = self.field_specifier_and_contents(instruction);
        let memory_word = Word {
            sign: Self::sign_to_load_or_store(&field_specifier, memory_contents, Sign::PLUS),
            bytes: Self::bytes_to_load_word(&field_specifier, memory_contents),
        };
        let index_value_as_word = Word::from_i32(self.registers.i6.to_i32()).unwrap();
        let register_word = Word {
            sign: Self::sign_to_load_or_store(&field_specifier, index_value_as_word, Sign::PLUS),
            bytes: Self::bytes_to_load_word(&field_specifier, index_value_as_word),
        };
        self.do_comparison(register_word, memory_word);
    }

    fn cmpx(&mut self, instruction: Word) {
        let (field_specifier, memory_contents) = self.field_specifier_and_contents(instruction);
        let register_word = Word {
            sign: Self::sign_to_load_or_store(&field_specifier, self.registers.x, Sign::PLUS),
            bytes: Self::bytes_to_load_word(&field_specifier, self.registers.x),
        };
        let memory_word = Word {
            sign: Self::sign_to_load_or_store(&field_specifier, memory_contents, Sign::PLUS),
            bytes: Self::bytes_to_load_word(&field_specifier, memory_contents),
        };
        self.do_comparison(register_word, memory_word);
    }
}
