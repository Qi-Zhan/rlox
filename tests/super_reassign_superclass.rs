use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
class Base {
  method() {
    print "Base.method()";
  }
}

class Derived < Base {
  method() {
    super.method();
  }
}

class OtherBase {
  method() {
    print "OtherBase.method()";
  }
}

var derived = Derived();
derived.method(); // expect: Base.method()
Base = OtherBase;
derived.method(); // expect: Base.method()

"#;

#[test]
fn test_files_super_reassign_superclass() {
    let expected_output = vec!["Base.method()","Base.method()"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}