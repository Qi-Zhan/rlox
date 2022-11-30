use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
fun f(a, b) {
  print a;
  print b;
}

f(1, 2, 3, 4); // expect runtime error: Expected 2 arguments but got 4.

"#;

#[test]
fn test_files_function_extra_arguments() {
    
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::RuntimeError{..}));
}