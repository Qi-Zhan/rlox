use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
fun foo(a) {
  var a; // Error at 'a': Already a variable with this name in this scope.
}

"#;

#[test]
fn test_files_variable_collide_with_parameter() {
    
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::CompilerError{..}));
}