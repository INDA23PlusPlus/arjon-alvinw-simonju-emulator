type InstructionType = u8;

const NOOP: InstructionType = 0;
const HALT: InstructionType = 1;
const COUT: InstructionType = 2;
const IOUT: InstructionType = 3;

const JUMP: InstructionType = 4;
const FORK: InstructionType = 5;
const LOAD: InstructionType = 6;
const POOL: InstructionType = 7;

const IADD: InstructionType = 8;
const ISUB: InstructionType = 9;
const IMUL: InstructionType = 10;
const IDIV: InstructionType = 11;

pub type Registry = usize;
pub type Immediate = u16;

pub enum Instruction {
    NOOP,
    HALT,

    JUMP(Registry, Immediate), // u16
    FORK(Registry, Registry, Registry, Immediate), // u16
    LOAD(Registry, Registry, Immediate), // u16
    POOL(Registry, Registry, Immediate), // u16

    COUT(Registry),
    IOUT(Registry),

    IADD(Registry, Registry, Registry, Immediate), // i16
    ISUB(Registry, Registry, Registry, Immediate), // i16
    IMUL(Registry, Registry, Registry, Immediate), // i16
    IDIV(Registry, Registry, Registry, Immediate), // i16

    ERROR,
}

impl From<[u8; 4]> for Instruction {
    fn from(value: [u8; 4]) -> Self {
        const INSTRUCTION: u8 = 0b1111_0000;
        const REGISTRY_1: u8 = 0b0000_1111;
        const REGISTRY_2: u8 = 0b1111_0000;
        const REGISTRY_3: u8 = 0b0000_1111;

        match value[0] & INSTRUCTION {
            NOOP => Instruction::NOOP,
            HALT => Instruction::HALT,
            JUMP => Instruction::JUMP(
                (value[0] & REGISTRY_1) as usize, 
                u16::from_be_bytes([value[2], value[3]])
            ),
            FORK => Instruction::FORK(
                (value[0] & REGISTRY_1) as usize, 
                (value[1] & REGISTRY_2 >> 4) as usize,
                (value[1] & REGISTRY_3) as usize, 
                u16::from_be_bytes([value[2], value[3]])
            ),
            LOAD => Instruction::LOAD(
                (value[0] & REGISTRY_1) as usize, 
                (value[1] & REGISTRY_2 >> 4) as usize,
                u16::from_be_bytes([value[2], value[3]])
            ),
            POOL => Instruction::POOL(
                (value[0] & REGISTRY_1) as usize, 
                (value[1] & REGISTRY_2 >> 4) as usize,
                u16::from_be_bytes([value[2], value[3]])
            ),
            COUT => Instruction::COUT(
                (value[0] & REGISTRY_1) as usize,
            ),
            IOUT => Instruction::IOUT(
                (value[0] & REGISTRY_1) as usize,
            ),
            IADD => Instruction::IADD(
                (value[0] & REGISTRY_1) as usize, 
                (value[1] & REGISTRY_2 >> 4) as usize,
                (value[1] & REGISTRY_3) as usize, 
                i16::from_be_bytes([value[2], value[3]])
            ),
            ISUB => Instruction::ISUB(
                (value[0] & REGISTRY_1) as usize, 
                (value[1] & REGISTRY_2 >> 4) as usize,
                (value[1] & REGISTRY_3) as usize, 
                u16::from_be_bytes([value[2], value[3]])
            ),
            IMUL => Instruction::IMUL(
                (value[0] & REGISTRY_1) as usize, 
                (value[1] & REGISTRY_2 >> 4) as usize,
                (value[1] & REGISTRY_3) as usize, 
                i16::from_be_bytes([value[2], value[3]])
            ),
            IDIV => Instruction::IDIV(
                (value[0] & REGISTRY_1) as usize, 
                (value[1] & REGISTRY_2 >> 4) as usize,
                (value[1] & REGISTRY_3) as usize, 
                i16::from_be_bytes([value[2], value[3]])
            ),
            _ => Instruction::ERROR,
        }
        
    }
}