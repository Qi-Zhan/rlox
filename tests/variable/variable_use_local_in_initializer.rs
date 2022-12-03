use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
var a = "outer";
{
  var a = a; // Error at 'a': Can't read local variable in its own initializer.
}

"#;

#[test]
fn test_files_variable_use_local_in_initializer() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::CompileError{..}));
}
