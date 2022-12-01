use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
fun foo() {}

class Subclass < foo {} // expect runtime error: Superclass must be a class.

"#;

#[test]
fn test_files_inheritance_inherit_from_function() {
    let expected_output = vec![];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::RuntimeError{..}));
}
