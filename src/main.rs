<<<<<<< HEAD
mod emulator;

use std::error::Error;
use std::fs;

=======
use std::error::Error;
use std::fs;
use std::io::{self, ErrorKind};
mod instruction_parser;
mod instructions;

>>>>>>> 2762b00 (add instructions and an error)
fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "";
    let prog = fs::read(file_path)?;
    let mut counter: usize = 0;
    let mut reg = [0u8; 8];
    loop {
        let instruction= prog[counter];
        match instruction {
            _ => {}
        }
    }
}