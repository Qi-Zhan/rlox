use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
undefined1.bar // expect runtime error: Undefined variable 'undefined1'.
  = undefined2;
"#;

#[test]
fn test_files_field_set_evaluation_order() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::RuntimeError{..}));
}
