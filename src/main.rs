mod emulator;
use std::fs;
use emulator::EmulatorError;
use gen_binary::gen_fibonacci_binary;

use crate::emulator::Emulator;

mod gen_binary;

fn main() -> Result<(), EmulatorError> {
    gen_fibonacci_binary();

    let file = fs::File::open("fibonacci.bin")?;
    let mut emulator: Emulator = Emulator::new(file);
    emulator.run()
}
