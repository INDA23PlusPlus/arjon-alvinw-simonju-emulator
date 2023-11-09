use std::fmt::Display;

use super::registries::Registry;

// Instruction types
const NOOP: u8 = 0;
const HALT: u8 = 1;
const IOUT: u8 = 2;
const JUMP: u8 = 3;

const IADD: u8 = 4;
const ISUB: u8 = 5;
const IMUL: u8 = 6;
const IDIV: u8 = 7;

type AddressImmediate = u16;
type IntegerImmediate = i16;

pub enum Instruction {
    NOOP,
    HALT,
    JUMP(Registry, Registry, Registry, AddressImmediate),
    IOUT(Registry),

    IADD(Registry, Registry, Registry, IntegerImmediate),
    ISUB(Registry, Registry, Registry, IntegerImmediate),
    IMUL(Registry, Registry, Registry, IntegerImmediate),
    IDIV(Registry, Registry, Registry, IntegerImmediate),

    ERROR,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::NOOP => write!(f, "NOOP"),
            Instruction::HALT => write!(f, "HALT"),
            Instruction::JUMP(r1, r2, r3, i) => write!(f, "JUMP: {}, {}, {}, {}", r1, r2, r3, i),
            Instruction::IOUT(r) => write!(f, "IOUT: {}", r),
            Instruction::IADD(r1, r2, r3, i) => write!(f, "IADD: {}, {}, {}, {}", r1, r2, r3, i),
            Instruction::ISUB(r1, r2, r3, i) => write!(f, "ISUB: {}, {}, {}, {}", r1, r2, r3, i),
            Instruction::IMUL(r1, r2, r3, i) => write!(f, "IMUL: {}, {}, {}, {}", r1, r2, r3, i),
            Instruction::IDIV(r1, r2, r3, i) => write!(f, "IDIV: {}, {}, {}, {}", r1, r2, r3, i),
            Instruction::ERROR => write!(f, "Unrecognized instruction"),
        }
    }
}

fn to_registry(val: u8) -> Result<Registry, std::io::Error> {
    if val > 16 {
        panic!(); // TODO error
    }
    return Ok(val as Registry);
}

impl TryFrom<[u8; 4]> for Instruction {
    type Error = std::io::Error; // todo wait for Simons changes

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        const INSTRUCTION: u8 = 0b1111_0000;
        const REGISTRY_1: u8 = 0b0000_1111;
        const REGISTRY_2: u8 = 0b1111_0000;
        const REGISTRY_3: u8 = 0b0000_1111;

        Ok(match value[0] & INSTRUCTION {
            NOOP => Instruction::NOOP,
            HALT => Instruction::HALT,
            JUMP => Instruction::JUMP(
                to_registry(value[0] & REGISTRY_1)?, 
                to_registry(value[1] & REGISTRY_2 >> 4)?,
                to_registry(value[1] & REGISTRY_3)?, 
                AddressImmediate::from_be_bytes([value[2], value[3]])
            ),
            IOUT => Instruction::IOUT(
                to_registry(value[0] & REGISTRY_1)?,
            ),
            IADD => Instruction::IADD(
                to_registry(value[0] & REGISTRY_1)?, 
                to_registry(value[1] & REGISTRY_2 >> 4)?,
                to_registry(value[1] & REGISTRY_3)?, 
                i16::from_be_bytes([value[2], value[3]])
            ),
            ISUB => Instruction::ISUB(
                to_registry(value[0] & REGISTRY_1)?, 
                to_registry(value[1] & REGISTRY_2 >> 4)?,
                to_registry(value[1] & REGISTRY_3)?, 
                i16::from_be_bytes([value[2], value[3]])
            ),
            IMUL => Instruction::IMUL(
                to_registry(value[0] & REGISTRY_1)?, 
                to_registry(value[1] & REGISTRY_2 >> 4)?,
                to_registry(value[1] & REGISTRY_3)?, 
                i16::from_be_bytes([value[2], value[3]])
            ),
            IDIV => Instruction::IDIV(
                to_registry(value[0] & REGISTRY_1)?, 
                to_registry(value[1] & REGISTRY_2 >> 4)?,
                to_registry(value[1] & REGISTRY_3)?, 
                i16::from_be_bytes([value[2], value[3]])
            ),
            _ => Instruction::ERROR,
        })
        
    }
}