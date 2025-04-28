use crate::Operation::*;
use crate::{
    data::{Address, Nibble, OpCode, RegisterIndex, Word},
    image::Image,
    platform::{Platform, Point, Sprite},
    Error, Offset, Result,
};

////////////////////////////////////////////////////////////////////////////////
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
pub const FLAG_REGISTER: usize = 0xf;
pub const FONT_ADDRESS: Address = Address::new(0x0);
pub const FONT_HEIGHT: Offset = 5;
pub const FONT_SPRITES: [u8; 16 * FONT_HEIGHT as usize] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

////////////////////////////////////////////////////////////////////////////////

pub struct Interpreter<P: Platform> {
    platform: P,
    register: [u8; 16],
    index: Address,
    memory: [u8; 4096],
    instruction_counter: Address,
    stack: [Address; 16],
    stack_top_index: usize,
    is_first_wait: bool,
}

impl<P: Platform> Interpreter<P> {
    pub fn new(image: impl Image, platform: P) -> Self {
        let mut memory = [0; 4096];
        image.load_into_memory(&mut memory);

        Self {
            platform,
            register: [0; 16],
            index: Default::default(),
            memory,
            instruction_counter: image.entry_point(),
            stack: [Address::new(0); 16],
            stack_top_index: 0,
            is_first_wait: true,
        }
    }

    pub fn platform(&self) -> &P {
        &self.platform
    }

    pub fn platform_mut(&mut self) -> &mut P {
        &mut self.platform
    }

    fn push_to_stack(&mut self, address: Address) {
        self.stack[self.stack_top_index] = address;
        self.stack_top_index += 1;
    }

    fn pop_from_stack(&mut self) -> Address {
        self.stack_top_index -= 1;
        let addr = self.stack[self.stack_top_index];
        self.stack[self.stack_top_index] = Address::new(0);
        addr
    }

