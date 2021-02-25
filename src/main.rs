use std::env;

mod instruction;
mod opcode;
mod registers;
mod vm;
mod pin_values;

use instruction::InstructionCollection;
use registers::Registers;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    let register_vals : Registers = Registers::new(args[1]);

    Ok(())    
}