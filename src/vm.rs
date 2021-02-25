use crate::pin_values::{InputPin, OutputPin};
use crate::registers::Registers;
use crate::instruction::{InstructionCollection, InstructionData};
use crate::opcode::OpCode;

struct VirtualMachine {
    input_pins : Vec<InputPin>,
    output_pins : Vec<OutputPin>,
    pub registers : Registers,
    rungs : Vec<InstructionCollection>,
}

impl VirtualMachine {
    pub fn run(&mut self){
        // READ INPUTS AND ASSIGN THEIR VALUES TO REGISTERS
        for collection in self.rungs.iter() {
            let mut index = 0;
            while index < collection.get_len(){
                let inst = collection.get_instruction(index); 
                if inst.get_opcode().is_boolean() {
                    // Evaluate boolean 
                    index = VirtualMachine::EvaluateBooleanStatement(&inst, inst.get_opcode().boolean(&inst, &self.registers), index);
                }
                else{
                    // Evaluate operation
                    self.registers = inst.get_opcode().execute(&inst, &self.registers);
                }
            }
        }
        // READ OUTPUT REGISTERS AND WRITE THEIR VALUES TO PINS
        println!("Ran a successful loop");
    }

    fn EvaluateBooleanStatement(inst : &InstructionData, truth_value : bool, curr_index : usize) -> usize {
        if truth_value {
            inst.get_jump_to_then()
        }
        else {
            inst.get_jump_to_else()
        }
    }

    
}

