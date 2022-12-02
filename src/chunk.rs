#![allow(dead_code)]

use std::fmt::Display;

use crate::value::{ValueArray, Value};
use crate::opcode::*;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub code:       Vec<u8>,
    pub constants:  ValueArray,
}

impl Chunk {
    pub fn new() -> Self {
        Self { 
            code: vec![], 
            constants: ValueArray::new(), 
        }
    }

    pub fn write_chunk(&mut self, byte: u8 ) {
        self.code.push(byte);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.write_valuearray(value);
        self.constants.values.len() - 1
    }

    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);
        let instruction = self.code[offset];
        match instruction {
            OP_CONSTANT => {
                let constant = self.code[offset + 1];
                println!("OP_CONSTANT {:04} '{:?}'", constant, self.constants.values[constant as usize]);
                offset + 2
            }
            OP_NEGATE => {
                println!("OP_NEGATE");
                offset + 1
            }
            OP_ADD => {
                println!("OP_ADD");
                offset + 1
            }
            OP_SUBTRACT => {
                println!("OP_SUBSTRACT");
                offset + 1
            }
            OP_DIVIDE => {
                println!("OP_DIVIDE");
                offset + 1
            }
            OP_RETURN => {
                println!("OP_RETURN");
                offset + 1
            }
            OP_MULTIPLY => {
                println!("OP_MULTIPLY");
                offset + 1
            }
            
            _ => {
                println!("Unknown opcode {}", instruction);
                offset + 1
            }
        }
    }
}

impl Display for Chunk {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
        Ok(())
    }
}
