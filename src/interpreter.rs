use crate::compiler::Compiler;
use crate::vm::VM;
use crate::result::InterpretResult;
use crate::scanner::tokenize;


pub fn run(string:&str) -> InterpretResult<Vec<String>> {
    // println!("{}", string);
    let tokens = tokenize(string);

    let mut compiler = Compiler::new();
    let function = compiler.compile(tokens)?;
    println!("{}", function.chunk);
    let mut vm = VM::new();
    let result = vm.interpreter(function)?;
    // println!("{:?}", result);
    InterpretResult::Ok(result)

}

