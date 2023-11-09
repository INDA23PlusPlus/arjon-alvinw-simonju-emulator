use std::fmt::Display;

use super::registries::{Registry, Value};

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

#[derive(Debug)]
pub enum Instruction {
    NOOP,
    HALT,
    JUMP(Registry, Registry, Registry, AddressImmediate),
    IOUT(Registry),

    IADD(Registry, Registry, Registry, IntegerImmediate),
    ISUB(Registry, Registry, Registry, IntegerImmediate),
    IMUL(Registry, Registry, Registry, IntegerImmediate),
    IDIV(Registry, Registry, Registry, IntegerImmediate),

    ERROR(u8),
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
            Instruction::ERROR(e) => write!(f, "Unrecognized instruction: {}", e),
        }
    }
}

impl From<[u8; 4]> for Instruction {
    fn from(value: [u8; 4]) -> Self {
        const INSTRUCTION: u8 = 0b1111_0000;
        const REGISTRY_1: u8 = 0b0000_1111;
        const REGISTRY_2: u8 = 0b1111_0000;
        const REGISTRY_3: u8 = 0b0000_1111;

        let val = (value[0] & INSTRUCTION) >> 4;
        match val {
            NOOP => Instruction::NOOP,
            HALT => Instruction::HALT,
            JUMP => Instruction::JUMP(
                (value[0] & REGISTRY_1) as Registry, 
                ((value[1] & REGISTRY_2) >> 4) as Registry,
                (value[1] & REGISTRY_3) as Registry, 
                AddressImmediate::from_be_bytes([value[2], value[3]])
            ),
            IOUT => Instruction::IOUT(
                (value[0] & REGISTRY_1) as Registry,
            ),
            IADD => Instruction::IADD(
                (value[0] & REGISTRY_1) as Registry, 
                ((value[1] & REGISTRY_2) >> 4) as Registry,
                (value[1] & REGISTRY_3) as Registry, 
                i16::from_be_bytes([value[2], value[3]])
            ),
            ISUB => Instruction::ISUB(
                (value[0] & REGISTRY_1) as Registry, 
                ((value[1] & REGISTRY_2) >> 4) as Registry,
                (value[1] & REGISTRY_3) as Registry, 
                i16::from_be_bytes([value[2], value[3]])
            ),
            IMUL => Instruction::IMUL(
                (value[0] & REGISTRY_1) as Registry, 
                ((value[1] & REGISTRY_2) >> 4) as Registry,
                (value[1] & REGISTRY_3) as Registry, 
                i16::from_be_bytes([value[2], value[3]])
            ),
            IDIV => Instruction::IDIV(
                (value[0] & REGISTRY_1) as Registry, 
                ((value[1] & REGISTRY_2) >> 4) as Registry,
                (value[1] & REGISTRY_3) as Registry, 
                i16::from_be_bytes([value[2], value[3]])
            ),
            e => Instruction::ERROR(e),
        }
        
    }
}