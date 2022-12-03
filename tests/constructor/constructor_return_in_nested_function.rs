use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
class Foo {
  init() {
    fun init() {
      return "bar";
    }
    print init(); // expect: bar
  }
}

print Foo(); // expect: Foo instance

"#;

#[test]
fn test_files_constructor_return_in_nested_function() {
    let expected_output = vec!["bar".to_string(),"Foo instance".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
