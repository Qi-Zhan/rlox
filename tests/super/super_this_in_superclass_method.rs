use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
class Base {
  init(a) {
    this.a = a;
  }
}

class Derived < Base {
  init(a, b) {
    super.init(a);
    this.b = b;
  }
}

var derived = Derived("a", "b");
print derived.a; // expect: a
print derived.b; // expect: b

"#;

#[test]
fn test_files_super_this_in_superclass_method() {
    let expected_output = vec!["a".to_string(),"b".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
