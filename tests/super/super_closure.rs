use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
class Base {
  toString() { return "Base"; }
}

class Derived < Base {
  getClosure() {
    fun closure() {
      return super.toString();
    }
    return closure;
  }

  toString() { return "Derived"; }
}

var closure = Derived().getClosure();
print closure(); // expect: Base

"#;

#[test]
fn test_files_super_closure() {
    let expected_output = vec!["Base".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
