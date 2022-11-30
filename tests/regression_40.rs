use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
fun caller(g) {
  g();
  // g should be a function, not nil.
  print g == nil; // expect: false
}

fun callCaller() {
  var capturedVar = "before";
  var a = "a";

  fun f() {
    // Commenting the next line out prevents the bug!
    capturedVar = "after";

    // Returning anything also fixes it, even nil:
    //return nil;
  }

  caller(f);
}

callCaller();

"#;

#[test]
fn test_files_regression_40() {
    let expected_output = vec!["false"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}