use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
fun f() {
  for (;;) {
    var i = "i";
    fun g() { print i; }
    return g;
  }
}

var h = f();
h(); // expect: i

"#;

#[test]
fn test_files_for_return_closure() {
    let expected_output = vec!["i".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
