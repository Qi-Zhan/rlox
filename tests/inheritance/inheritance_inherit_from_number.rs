use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
var Number = 123;
class Foo < Number {} // expect runtime error: Superclass must be a class.

"#;

#[test]
fn test_files_inheritance_inherit_from_number() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::RuntimeError{..}));
}
