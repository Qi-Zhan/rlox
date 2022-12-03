use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
class Foo {
  method() { }
}
var foo = Foo();
print foo.method; // expect: <fn method>

"#;

#[test]
fn test_files_method_print_bound_method() {
    let expected_output = vec!["<fn method>".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
