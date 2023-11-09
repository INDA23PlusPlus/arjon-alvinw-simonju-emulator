mod instructions;
mod registries;

use std::{error::Error, fs::File, os::unix::prelude::FileExt, fmt::Display};
use instructions::{Instruction, };
use crate::emulator::registries::Value;

use self::registries::{RegistryBank, Registry};

enum EmulatorError {
    InvalidInstruction(Instruction),
    InvalidRegistry(Registry),
    UnexpectedEndOfFile,
}

impl Display for EmulatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmulatorError::InvalidInstruction(i) => write!(f, "Invalid instruction: {}", i),
            EmulatorError::InvalidRegistry(r) => write!(f, "Invalid registry: {}", r),
            EmulatorError::UnexpectedEndOfFile => write!(f, "Unexpected end of file"),
        }
    }
}

impl Error for EmulatorError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }

    fn provide<'a>(&'a self, demand: &mut std::any::Demand<'a>) {}
}

struct Emulator {
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

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        'run: loop {
            let instructions = self.file.read_at(&mut self.buffer, self.cursor * 4)? / 4;

            if instructions == 0 {
                return Err(EmulatorError::UnexpectedEndOfFile)
            }

            'inner: for i in 0..instructions {
                let instruction = Instruction::from([
                    self.buffer[i * 4],
                    self.buffer[i * 4 + 1],
                    self.buffer[i * 4 + 2],
                    self.buffer[i * 4 + 3],
                ]);

                match instruction {
                    Instruction::NOOP => continue 'inner,
                    Instruction::HALT => break 'run,
                    Instruction::JUMP(address, offset) => { 
                        let address = if let Some(x) = self.registries.read(address) {
                            (x + offset) as u64
                        } else {
                            break 'run // should return error
                        };

                        self.cursor = address;
                        continue 'run
                    },
                    Instruction::FORK(address, condition_1, condition_2, offset) => {
                        let condition_1 = if let Some(x) = self.registries.read(condition_1) {
                            x
                        } else {
                            break 'run // should return error
                        };

                        let condition_2 = if let Some(x) = self.registries.read(condition_2) {
                            x
                        } else {
                            break 'run // should return error
                        };

                        let address = if let Some(x) = self.registries.read(address) {
                            (x + offset) as u64
                        } else {
                            break 'run // should return error
                        };

                        if condition_1 == condition_2 {
                            self.cursor = address;
                            continue 'run
                        }

                        continue 'inner
                    },
                    Instruction::LOAD(registry, address, offset) => {

                    },
                    Instruction::POOL(_, _, _) => todo!(),
                    Instruction::COUT(_) => todo!(),
                    Instruction::IOUT(_) => todo!(),
                    Instruction::IADD(res_reg, a_reg, b_reg, immediate) => {
                        let a = self.registries.read_i16(a_reg);
                        let b = self.registries.read_i16(a_reg);
                        let res = a + b + immediate;
                        self.registries.write_i16(res_reg, res);
                    },
                    Instruction::ISUB(res_reg, a_reg, b_reg, immediate) => {
                        let a = self.registries.read_i16(a_reg);
                        let b = self.registries.read_i16(a_reg);
                        let res = a - b - immediate;
                        self.registries.write_i16(res_reg, res);
                    },
                    Instruction::IMUL(res_reg, a_reg, b_reg, immediate) => {
                        let a = self.registries.read_i16(a_reg);
                        let b = self.registries.read_i16(a_reg);
                        let res = a * b * immediate;
                        self.registries.write_i16(res_reg, res);
                    },
                    Instruction::IDIV(res_reg, a_reg, b_reg, immediate) => {
                        let a = self.registries.read_i16(a_reg);
                        let b = self.registries.read_i16(a_reg);
                        let res = a / b / immediate;
                        self.registries.write_i16(res_reg, res);
                    },
                    Instruction::ERROR => break 'run, // should return error
                }
            }
        }

        Ok(())
    }
}