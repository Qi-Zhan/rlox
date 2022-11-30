use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
class Foo {
  getClosure() {
    fun f() {
      fun g() {
        fun h() {
          return this.toString();
        }
        return h;
      }
      return g;
    }
    return f;
  }

  toString() { return "Foo"; }
}

var closure = Foo().getClosure();
print closure()()(); // expect: Foo

"#;

#[test]
fn test_files_this_nested_closure() {
    let expected_output = vec!["Foo"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}