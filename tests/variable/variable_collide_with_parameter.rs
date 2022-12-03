use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
fun foo(a) {
  var a; // Error at 'a': Already a variable with this name in this scope.
}

"#;

#[test]
#[ignore = "function"]
fn test_files_variable_collide_with_parameter() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::CompileError{..}));
}