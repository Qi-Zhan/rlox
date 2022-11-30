use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
fun foo() {
  var a1;
  var a2;
  var a3;
  var a4;
  var a5;
  var a6;
  var a7;
  var a8;
  var a9;
  var a10;
  var a11;
  var a12;
  var a13;
  var a14;
  var a15;
  var a16;
  foo(); // expect runtime error: Stack overflow.
}

foo();

"#;

#[test]
fn test_files_limit_stack_overflow() {
    
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::RuntimeError{..}));
}