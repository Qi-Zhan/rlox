use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
return "wat"; // Error at 'return': Can't return from top-level code.

"#;

#[test]
fn test_files_return_at_top_level() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::CompileError{..}));
}
