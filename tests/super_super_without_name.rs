use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
class A {}

class B < A {
  method() {
    super.; // Error at ';': Expect superclass method name.
  }
}

"#;

#[test]
fn test_files_super_super_without_name() {
    let expected_output = vec![];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::CompileError{..}));
}
