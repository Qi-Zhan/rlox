use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
print !true;     // expect: false
print !false;    // expect: true
print !!true;    // expect: true

print !123;      // expect: false
print !0;        // expect: false

print !nil;     // expect: true

print !"";       // expect: false

fun foo() {}
print !foo;      // expect: false

"#;

#[test]
#[ignore = "function"]
fn test_files_operator_not() {
    let expected_output = vec!["false".to_string(),"true".to_string(),"true".to_string(),"false".to_string(),"false".to_string(),"true".to_string(),"false".to_string(),"false".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
