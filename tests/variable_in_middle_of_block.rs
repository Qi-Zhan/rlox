use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
{
  var a = "a";
  print a; // expect: a
  var b = a + " b";
  print b; // expect: a b
  var c = a + " c";
  print c; // expect: a c
  var d = b + " d";
  print d; // expect: a b d
}

"#;

#[test]
fn test_files_variable_in_middle_of_block() {
    let expected_output = vec!["a".to_string(),"a b".to_string(),"a c".to_string(),"a b d".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
