use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
var Nil = nil;
class Foo < Nil {} // expect runtime error: Superclass must be a class.

"#;

#[test]
fn test_files_inheritance_inherit_from_nil() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::RuntimeError{..}));
}
