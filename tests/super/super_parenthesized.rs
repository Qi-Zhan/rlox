use rlox::interpreter::run;
use rlox::result::InterpretResult;

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
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::CompileError{..}));
}
