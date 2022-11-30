use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
// This is a regression test. There was a bug where if an upvalue for an
// earlier local (here "a") was captured *after* a later one ("b"), then it
// would crash because it walked to the end of the upvalue list (correct), but
// then didn't handle not finding the variable.

fun f() {
  var a = "a";
  var b = "b";
  fun g() {
    print b; // expect: b
    print a; // expect: a
  }
  g();
}
f();

"#;

#[test]
fn test_files_closure_close_over_later_variable() {
    let expected_output = vec!["b","a"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}