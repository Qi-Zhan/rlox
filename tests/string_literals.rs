use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
print "(" + "" + ")";   // expect: ()
print "a string"; // expect: a string

// Non-ASCII.
print "A~¶Þॐஃ"; // expect: A~¶Þॐஃ

"#;

#[test]
fn test_files_string_literals() {
    let expected_output = vec!["()","a string","A~¶Þॐஃ"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}