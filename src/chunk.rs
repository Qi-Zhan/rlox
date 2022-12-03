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
            OP_CONSTANT | 
            OP_DEFINE_GLOBAL | OP_GET_GLOBAL | OP_SET_GLOBAL |
            OP_GET_LOCAL | OP_SET_LOCAL => {
                let constant = self.code[offset + 1];
                println!("{} {:04} '{}'",opcode2string(instruction), constant, self.constants.values[constant as usize]);
                offset + 2
            }
            OP_JUMP_IF_FALSE | OP_JUMP | OP_LOOP => {
                let new_offset = (self.code[offset + 1] as usize) << 8 | self.code[offset + 2] as usize;
                println!("{} {:04}", opcode2string(instruction), new_offset + 3);
                offset + 3
            }
            opcode if is_binary_op(opcode) => {
                println!("{}", opcode2string(opcode));
                offset + 1
            }
            opcode if is_unary_op(opcode) => {
                println!("{}", opcode2string(opcode));
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
