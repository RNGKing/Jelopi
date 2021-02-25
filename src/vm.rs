use crate::pin_values::{InputPin, OutputPin};
use crate::registers::Registers;
use crate::instruction::{InstructionCollection, InstructionData};
use crate::opcode::OpCode;
use std::{thread, time};

pub struct VirtualMachine {
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
                VirtualMachine::print_registers(self);
                let inst = collection.get_instruction(index); 
                println!("RUNNING INSTRUCTION TYPE: {}, ARG_A: {}, ARG_B: {}, OUT_REG: {}", 
                    inst.get_opcode(), inst.get_val_a(), inst.get_val_b(), inst.get_out_id());
                if inst.get_opcode().is_boolean() {
                    // Evaluate boolean 
                    index = VirtualMachine::EvaluateBooleanStatement(&inst, inst.get_opcode().boolean(&inst, &self.registers), index);
                    println!("THEN Index : {}", index);
                }
                else{
                    // Evaluate operation
                    self.registers = inst.get_opcode().execute(&inst, &self.registers);
                    index = index + 1;
                }
                thread::sleep(time::Duration::from_millis(500));
            }
        }
        // READ OUTPUT REGISTERS AND WRITE THEIR VALUES TO PINS
        println!("Ran a successful loop");
    }

    fn EvaluateBooleanStatement(inst : &InstructionData, truth_value : bool, curr_index : usize) -> usize {
        println!("THEN : {} // ELSE : {}", inst.get_jump_to_then(), inst.get_jump_to_else());
        
        if truth_value {
            inst.get_jump_to_then()
        }
        else {
            inst.get_jump_to_else()
        }
    }

    pub fn new(in_pins : Vec<InputPin>, out_pins : Vec<OutputPin>, rung_info : Vec<InstructionCollection>, register_info : Registers) -> VirtualMachine {
        VirtualMachine {
            input_pins : in_pins,
            output_pins : out_pins,
            rungs : rung_info,
            registers : register_info,
        }
    }

    pub fn print_registers(vm : &VirtualMachine){
        println!("-------------------------------------------");
        for reg in vm.registers.register_entries.clone() {
            println!("-- Register Value : {}", reg.get_value(0));
        }
        println!("-------------------------------------------");
    }

}

