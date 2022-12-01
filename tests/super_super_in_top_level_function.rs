use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
  super.bar(); // Error at 'super': Can't use 'super' outside of a class.
fun foo() {
}
"#;

#[test]
fn test_files_super_super_in_top_level_function() {
    let expected_output = vec![];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::CompileError{..}));
}
