use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
var f;

{
  var a = "a";
  fun f_() {
    print a;
    print a;
  }
  f = f_;
}

f();
// expect: a
// expect: a

"#;

#[test]
fn test_files_closure_reference_closure_multiple_times() {
    let expected_output = vec!["a","a"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}