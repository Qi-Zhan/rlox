use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
// Unicode characters are allowed in comments.
//
// Latin 1 Supplement: £§¶ÜÞ
// Latin Extended-A: ĐĦŋœ
// Latin Extended-B: ƂƢƩǁ
// Other stuff: ឃᢆ᯽₪ℜ↩⊗┺░
// Emoji: ☃☺♣

print "ok"; // expect: ok

"#;

#[test]
fn test_files_comments_unicode() {
    let expected_output = vec!["ok"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}