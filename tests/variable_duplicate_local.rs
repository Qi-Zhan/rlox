use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
{
  var a = "value";
  var a = "other"; // Error at 'a': Already a variable with this name in this scope.
}

"#;

#[test]
fn test_files_variable_duplicate_local() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::CompileError{..}));
}
