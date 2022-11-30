use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
{
  var f;

  {
    var a = "a";
    fun f_() { print a; }
    f = f_;
  }

  {
    // Since a is out of scope, the local slot will be reused by b. Make sure
    // that f still closes over a.
    var b = "b";
    f(); // expect: a
  }
}

"#;

#[test]
fn test_files_closure_reuse_closure_slot() {
    let expected_output = vec!["a"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}