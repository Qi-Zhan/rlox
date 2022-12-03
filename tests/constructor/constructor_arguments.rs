use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
class Foo {
  init(a, b) {
    print "init"; // expect: init
    this.a = a;
    this.b = b;
  }
}

var foo = Foo(1, 2);
print foo.a; // expect: 1
print foo.b; // expect: 2

"#;

#[test]
fn test_files_constructor_arguments() {
    let expected_output = vec!["init".to_string(),"1".to_string(),"2".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
