use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
var f;

class Foo {
  method(param) {
    fun f_() {
      print param;
    }
    f = f_;
  }
}

Foo().method("param");
f(); // expect: param

"#;

#[test]
fn test_files_closure_close_over_method_parameter() {
    let expected_output = vec!["param".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
