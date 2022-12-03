use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
class Base {}

class Derived < Base {
  foo() {
    super.doesNotExist(1); // expect runtime error: Undefined property 'doesNotExist'.
  }
}

Derived().foo();

"#;

#[test]
fn test_files_super_no_superclass_method() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::RuntimeError{..}));
}
