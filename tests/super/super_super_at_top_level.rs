use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
super.foo("bar"); // Error at 'super': Can't use 'super' outside of a class.
super.foo; // Error at 'super': Can't use 'super' outside of a class.
"#;

#[test]
fn test_files_super_super_at_top_level() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::CompileError{..}));
}
