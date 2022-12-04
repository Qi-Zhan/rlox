use crate::compiler::Compiler;
use crate::vm::VM;
use crate::result::InterpretResult;
use crate::scanner::tokenize;
use crate::value::FunctionType::Script;

pub fn run(string:&str) -> InterpretResult<Vec<String>> {
    // println!("{}", string);
    let tokens = tokenize(string);

    let mut compiler = Compiler::new("".to_string(), Script);
    let function = compiler.compile(tokens.collect())?;
    println!("{}", function.chunk);
    let mut vm = VM::new();
    let result = vm.interpreter(function)?;
    // println!("{:?}", result);
    InterpretResult::Ok(result)

}

