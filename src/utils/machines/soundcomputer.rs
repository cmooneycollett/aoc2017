use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use fancy_regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref SND_REGEX: Regex = Regex::new(r"^snd ([a-z]|-?\d+)$").unwrap();
    static ref SET_REGEX: Regex = Regex::new(r"^set ([a-z]) ([a-z]|-?\d+)$").unwrap();
    static ref ADD_REGEX: Regex = Regex::new(r"^add ([a-z]) ([a-z]|-?\d+)$").unwrap();
    static ref MUL_REGEX: Regex = Regex::new(r"^mul ([a-z]) ([a-z]|-?\d+)$").unwrap();
    static ref MOD_REGEX: Regex = Regex::new(r"^mod ([a-z]) ([a-z]|-?\d+)$").unwrap();
    static ref RCV_REGEX: Regex = Regex::new(r"^rcv ([a-z])$").unwrap();
    static ref JGZ_REGEX: Regex = Regex::new(r"^jgz ([a-z]|-?\d+) ([a-z]|-?\d+)$").unwrap();
    static ref SUB_REGEX: Regex = Regex::new(r"^sub ([a-z]) ([a-z]|-?\d+)$").unwrap();
    static ref JNZ_REGEX: Regex = Regex::new(r"^jnz ([a-z]|-?\d+) ([a-z]|-?\d+)$").unwrap();
}

/// Custom error type indicating that the parsing of raw input to a variant of the [`Instruction`]
/// enum has failed.
#[derive(Debug)]
pub struct InstructionParseError;

/// Custom error type indicating that a register read for a [`SoundComputer`] has failed.
#[derive(Debug)]
pub struct RegisterReadError;

/// Custom error type indicating that a register write for a [`SoundComputer`] has failed.
#[derive(Debug)]
pub struct RegisterWriteError;

/// Enum representing the different instructions that can be executed by the [`SoundComputer`].
#[derive(Copy, Clone)]
pub enum Instruction {
    /// Play sound / send (duet mode)
    Snd { arg: InstructionArgument },
    /// Set
    Set { reg: char, arg: InstructionArgument },
    /// Add
    Add { reg: char, arg: InstructionArgument },
    /// Multiply
    Mul { reg: char, arg: InstructionArgument },
    /// Modulus
    Mod { reg: char, arg: InstructionArgument },
    /// Recover frequency / receive (duet mode)
    Rcv { reg: char },
    /// Jump if greater than zero
    Jgz {
        arg1: InstructionArgument,
        arg2: InstructionArgument,
    },
    /// Subtract
    Sub { reg: char, arg: InstructionArgument },
    /// Jump if not zero
    Jnz {
        arg1: InstructionArgument,
        arg2: InstructionArgument,
    },
}

impl Instruction {
    /// Parses a line-separated sequence of instructions into a vector.
    pub fn parse_raw_input(raw_input: &str) -> Vec<Instruction> {
        raw_input
            .trim()
            .lines()
            .map(|line| Instruction::from_str(line).unwrap())
            .collect::<Vec<Instruction>>()
    }
}

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(Some(caps)) = SND_REGEX.captures(s) {
            let arg = InstructionArgument::from_str(&caps[1]).unwrap();
            return Ok(Instruction::Snd { arg });
        } else if let Ok(Some(caps)) = SET_REGEX.captures(s) {
            let reg = caps[1].parse::<char>().unwrap();
            let arg = InstructionArgument::from_str(&caps[2]).unwrap();
            return Ok(Instruction::Set { reg, arg });
        } else if let Ok(Some(caps)) = ADD_REGEX.captures(s) {
            let reg = caps[1].parse::<char>().unwrap();
            let arg = InstructionArgument::from_str(&caps[2]).unwrap();
            return Ok(Instruction::Add { reg, arg });
        } else if let Ok(Some(caps)) = MUL_REGEX.captures(s) {
            let reg = caps[1].parse::<char>().unwrap();
            let arg = InstructionArgument::from_str(&caps[2]).unwrap();
            return Ok(Instruction::Mul { reg, arg });
        } else if let Ok(Some(caps)) = MOD_REGEX.captures(s) {
            let reg = caps[1].parse::<char>().unwrap();
            let arg = InstructionArgument::from_str(&caps[2]).unwrap();
            return Ok(Instruction::Mod { reg, arg });
        } else if let Ok(Some(caps)) = RCV_REGEX.captures(s) {
            let reg = caps[1].parse::<char>().unwrap();
            return Ok(Instruction::Rcv { reg });
        } else if let Ok(Some(caps)) = JGZ_REGEX.captures(s) {
            let arg1 = InstructionArgument::from_str(&caps[1]).unwrap();
            let arg2 = InstructionArgument::from_str(&caps[2]).unwrap();
            return Ok(Instruction::Jgz { arg1, arg2 });
        } else if let Ok(Some(caps)) = SUB_REGEX.captures(s) {
            let reg = caps[1].parse::<char>().unwrap();
            let arg = InstructionArgument::from_str(&caps[2]).unwrap();
            return Ok(Instruction::Sub { reg, arg });
        } else if let Ok(Some(caps)) = JNZ_REGEX.captures(s) {
            let arg1 = InstructionArgument::from_str(&caps[1]).unwrap();
            let arg2 = InstructionArgument::from_str(&caps[2]).unwrap();
            return Ok(Instruction::Jnz { arg1, arg2 });
        }
        // Failed to match the input text to an instruction pattern
        Err(InstructionParseError)
    }
}

