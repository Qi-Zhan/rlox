#![feature(try_trait_v2)]
#![feature(never_type)]
pub mod interpreter;
mod scanner;
mod compiler;
mod vm;
mod value;
mod chunk;
mod opcode;
pub mod result;
mod parser;
