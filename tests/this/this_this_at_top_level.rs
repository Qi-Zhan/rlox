use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
this; // Error at 'this': Can't use 'this' outside of a class.

"#;

#[test]
fn test_files_this_this_at_top_level() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::CompileError{..}));
}
