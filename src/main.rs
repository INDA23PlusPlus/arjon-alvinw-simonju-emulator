mod emulator;
use std::error::Error;
use std::fs;
use emulator::EmulatorError;

use crate::emulator::Emulator;

fn main() -> Result<(), EmulatorError> {
    let file = fs::File::open("test.bin")?;
    let mut emulator: Emulator = Emulator::new(file);
    emulator.run()
}
