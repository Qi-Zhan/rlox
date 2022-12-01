use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
{
  print notDefined;  // expect runtime error: Undefined variable 'notDefined'.
}

"#;

#[test]
fn test_files_variable_undefined_local() {
    let expected_output = vec![];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::RuntimeError{..}));
}