    pub fn run_next_instruction(&mut self) -> Result<()> {
        let i = self.instruction_counter;
        let code: OpCode =
            OpCode::from_bytes(self.memory[i.as_usize()], self.memory[i.as_usize() + 1]);

        if let Ok(op) = Operation::try_from(code) {
            match op {
                ClearScreen => {
                    self.platform.clear_screen();
                }
                SetRegister(x, y) => {
                    self.register[x.as_usize()] = y;
                }
                SetIndexRegister(address) => {
                    self.index = address;
                }
                AddValue(x, word) => {
                    self.register[x.as_usize()] =
                        self.register[x.as_usize()].overflowing_add(word).0;
                }
                SkipIfRegistersEqual(x, y) => {
                    if self.register[x.as_usize()] == self.register[y.as_usize()] {
                        self.instruction_counter += 2;
                    }
                }
                SkipIfRegistersNotEqual(x, y) => {
                    if self.register[x.as_usize()] != self.register[y.as_usize()] {
                        self.instruction_counter += 2;
                    }
                }
                SkipIfEqual(x, word) => {
                    if self.register[x.as_usize()] == word {
                        self.instruction_counter += 2;
                    }
                }
                SkipIfNotEqual(x, word) => {
                    if self.register[x.as_usize()] != word {
                        self.instruction_counter += 2;
                    }
                }
                SetToRegister(x, y) => {
                    self.register[x.as_usize()] = self.register[y.as_usize()];
                }
                Or(x, y) => {
                    self.register[x.as_usize()] |= self.register[y.as_usize()];
                    self.register[FLAG_REGISTER] = 0;
                }
                Xor(x, y) => {
                    self.register[x.as_usize()] ^= self.register[y.as_usize()];
                    self.register[FLAG_REGISTER] = 0;
                }
                And(x, y) => {
                    self.register[x.as_usize()] &= self.register[y.as_usize()];
                    self.register[FLAG_REGISTER] = 0;
                }
                AddRegister(x, y) => {
                    let res =
                        self.register[x.as_usize()].overflowing_add(self.register[y.as_usize()]);
                    self.register[x.as_usize()] = res.0;
                    self.register[FLAG_REGISTER] = res.1 as u8;
                }
                SubRegister(x, y) => {
                    let res =
                        self.register[x.as_usize()].overflowing_sub(self.register[y.as_usize()]);
                    self.register[x.as_usize()] = res.0;
                    self.register[FLAG_REGISTER] = !res.1 as u8;
                }
                SubRegisterReversed(x, y) => {
                    let res =
                        self.register[y.as_usize()].overflowing_sub(self.register[x.as_usize()]);
                    self.register[x.as_usize()] = res.0;
                    self.register[FLAG_REGISTER] = !res.1 as u8;
                }
                ShiftRight(x, y) => {
                    let res = self.register[y.as_usize()] % 2;
                    self.register[x.as_usize()] = self.register[y.as_usize()].overflowing_shr(1).0;
                    self.register[FLAG_REGISTER] = res;
                }
                ShiftLeft(x, y) => {
                    let res = self.register[y.as_usize()] >> 7;
                    self.register[x.as_usize()] = self.register[y.as_usize()].overflowing_shl(1).0;
                    self.register[FLAG_REGISTER] = res;
                }
                WriteMemory(x) => {
                    let ind = self.index.as_usize();
                    for i in ind..=ind + x.as_usize() {
                        self.memory[i] = self.register[i - ind];
                    }
                    self.index += x.as_offset() + 1;
                }
                ReadMemory(x) => {
                    let ind = self.index.as_usize();
                    for i in ind..=ind + x.as_usize() {
                        self.register[i - ind] = self.memory[i];
                    }
                    self.index += x.as_offset() + 1;
                }
                Call(address) => {
                    self.push_to_stack(self.instruction_counter);
                    self.instruction_counter = address;
                    return Ok(());
                }
                Return => {
                    self.instruction_counter = self.pop_from_stack();
                }
                Jump(address) => {
                    self.instruction_counter = address;
                    return Ok(());
                }
                ToDecimal(x) => {
                    let value = self.register[x.as_usize()];
                    self.memory[self.index.as_usize()] = value / 100;
                    self.memory[self.index.as_usize() + 1] = (value / 10) % 10;
                    self.memory[self.index.as_usize() + 2] = value % 10;
                }
                IncrementIndexRegister(x) => {
                    self.index += self.register[x.as_usize()] as Offset;
                }
                Draw(x, y, n) => {
                    let point = Point {
                        x: self.register[x.as_usize()],
                        y: self.register[y.as_usize()],
                    };
                    let sprite = Sprite::new(
                        &self.memory[self.index.as_usize()..self.index.as_usize() + n.as_usize()],
                    );
                    self.register[FLAG_REGISTER] =
                        u8::from(self.platform.draw_sprite(point, sprite));
                }
                SkipIfKeyDown(x) => {
                    if self
                        .platform
                        .is_key_down(Nibble::try_from(self.register[x.as_usize()]).unwrap())
                    {
                        self.instruction_counter += 2;
                    }
                }
                SkipIfKeyUp(x) => {
                    if !self
                        .platform
                        .is_key_down(Nibble::try_from(self.register[x.as_usize()]).unwrap())
                    {
                        self.instruction_counter += 2;
                    }
                }

                GetDelayTimer(x) => {
                    self.register[x.as_usize()] = self.platform.get_delay_timer();
                }
                SetDelayTimer(x) => {
                    let timer = self.register[x.as_usize()];
                    self.platform_mut().set_delay_timer(timer);
                }
                SetSoundTimer(x) => {
                    let timer = self.register[x.as_usize()];
                    self.platform_mut().set_sound_timer(timer);
                }
                JumpV0(address) => {
                    self.instruction_counter = address + self.register[0] as Offset;
                    return Ok(());
                }
                WaitForKey(x) => {
                    if self.is_first_wait {
                        self.platform.consume_key_press();
                        self.is_first_wait = false;
                        self.register[FLAG_REGISTER] = 16;
                    }

                    match self.platform.consume_key_press() {
                        Some(key) => {
                            if self.register[x.as_usize()] == 16 {
                                self.register[x.as_usize()] = key.as_u8();
                            }
                        }
                        None => return Ok(()),
                    }

                    if self
                        .platform
                        .is_key_down(Nibble::try_from(self.register[x.as_usize()]).unwrap())
                    {
                        return Ok(());
                    }
                }
                SetToRandom(x, y) => {
                    self.register[x.as_usize()] = self.platform.get_random_word() & y;
                }

                _ => {
                    return Err(Error::UnsupportedOperation(op));
                }
            }
        } else {
            return Err(Error::UnknownOpCode(code));
        }
        self.instruction_counter += 2;
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    ClearScreen,
    Return,
    Jump(Address),
    Call(Address),
    SkipIfEqual(RegisterIndex, Word),
    SkipIfNotEqual(RegisterIndex, Word),
    SkipIfRegistersEqual(RegisterIndex, RegisterIndex),
    SetRegister(RegisterIndex, Word),
    AddValue(RegisterIndex, Word),
    SetToRegister(RegisterIndex, RegisterIndex),
    Or(RegisterIndex, RegisterIndex),
    And(RegisterIndex, RegisterIndex),
    Xor(RegisterIndex, RegisterIndex),
    AddRegister(RegisterIndex, RegisterIndex),
    SubRegister(RegisterIndex, RegisterIndex),
    ShiftRight(RegisterIndex, RegisterIndex),
    SubRegisterReversed(RegisterIndex, RegisterIndex),
    ShiftLeft(RegisterIndex, RegisterIndex),
    SkipIfRegistersNotEqual(RegisterIndex, RegisterIndex),
    SetIndexRegister(Address),
    JumpV0(Address),
    SetToRandom(RegisterIndex, Word),
    Draw(RegisterIndex, RegisterIndex, Nibble),
    SkipIfKeyDown(RegisterIndex),
    SkipIfKeyUp(RegisterIndex),
    GetDelayTimer(RegisterIndex),
    WaitForKey(RegisterIndex),
    SetDelayTimer(RegisterIndex),
    SetSoundTimer(RegisterIndex),
    IncrementIndexRegister(RegisterIndex),
    SetIndexRegisterToSprite(Nibble),
    ToDecimal(RegisterIndex),
    WriteMemory(Nibble),
    ReadMemory(Nibble),
}

impl TryFrom<OpCode> for Operation {
    type Error = ();