/// Enum used to represent the two possible types of arguments present in [`Instruction`] variant
/// fields. Some Instructions have arguments have can either be a constant integer value or a value
/// read from the register of a [`SoundComputer`].
#[derive(Copy, Clone)]
pub enum InstructionArgument {
    Value { val: i64 },
    Register { reg: char },
}

impl FromStr for InstructionArgument {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(val) = s.parse::<i64>() {
            return Ok(InstructionArgument::Value { val });
        } else if let Ok(reg) = s.parse::<char>() {
            return Ok(InstructionArgument::Register { reg });
        }
        Err(InstructionParseError)
    }
}

/// Represents a sound computer that can execute instructions (see [`Instruction`]) in either
/// single-mode or duet-mode.
///
/// The sound computer has 26 registers (labelled 'a' to 'z') that are initialised to 0.
pub struct SoundComputer {
    instructions: Vec<Instruction>,
    registers: HashMap<char, i64>,
    duet_mode: bool,
    pc: usize,
    sounds_sent: VecDeque<i64>,
    sounds_received: VecDeque<i64>,
    awaiting_input: bool,
    halted: bool,
    total_sounds_sent: u64,
    last_sound_sent: Option<i64>,
    mul_executions_count: usize,
}

impl SoundComputer {
    pub fn new(instructions: &[Instruction], duet_mode: bool) -> SoundComputer {
        SoundComputer {
            instructions: instructions.to_vec(),
            registers: ('a'..='z').map(|c| (c, 0)).collect::<HashMap<char, i64>>(),
            duet_mode,
            pc: 0,
            sounds_sent: VecDeque::new(),
            sounds_received: VecDeque::new(),
            awaiting_input: false,
            halted: false,
            total_sounds_sent: 0,
            last_sound_sent: None,
            mul_executions_count: 0,
        }
    }

    /// Executes instructions held by the [`SoundComputer`] until execution is halted or input is
    /// required.
    pub fn execute(&mut self) {
        if self.halted || self.awaiting_input {
            return;
        }
        // Execute instructions while the program counter remains within the instruction space
        loop {
            if self.pc >= self.instructions.len() {
                break;
            }
            // Execute current instruction
            match self.instructions[self.pc] {
                Instruction::Snd { arg } => {
                    let value = self.decode_instruction_argument(arg).unwrap();
                    self.sounds_sent.push_back(value);
                    self.total_sounds_sent += 1;
                    self.last_sound_sent = Some(value);
                }
                Instruction::Set { reg, arg } => {
                    let value = self.decode_instruction_argument(arg).unwrap();
                    self.update_register(&reg, value).unwrap();
                }
                Instruction::Add { reg, arg } => {
                    let value = self.decode_instruction_argument(arg).unwrap();
                    *self.registers.get_mut(&reg).unwrap() += value;
                }
                Instruction::Mul { reg, arg } => {
                    let value = self.decode_instruction_argument(arg).unwrap();
                    *self.registers.get_mut(&reg).unwrap() *= value;
                    self.mul_executions_count += 1;
                }
                Instruction::Mod { reg, arg } => {
                    let value = self.decode_instruction_argument(arg).unwrap();
                    *self.registers.get_mut(&reg).unwrap() %= value;
                }
                Instruction::Rcv { reg } => {
                    let value = self.read_register(&reg).unwrap();
                    if !self.duet_mode && value != 0 {
                        return;
                    } else {
                        if self.sounds_received.is_empty() {
                            self.awaiting_input = true;
                            return;
                        }
                        let sound_received = self.sounds_received.pop_front().unwrap();
                        self.update_register(&reg, sound_received).unwrap();
                    }
                }
                Instruction::Jgz { arg1, arg2 } => {
                    let check_value = self.decode_instruction_argument(arg1).unwrap();
                    let jmp = self.decode_instruction_argument(arg2).unwrap();
                    if check_value > 0 {
                        if self.conduct_pc_jump(jmp) {
                            return;
                        }
                        continue;
                    }
                }
                Instruction::Sub { reg, arg } => {
                    let value = self.decode_instruction_argument(arg).unwrap();
                    *self.registers.get_mut(&reg).unwrap() -= value;
                }
                Instruction::Jnz { arg1, arg2 } => {
                    let check_value = self.decode_instruction_argument(arg1).unwrap();
                    let jmp = self.decode_instruction_argument(arg2).unwrap();
                    if check_value != 0 {
                        if self.conduct_pc_jump(jmp) {
                            return;
                        }
                        continue;
                    }
                }
            }
            // Go to the next instruction
            self.pc += 1;
        }
        // Halt execution now that PC is outside of instruction space
        self.halted = true;
    }

