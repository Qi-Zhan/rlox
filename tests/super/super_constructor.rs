use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
class Base {
  init(a, b) {
    print "Base.init(" + a + ", " + b + ")";
  }
}

class Derived < Base {
  init() {
    print "Derived.init()";
    super.init("a", "b");
  }
}

Derived();
// expect: Derived.init()
// expect: Base.init(a, b)

"#;

#[test]
fn test_files_super_constructor() {
    let expected_output = vec!["Derived.init()".to_string(),"Base.init(a, b)".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
