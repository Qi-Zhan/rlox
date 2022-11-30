use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
class Foo {
  method() { }
}
var foo = Foo();
print foo.method; // expect: <fn method>

"#;

#[test]
fn test_files_method_print_bound_method() {
    let expected_output = vec!["<fn method>"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}