    /// Returns the value held in the specified register.
    ///
    /// If the register does not exist, a [`RegisterReadError`] is returned.
    pub fn read_register(&self, register: &char) -> Result<i64, RegisterReadError> {
        if !self.registers.contains_key(register) {
            return Err(RegisterReadError);
        }
        Ok(*self.registers.get(register).unwrap())
    }

    /// Updates the value held in the specified register.
    ///
    /// If the register does not exist, a [`RegisterWriteError`] is returned.
    pub fn update_register(
        &mut self,
        register: &char,
        value: i64,
    ) -> Result<(), RegisterWriteError> {
        if !self.registers.contains_key(register) {
            return Err(RegisterWriteError);
        }
        self.registers.insert(*register, value);
        Ok(())
    }

    /// Takes the sounds that have been added to the sent buffer.
    pub fn take_sent_sounds(&mut self) -> Vec<i64> {
        let taken_sounds = self.sounds_sent.iter().copied().collect::<Vec<i64>>();
        self.sounds_sent = VecDeque::new();
        taken_sounds
    }

    /// Adds the sounds to the receive buffer.
    pub fn receive_sounds(&mut self, sounds: &[i64]) {
        for sound in sounds {
            self.sounds_received.push_back(*sound);
        }
        if !self.sounds_received.is_empty() {
            self.awaiting_input = false;
        }
    }

    /// Gets the last sound sent by the [`SoundComputer`].
    pub fn get_last_sent_sound(&self) -> Option<i64> {
        self.last_sound_sent
    }

    /// Checks if the [`SoundComputer`] is awaiting input.
    pub fn is_awaiting_input(&self) -> bool {
        self.awaiting_input
    }

    /// Checks if the [`SoundComputer`] has halted execution.
    pub fn is_halted(&self) -> bool {
        self.halted
    }

    /// Gets the total number of sounds sent by the [`SoundComputer`].
    pub fn get_total_sounds_sent(&self) -> u64 {
        self.total_sounds_sent
    }

    /// Gets the number of times the [`SoundComputer`] has executed the MUL (multiply) instruction.
    pub fn get_mul_executions_count(&self) -> usize {
        self.mul_executions_count
    }

    /// Extracts the the value from the last argument in the instruction at the given index.
    ///
    /// Returns None if the [`SoundComputer`] has an empty instruction set, the index is outside of
    /// the instruction space or the instruction at the location does not have any argument fields.
    pub fn extract_last_arg_value(&self, i: usize) -> Option<i64> {
        if self.instructions.is_empty() || i >= self.instructions.len() {
            return None;
        }
        let arg = match self.instructions[i] {
            Instruction::Snd { arg } => arg,
            Instruction::Set { reg: _, arg } => arg,
            Instruction::Add { reg: _, arg } => arg,
            Instruction::Mul { reg: _, arg } => arg,
            Instruction::Mod { reg: _, arg } => arg,
            Instruction::Rcv { reg: _ } => return None,
            Instruction::Jgz { arg1: _, arg2 } => arg2,
            Instruction::Sub { reg: _, arg } => arg,
            Instruction::Jnz { arg1: _, arg2 } => arg2,
        };
        self.decode_instruction_argument(arg).ok()
    }

    /// Decodes an [`InstructionArgument`] variant by returning its integer value or the value held
    /// in the designated register.
    ///
    /// If the register does not exist, a [`RegisterReadError`] is returned.
    fn decode_instruction_argument(
        &self,
        arg: InstructionArgument,
    ) -> Result<i64, RegisterReadError> {
        match arg {
            InstructionArgument::Value { val: value } => Ok(value),
            InstructionArgument::Register { reg: register } => self.read_register(&register),
        }
    }

    /// Helper function to implement a jump to the [`SoundComputer`] program counter.
    ///
    /// Returns true if the jump moves the PC outside of the instruction space and thereby halts the
    /// machine, otherwise returns false.
    fn conduct_pc_jump(&mut self, jmp: i64) -> bool {
        match jmp.is_negative() {
            true => {
                // Check if the jump would move the pc left of instruction space
                let jump_value = usize::try_from(jmp.unsigned_abs()).unwrap();
                if jump_value > self.pc {
                    self.halted = true;
                    return true;
                }
                self.pc -= jump_value;
            }
            false => {
                self.pc += usize::try_from(jmp.unsigned_abs()).unwrap();
            }
        }
        false
    }
}
