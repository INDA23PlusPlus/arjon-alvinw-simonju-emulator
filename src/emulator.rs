mod instructions;
mod registries;

use std::{error::Error, fs::File, os::unix::prelude::FileExt, fmt::Display};
use instructions::Instruction;
use crate::emulator::registries::Value;

use self::registries::{RegistryBank, Registry};

#[derive(Debug)]
pub enum EmulatorError {
    InvalidInstruction(Instruction),
    InvalidRegistry(Registry),
    UnexpectedEndOfFile,
    // DivZeroError
    IOError(std::io::Error),
}

impl Display for EmulatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmulatorError::InvalidInstruction(i) => write!(f, "Invalid instruction: {}", i),
            EmulatorError::InvalidRegistry(r) => write!(f, "Invalid registry: {}", r),
            EmulatorError::UnexpectedEndOfFile => write!(f, "Unexpected end of file"),
            EmulatorError::IOError(err) => write!(f, "Failed to read: {}", err),
        }
    }
}

impl Error for EmulatorError {}

impl From<std::io::Error> for EmulatorError {
    fn from(err: std::io::Error) -> Self {
        Self::IOError(err)
    }
}

pub(super) struct Emulator {
    file: File,
    buffer: [u8; 1024],
    registries: RegistryBank<16>,
    cursor: u64,
}

impl Emulator {
    pub fn new(file: File) -> Self {
        const ZERO: Value = [0, 0];
        const PONE: Value = [0, 1];
        const NONE: Value = i16::to_be_bytes(-1);
        const RAND: Value = [0, 0];

        let registries = RegistryBank::<16>::new( [
            ZERO,   PONE,   NONE,   RAND,
            [0, 0], [0, 0], [0, 0], [0, 0],
            [0, 0], [0, 0], [0, 0], [0, 0], 
            [0, 0], [0, 0], [0, 0], [0, 0]
        ]);

        Self {
            file,
            buffer: [0; 1024],
            registries,
            cursor: 0,
        }
    }

    pub fn run(&mut self) -> Result<(), EmulatorError> {
        'run: loop {
            let instructions = self.file.read_at(&mut self.buffer, self.cursor * 4)? / 4;

            if instructions == 0 {
                return Err(EmulatorError::UnexpectedEndOfFile)
            }

            for i in 0..instructions {
                let instruction = Instruction::from([
                    self.buffer[i * 4],
                    self.buffer[i * 4 + 1],
                    self.buffer[i * 4 + 2],
                    self.buffer[i * 4 + 3],
                ]);

                match instruction {
                    Instruction::NOOP => (),
                    Instruction::HALT => break 'run,
                    Instruction::JUMP(address_registry, comparison_registry_left, comparison_registry_right, address_offset) => { 
                        let address = (self.registries
                            .read_u16(address_registry)
                            .ok_or(EmulatorError::InvalidRegistry(address_registry))? + address_offset) as u64;

                        let comparison_left = self.registries
                            .read_i16(address_registry)
                            .ok_or(EmulatorError::InvalidRegistry(address_registry))?;

                        let comparison_right = self.registries
                            .read_i16(address_registry)
                            .ok_or(EmulatorError::InvalidRegistry(address_registry))?;

                        if comparison_left == comparison_right {
                            self.cursor = address;
                            continue 'run
                        }
                    },
                    Instruction::IOUT(printable_registry) => {
                        let printable = self.registries
                            .read_i16(printable_registry)
                            .ok_or(EmulatorError::InvalidRegistry(printable_registry))?;

                        print!("{}", printable);
                    },
                    Instruction::IADD(result_registry, left_operand_registry, right_operand_registry, right_right_operand_immediate) => {
                        let left_operand = self.registries
                            .read_i16(left_operand_registry)
                            .ok_or(EmulatorError::InvalidRegistry(left_operand_registry))?;

                        let right_operand = self.registries
                            .read_i16(right_operand_registry)
                            .ok_or(EmulatorError::InvalidRegistry(right_operand_registry))?;

                        let result = left_operand + right_operand + right_right_operand_immediate;

                        self.registries.write_i16(result_registry, result)?;
                    },
                    Instruction::ISUB(result_registry, left_operand_registry, right_operand_registry, right_right_operand_immediate) => {
                        let left_operand = self.registries
                            .read_i16(left_operand_registry)
                            .ok_or(EmulatorError::InvalidRegistry(left_operand_registry))?;

                        let right_operand = self.registries
                            .read_i16(right_operand_registry)
                            .ok_or(EmulatorError::InvalidRegistry(right_operand_registry))?;

                        let result = left_operand - right_operand - right_right_operand_immediate;

                        self.registries.write_i16(result_registry, result)?;
                    },
                    Instruction::IMUL(result_registry, left_operand_registry, right_operand_registry, right_right_operand_immediate) => {
                        let left_operand = self.registries
                            .read_i16(left_operand_registry)
                            .ok_or(EmulatorError::InvalidRegistry(left_operand_registry))?;

                        let right_operand = self.registries
                            .read_i16(right_operand_registry)
                            .ok_or(EmulatorError::InvalidRegistry(right_operand_registry))?;

                        let result = left_operand * right_operand * right_right_operand_immediate;

                        self.registries.write_i16(result_registry, result)?;
                    },
                    Instruction::IDIV(result_registry, left_operand_registry, right_operand_registry, right_right_operand_immediate) => {
                        let left_operand = self.registries
                            .read_i16(left_operand_registry)
                            .ok_or(EmulatorError::InvalidRegistry(left_operand_registry))?;

                        let right_operand = self.registries
                            .read_i16(right_operand_registry)
                            .ok_or(EmulatorError::InvalidRegistry(right_operand_registry))?;

                        let result = left_operand / right_operand / right_right_operand_immediate;

                        self.registries.write_i16(result_registry, result)?;
                    },
                    Instruction::ERROR => break 'run, // should return error
                }
            }
        }

        Ok(())
    }
}