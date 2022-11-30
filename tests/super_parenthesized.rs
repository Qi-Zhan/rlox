use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
class A {
  method() {}
}

class B < A {
  method() {
    // [line 8] Error at ')': Expect '.' after 'super'.
    (super).method();
  }
}

"#;

#[test]
fn test_files_super_parenthesized() {
    
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::LexerError{..}));
}