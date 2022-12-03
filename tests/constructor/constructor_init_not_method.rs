use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
class Foo {
  init(arg) {
    print "Foo.init(" + arg + ")";
    this.field = "init";
  }
}

fun init() {
  print "not initializer";
}

init(); // expect: not initializer

"#;

#[test]
fn test_files_constructor_init_not_method() {
    let expected_output = vec!["not initializer".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
