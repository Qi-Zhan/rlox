use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
var f1;
var f2;
var f3;

for (var i = 1; i < 4; i = i + 1) {
  var j = i;
  fun f() {
    print i;
    print j;
  }

  if (j == 1) f1 = f;
  else if (j == 2) f2 = f;
  else f3 = f;
}

f1(); // expect: 4
      // expect: 1
f2(); // expect: 4
      // expect: 2
f3(); // expect: 4
      // expect: 3

"#;

#[test]
fn test_files_for_closure_in_body() {
    let expected_output = vec!["4".to_string(),"1".to_string(),"4".to_string(),"2".to_string(),"4".to_string(),"3".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
