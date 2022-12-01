use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
class A {}

class B < A {
  method() {
    // [line 6] Error at ';': Expect '.' after 'super'.
    super;
  }
}

"#;

#[test]
fn test_files_super_super_without_dot() {
    let expected_output = vec![];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::LexError{..}));
}
