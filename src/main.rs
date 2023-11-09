mod emulator;
use std::error::Error;
use std::fs;
use emulator::EmulatorError;
use gen_binary::gen_test_binary;

use crate::emulator::Emulator;

mod gen_binary;

fn main() -> Result<(), EmulatorError> {
    gen_test_binary();

    let file = fs::File::open("test.bin")?;
    let mut emulator: Emulator = Emulator::new(file);
    emulator.run()
}
