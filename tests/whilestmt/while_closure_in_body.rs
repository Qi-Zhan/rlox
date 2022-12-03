use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
var f1;
var f2;
var f3;

var i = 1;
while (i < 4) {
  var j = i;
  fun f() { print j; }

  if (j == 1) f1 = f;
  else if (j == 2) f2 = f;
  else f3 = f;

  i = i + 1;
}

f1(); // expect: 1
f2(); // expect: 2
f3(); // expect: 3

"#;

#[test]
#[ignore="closure"]
fn test_files_while_closure_in_body() {
    let expected_output = vec!["1".to_string(),"2".to_string(),"3".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
