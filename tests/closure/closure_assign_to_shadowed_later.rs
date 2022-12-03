use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
var a = "global";

{
  fun assign() {
    a = "assigned";
  }

  var a = "inner";
  assign();
  print a; // expect: inner
}

print a; // expect: assigned

"#;

#[test]
fn test_files_closure_assign_to_shadowed_later() {
    let expected_output = vec!["inner".to_string(),"assigned".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
