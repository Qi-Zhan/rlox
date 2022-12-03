use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
class Foo {
  bar() { return this; }
  baz() { return "baz"; }
}

print Foo().bar().baz(); // expect: baz

"#;

#[test]
fn test_files_this_this_in_method() {
    let expected_output = vec!["baz".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
