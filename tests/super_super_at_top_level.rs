use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
super.foo("bar"); // Error at 'super': Can't use 'super' outside of a class.
super.foo; // Error at 'super': Can't use 'super' outside of a class.
"#;

#[test]
fn test_files_super_super_at_top_level() {
    let expected_output = vec![];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::CompileError{..}));
}
