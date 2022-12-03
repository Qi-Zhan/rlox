use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
  super.bar(); // Error at 'super': Can't use 'super' outside of a class.
fun foo() {
}
"#;

#[test]
fn test_files_super_super_in_top_level_function() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::CompileError{..}));
}
