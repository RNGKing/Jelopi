use crate::opcode::OpCode;
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;
use std::fmt;
use json::{self, JsonValue};
use std::error::Error;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, 
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
#[derive(Clone, Copy)]
pub struct InstructionData {
    op_code: OpCode,
    val_a : usize,
    val_b : usize,
    out_id : usize,
    jump_to_then : usize,
    jump_to_else : usize,
}

impl InstructionData {
    pub fn new(string_value : &str) -> Result<InstructionData, Box<dyn Error>> {
        let mut a : usize = 0;
        let mut b : usize = 0;
        let mut out : usize = 0;
        let mut jump_then : usize = 0;
        let mut jump_else : usize = 0;
        let split : Vec<&str> = string_value.split(" ").collect();
        let code = OpCode::from(split[0]);
        a = split[1].parse::<usize>()?;       
        b = split[2].parse::<usize>()?;       
        out = split[3].parse::<usize>()?;       
        jump_then = split[4].parse::<usize>()?;       
        jump_else = split[5].parse::<usize>()?;       
        Ok(InstructionData {
            op_code : code,
            val_a : a,
            val_b : b,
            out_id : out,
            jump_to_then : jump_then,
            jump_to_else : jump_else,
        })
    }

    pub fn get_opcode(&self) -> OpCode{
        self.op_code
    }

    pub fn get_val_a(&self) -> usize {
        self.val_a
    }

    pub fn get_val_b(&self) -> usize{
        self.val_b
    }

    pub fn get_out_id(&self) -> usize{
        self.out_id
    }

    pub fn get_jump_to_then(&self) -> usize {
        self.jump_to_then
    }

    pub fn get_jump_to_else(&self) -> usize {
        self.jump_to_else
    }
}

#[derive(Clone)]
pub struct InstructionCollection {
    instructions : Vec<InstructionData>
}


impl InstructionCollection {
    pub fn new(filepath : &str ) -> Result<InstructionCollection, String>{
        let mut instruction_list: Vec<InstructionData> = Vec::new();
        match read_lines(filepath){
            Ok(lines) => {
                for line in lines {
                    if let Ok(info) = line {
                        match InstructionData::new(info.as_str()) {
                            Ok(data) => instruction_list.push(data),
                            Err(err) => return Err("Failed to comprehend data".to_string()),
                        }
                    }
                }
            },
            Err(err) => return Err("Failed to read lines from file".to_string()),
        }
        Ok(InstructionCollection {
            instructions : instruction_list,
        })
    }

    pub fn get_instruction(&self, index : usize) -> InstructionData {
        self.instructions[index]
    }

    pub fn get_len(&self) -> usize {
        self.instructions.len()
    }

}
