use crate::pin_values::OutputPin;
use crate::pin_values::InputPin;
use crate::vm::VirtualMachine;
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
    let register_vals : Registers = Registers::new(&args[1])?;
    let instructions : InstructionCollection = InstructionCollection::new(&args[2])?;
    let rung_info = vec![instructions];
    let in_pins : Vec<InputPin> = Vec::new();
    let out_pins : Vec<OutputPin> = Vec::new();

    let mut vm = VirtualMachine::new(in_pins, out_pins, rung_info, register_vals);
    vm.run();
    VirtualMachine::print_registers(&vm);
    Ok(())    
}