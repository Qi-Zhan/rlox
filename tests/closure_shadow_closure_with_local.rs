use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
{
  var foo = "closure";
  fun f() {
    {
      print foo; // expect: closure
      var foo = "shadow";
      print foo; // expect: shadow
    }
    print foo; // expect: closure
  }
  f();
}

"#;

#[test]
fn test_files_closure_shadow_closure_with_local() {
    let expected_output = vec!["closure","shadow","closure"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}