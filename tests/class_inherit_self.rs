use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
class Foo < Foo {} // Error at 'Foo': A class can't inherit from itself.

"#;

#[test]
fn test_files_class_inherit_self() {
    
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::CompilerError{..}));
}