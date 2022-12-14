use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
class Foo {
  bar(arg) {
    print arg;
  }
}

var bar = Foo().bar;
print "got method"; // expect: got method
bar("arg");          // expect: arg

"#;

#[test]
fn test_files_field_method() {
    let expected_output = vec!["got method".to_string(),"arg".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
