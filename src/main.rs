mod emulator;
use std::error::Error;
use std::fs;
use crate::emulator::Emulator;

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::File::open("test.bin")?;
    let mut emulator: Emulator = Emulator::new(file);
    emulator.run()
}
