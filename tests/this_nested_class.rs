use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
class Outer {
  method() {
    print this; // expect: Outer instance

    fun f() {
      print this; // expect: Outer instance

      class Inner {
        method() {
          print this; // expect: Inner instance
        }
      }

      Inner().method();
    }
    f();
  }
}

Outer().method();

"#;

#[test]
fn test_files_this_nested_class() {
    let expected_output = vec!["Outer instance","Outer instance","Inner instance"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}