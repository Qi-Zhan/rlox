use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
fun f() {
  while (true) {
    var i = "i";
    fun g() { print i; }
    return g;
  }
}

var h = f();
h(); // expect: i

"#;

#[test]
fn test_files_while_return_closure() {
    let expected_output = vec!["i"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}