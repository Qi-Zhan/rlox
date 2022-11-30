use crate::error::InterpretResult;
use crate::scanner::tokenize;

// #[cfg(not(test))]
pub fn run(string:&str) -> InterpretResult<Vec<&str>> {
    println!("{}", string);
    let tokens = tokenize(string);
    for token in tokens {
        println!("{:?}", token)
    }
    InterpretResult::Ok(vec![])
}

// #[cfg(test)]
// pub fn run(string:&str) -> InterpretResult<Vec<&str>> {
//     println!("{}", string);
//     let tokens = tokenize(string);
//     for token in tokens {
//         println!("{:?}", token)
//     }
//     InterpretResult::Ok(vec![])  // todo
// }
