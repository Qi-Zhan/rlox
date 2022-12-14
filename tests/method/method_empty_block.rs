use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
class Foo {
  bar() {}
}

print Foo().bar(); // expect: nil

"#;

#[test]
fn test_files_method_empty_block() {
    let expected_output = vec!["nil".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
