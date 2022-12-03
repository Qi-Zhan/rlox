use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
class Foo {
  getClosure() {
    fun closure() {
      return this.toString();
    }
    return closure;
  }

  toString() { return "Foo"; }
}

var closure = Foo().getClosure();
print closure(); // expect: Foo

"#;

#[test]
fn test_files_this_closure() {
    let expected_output = vec!["Foo".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
