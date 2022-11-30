use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
var f;
var g;

{
  var local = "local";
  fun f_() {
    print local;
    local = "after f";
    print local;
  }
  f = f_;

  fun g_() {
    print local;
    local = "after g";
    print local;
  }
  g = g_;
}

f();
// expect: local
// expect: after f

g();
// expect: after f
// expect: after g

"#;

#[test]
fn test_files_closure_assign_to_closure() {
    let expected_output = vec!["local","after f","after f","after g"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}