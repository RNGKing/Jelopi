use std::fmt;
use crate::instruction::{InstructionData};
use crate::registers::Registers;

#[derive(Clone, Copy)]
pub enum OpCode {
    ADDR,
    ADDI,
    SBTR,
    MULT,
    DIVD,
    MOD,
    BOR,
    BIN,
    BAND,
    JMPGE,
    JMPLE,
    JMPE,
    JMPG,
    JMPL,
    JMPB,
}

impl OpCode{
    pub fn execute(&self, instruction : &InstructionData, before : &Registers ) -> Registers {
        match self {
            OpCode::ADDR => addr(instruction, before),
            OpCode::ADDI => addi(instruction, before),
            OpCode::SBTR => sbtr(instruction, before),
            OpCode::MULT => multr(instruction, before),
            OpCode::MOD => modur(instruction, before),
            OpCode::DIVD => divd(instruction, before),
            OpCode::BOR => bor(instruction, before),
            OpCode::BIN => bin(instruction, before),
            OpCode::BAND => band(instruction, before),
            _ => unreachable!()
        }
    }

    pub fn boolean(&self, instruction : &InstructionData, before: &Registers) -> bool {
        match self {
            OpCode::JMPB => jmpb(instruction, before),
            OpCode::JMPE => jmpe(instruction, before),
            OpCode::JMPL => jmpl(instruction, before),
            OpCode::JMPG => jmpg(instruction, before),
            OpCode::JMPLE => jmple(instruction, before),
            OpCode::JMPGE => jmpge(instruction, before),
            _ => unreachable!()
        }
    } 

    pub fn is_boolean(&self) -> bool {
        match self {
            OpCode::JMPB => true,
            OpCode::JMPE => true,
            OpCode::JMPL => true,
            OpCode::JMPG => true,
            OpCode::JMPLE => true,
            OpCode::JMPGE => true,
            _ => false,
        }
    }
}

fn jmpb(inst : &InstructionData, before : &Registers) -> bool {
    println!("Value - {}", before.register_entries[inst.get_val_a()].get_value(0));
    before.register_entries[inst.get_val_a()].get_value(0) > 0
}

fn jmpg(inst : &InstructionData, before : &Registers) -> bool {
    before.register_entries[inst.get_val_a()].get_value(0) > 
        before.register_entries[inst.get_val_b()].get_value(0)
}

fn jmpl(inst : &InstructionData, before : &Registers) -> bool {
    before.register_entries[inst.get_val_a()].get_value(0) < 
        before.register_entries[inst.get_val_b()].get_value(0)
}

fn jmpe(inst : &InstructionData, before : &Registers) -> bool {
    before.register_entries[inst.get_val_a()].get_value(0) == 
        before.register_entries[inst.get_val_b()].get_value(0)
}

fn jmple(inst : &InstructionData, before : &Registers) -> bool {
    before.register_entries[inst.get_val_a()].get_value(0) <= 
        before.register_entries[inst.get_val_b()].get_value(0)
}

fn jmpge(inst : &InstructionData, before : &Registers) -> bool {
    before.register_entries[inst.get_val_a()].get_value(0) >= 
        before.register_entries[inst.get_val_b()].get_value(0)
}

impl From<&str> for OpCode {
    fn from(item : &str) -> OpCode {
        match item {
            "addr" => OpCode::ADDR,
            "addi" => OpCode::ADDI,
            "sbtr" => OpCode::SBTR,
            "mult" => OpCode::MULT,
            "divd" => OpCode::DIVD,
            "mod" => OpCode::MOD,
            "bor" => OpCode::BOR,
            "bin" => OpCode::BIN,
            "band" => OpCode::BAND,
            "jmpe" => OpCode::JMPE,
            "jmpb" => OpCode::JMPB,
            "jmpl" => OpCode::JMPL,
            "jmpg" => OpCode::JMPG,
            "jmple" => OpCode::JMPLE,
            "jmpge" => OpCode::JMPGE,
            _ => unreachable!()
        }
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let s = match *self {
            OpCode::ADDR => "addr",
            OpCode::ADDI => "addi",
            OpCode::SBTR => "sbtr",
            OpCode::MULT => "mult",
            OpCode::MOD => "mod",
            OpCode::DIVD => "divd",
            OpCode::BOR => "bor",
            OpCode::BIN => "bin",
            OpCode::BAND => "band",
            OpCode::JMPE => "jmpe",
            OpCode::JMPB => "jmpb",
            OpCode::JMPG => "jmpg",
            OpCode::JMPL => "jmpl",
            OpCode::JMPLE => "jmple",
            OpCode::JMPGE => "jmpge",
        };
        write!(f, "{}", s)
    }
}



fn band(instruction : &InstructionData, before:  &Registers) -> Registers {
    let mut out = before.clone();
    let new_val = out.register_entries[instruction.get_val_a()].get_value(0) &
            out.register_entries[instruction.get_val_b()].get_value(0);
    out.register_entries[instruction.get_out_id()].set_value(0, new_val); 
    out
}

fn bin(instruction : &InstructionData, before:  &Registers) -> Registers {
    let out = before.clone();
    // out.register_entries[instruction.output_register_id].values[0] = 
    //     out.register_entries[instruction.input_val_one].values[0] /
    //     out.register_entries[instruction.input_val_two].values[0];
    out
}

fn bor(instruction : &InstructionData, before:  &Registers) -> Registers {
    let mut out = before.clone();
    let new_val = out.register_entries[instruction.get_val_a()].get_value(0) |
        out.register_entries[instruction.get_val_b()].get_value(0);
    out.register_entries[instruction.get_out_id()].set_value(0, new_val); 
    out
}

fn divd(instruction : &InstructionData, before:  &Registers) -> Registers {
    let mut out = before.clone();
    let mut new_val = out.register_entries[instruction.get_val_a()].get_value(0) /
        out.register_entries[instruction.get_val_b()].get_value(0);
    out.register_entries[instruction.get_out_id()].set_value(0, new_val); 
    out
}

fn modur(instruction : &InstructionData, before:  &Registers) -> Registers {
    let mut out = before.clone();
    let new_val = out.register_entries[instruction.get_val_a()].get_value(0) %
        out.register_entries[instruction.get_val_b()].get_value(0);
    out.register_entries[instruction.get_out_id()].set_value(0, new_val);
    out
}

fn multr(instruction : &InstructionData, before:  &Registers) -> Registers {
    let mut out = before.clone();
    let new_val = out.register_entries[instruction.get_val_a()].get_value(0) *
        out.register_entries[instruction.get_val_b()].get_value(0);
    out.register_entries[instruction.get_out_id()].set_value(0, new_val);
    out
}

fn sbtr(instruction : &InstructionData, before:  &Registers) -> Registers {
    let mut out = before.clone();
    let new_val = out.register_entries[instruction.get_val_a()].get_value(0) -
        out.register_entries[instruction.get_val_b()].get_value(0);
    out.register_entries[instruction.get_out_id()].set_value(0, new_val);
    out
}

fn addi(instruction : &InstructionData, before:  &Registers) -> Registers {
    let mut out = before.clone();
    let new_val = out.register_entries[instruction.get_val_a()].get_value(0) +
        instruction.get_val_b();
    out.register_entries[instruction.get_out_id()].set_value(0, new_val);
    out
}

fn addr(instruction : &InstructionData, before:  &Registers) -> Registers {
    let mut out = before.clone();
    let new_val = out.register_entries[instruction.get_val_a()].get_value(0) +
        out.register_entries[instruction.get_val_b()].get_value(0);
    out.register_entries[instruction.get_out_id()].set_value(0, new_val);
    out
}