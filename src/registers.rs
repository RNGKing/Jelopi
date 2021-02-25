use std::fs;
use json::{self, JsonValue};

#[derive(Clone)]
pub enum RegisterType{
    BOOL,
    CHAR,
    CHAR_ARRAY,
    U8,
    U16,
    U32,
    I16,
    I32,
    FLOAT
}

impl From<&str> for RegisterType {
    fn from(item : &str) -> RegisterType {
        match item {
            "bool" => RegisterType::BOOL,
            "char" => RegisterType::CHAR,
            "char_array" => RegisterType::CHAR_ARRAY,
            "u8" => RegisterType::U8,
            "u16" => RegisterType::U16,
            "u32" => RegisterType::U32,
            "i16" => RegisterType::I32,
            "float" => RegisterType::FLOAT,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
pub struct Registers{
    pub register_entries : Vec<RegisterEntry>
}

impl Registers {
    pub fn new(path_to_data : String) -> Result<Registers, String> {
        match fs::read_to_string(path_to_data) {
            Ok(file_content) => {
                match json::parse(&file_content) {
                    Ok(json_data) => {
                        let mut reg_datas = Vec::new();
                        let mut entry_id : u32 = 0;
                        for data in json_data["registers"].members(){
                            match RegisterEntry::new(data, entry_id) {
                                Ok(reg_data) => reg_datas.push(reg_data),
                                Err(err) => return Err(err),
                            }
                            entry_id += 1;
                        }
                        Ok(Registers{
                            register_entries : reg_datas
                        })
                    },
                    Err(err) => Err(err.to_string()) 
                }
            },
            Err(err) => Err(format!("{}", err))
        }
    }
}

#[derive(Clone)]
pub struct RegisterEntry{
    name : String,
    register_type : RegisterType,
    values : Vec<usize>,
    id : u32
}

impl RegisterEntry {
    fn build(register_name : String, data_type : RegisterType, length : usize, reg_id : u32) -> RegisterEntry {
        RegisterEntry{
            name : register_name,
            register_type : data_type,
            values : vec![0;length],
            id : reg_id
        }
    }

    pub fn new(data: &JsonValue, entry_number: u32) -> Result<RegisterEntry, String> {
        let reg_name: String;
        let reg_type : String;
        match data["name"].as_str() {
            Some(name) => reg_name = name.to_string(),
            _ => return Err("Failed to get register_name".to_string())
        }
        match data["type"].as_str() {
            Some(registry_type) => reg_type = registry_type.to_string(),
            _ => return Err("Failed to get registry_type".to_string()), 
        }
        Ok(RegisterEntry::build(reg_name, RegisterType::from(reg_type.as_str()), 128, entry_number))
    }

    pub fn get_value(&self, index : usize) -> usize {
        self.values[index]
    }

    pub fn set_value(&mut self, index : usize, new_val : usize) {
        self.values[index] = new_val
    }
}