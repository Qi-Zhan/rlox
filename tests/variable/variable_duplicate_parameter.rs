use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
fun foo(arg,
        arg) { // Error at 'arg': Already a variable with this name in this scope.
  "body";
}

"#;

#[test]
fn test_files_variable_duplicate_parameter() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::CompileError{..}));
}
