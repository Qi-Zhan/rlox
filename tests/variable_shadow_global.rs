use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
var a = "global";
{
  var a = "shadow";
  print a; // expect: shadow
}
print a; // expect: global

"#;

#[test]
fn test_files_variable_shadow_global() {
    let expected_output = vec!["shadow".to_string(),"global".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
