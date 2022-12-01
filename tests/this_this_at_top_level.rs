use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
this; // Error at 'this': Can't use 'this' outside of a class.

"#;

#[test]
fn test_files_this_this_at_top_level() {
    let expected_output = vec![];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::CompileError{..}));
}
