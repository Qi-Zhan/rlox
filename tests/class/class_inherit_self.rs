use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
class Foo < Foo {} // Error at 'Foo': A class can't inherit from itself.

"#;

#[test]
fn test_files_class_inherit_self() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::CompileError{..}));
}
