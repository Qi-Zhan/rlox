use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
class Base {
  foo() {
    super.doesNotExist; // Error at 'super': Can't use 'super' in a class with no superclass.
  }
}

Base().foo();

"#;

#[test]
fn test_files_super_no_superclass_bind() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::CompileError{..}));
}
