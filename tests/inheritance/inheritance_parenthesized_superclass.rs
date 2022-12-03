use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
class Foo {}

// [line 4] Error at '(': Expect superclass name.
class Bar < (Foo) {}

"#;

#[test]
fn test_files_inheritance_parenthesized_superclass() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::CompileError{..}));
}
