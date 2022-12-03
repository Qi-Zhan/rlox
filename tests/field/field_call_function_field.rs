use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
class Foo {}

fun bar(a, b) {
  print "bar";
  print a;
  print b;
}

var foo = Foo();
foo.bar = bar;

foo.bar(1, 2);
// expect: bar
// expect: 1
// expect: 2

"#;

#[test]
fn test_files_field_call_function_field() {
    let expected_output = vec!["bar".to_string(),"1".to_string(),"2".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
