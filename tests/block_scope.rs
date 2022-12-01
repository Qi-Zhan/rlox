use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
var a = "outer";

{
  var a = "inner";
  print a; // expect: inner
}

print a; // expect: outer

"#;

#[test]
fn test_files_block_scope() {
    let expected_output = vec!["inner".to_string(),"outer".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
