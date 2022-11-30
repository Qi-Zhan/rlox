use rlox::interpreter::run;
use rlox::error::InterpretResult;

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
    let expected_output = vec!["Derived.init()","Base.init(a, b)"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}