    fn try_from(code: OpCode) -> core::result::Result<Self, ()> {
        let mut nibbs = [0u8; 4];
        for (i, nibb) in nibbs.iter_mut().enumerate() {
            *nibb = code.extract_nibble(3 - i).as_u8();
        }

        let op = match nibbs {
            [0, 0, 0xE, 0] => ClearScreen,
            [0, 0, 0xE, 0xE] => Return,
            [0x1, ..] => Jump(code.extract_address()),
            [0x2, ..] => Call(code.extract_address()),
            [0x3, x, ..] => SkipIfEqual(RegisterIndex::try_from(x)?, code.extract_word(0)),
            [0x4, x, ..] => SkipIfNotEqual(RegisterIndex::try_from(x)?, code.extract_word(0)),
            [0x5, x, y, 0] => {
                SkipIfRegistersEqual(RegisterIndex::try_from(x)?, RegisterIndex::try_from(y)?)
            }
            [0x6, x, ..] => SetRegister(RegisterIndex::try_from(x)?, code.extract_word(0)),
            [0x7, x, ..] => AddValue(RegisterIndex::try_from(x)?, code.extract_word(0)),
            [0x8, x, y, 0] => {
                SetToRegister(RegisterIndex::try_from(x)?, RegisterIndex::try_from(y)?)
            }
            [0x8, x, y, 1] => Or(RegisterIndex::try_from(x)?, RegisterIndex::try_from(y)?),
            [0x8, x, y, 2] => And(RegisterIndex::try_from(x)?, RegisterIndex::try_from(y)?),
            [0x8, x, y, 3] => Xor(RegisterIndex::try_from(x)?, RegisterIndex::try_from(y)?),
            [0x8, x, y, 4] => AddRegister(RegisterIndex::try_from(x)?, RegisterIndex::try_from(y)?),
            [0x8, x, y, 5] => SubRegister(RegisterIndex::try_from(x)?, RegisterIndex::try_from(y)?),
            [0x8, x, y, 6] => ShiftRight(RegisterIndex::try_from(x)?, RegisterIndex::try_from(y)?),
            [0x8, x, y, 7] => {
                SubRegisterReversed(RegisterIndex::try_from(x)?, RegisterIndex::try_from(y)?)
            }
            [0x8, x, y, 0xE] => ShiftLeft(RegisterIndex::try_from(x)?, RegisterIndex::try_from(y)?),
            [0x9, x, y, 0] => {
                SkipIfRegistersNotEqual(RegisterIndex::try_from(x)?, RegisterIndex::try_from(y)?)
            }
            [0xA, ..] => SetIndexRegister(code.extract_address()),
            [0xB, ..] => JumpV0(code.extract_address()), //TODO:: realise
            [0xC, x, ..] => SetToRandom(RegisterIndex::try_from(x)?, code.extract_word(0)), //TODO:: realise
            [0xD, x, y, n] => Draw(
                RegisterIndex::try_from(x)?,
                RegisterIndex::try_from(y)?,
                Nibble::try_from(n)?,
            ),
            [0xE, x, 9, 0xE] => SkipIfKeyDown(RegisterIndex::try_from(x)?),
            [0xE, x, 0xA, 1] => SkipIfKeyUp(RegisterIndex::try_from(x)?),
            [0xF, x, 0, 7] => GetDelayTimer(RegisterIndex::try_from(x)?),
            [0xF, x, 0, 0xA] => WaitForKey(RegisterIndex::try_from(x)?),
            [0xF, x, 1, 5] => SetDelayTimer(RegisterIndex::try_from(x)?),
            [0xF, x, 1, 8] => SetSoundTimer(RegisterIndex::try_from(x)?),
            [0xF, x, 1, 0xE] => IncrementIndexRegister(RegisterIndex::try_from(x)?),
            [0xF, x, 2, 9] => SetIndexRegisterToSprite(Nibble::try_from(x)?),
            [0xF, x, 3, 3] => ToDecimal(Nibble::try_from(x)?),
            [0xF, x, 5, 5] => WriteMemory(Nibble::try_from(x)?),
            [0xF, x, 6, 5] => ReadMemory(Nibble::try_from(x)?),

            _ => return Err(()),
        };
        Ok(op)
    }
}

////////////////////////////////////////////////////////////////////////////////
