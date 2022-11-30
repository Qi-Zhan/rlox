use rlox::interpreter::run;
use rlox::error::InterpretResult;

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
    let expected_output = vec!["got method","arg"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}