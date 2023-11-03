use std::fs;
use std::io::{self, ErrorKind};

#[repr(u8)]
pub enum Instruction {
    Halt = 0,
}
fn main() {
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
