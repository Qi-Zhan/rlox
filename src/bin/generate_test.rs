
use std::{fs, io::{Read, Write}, path};
use walkdir::WalkDir;
use regex::Regex;
use lazy_static::lazy_static;
use rlox::error::InterpretResult;

const TEST_DIR:&str = "tests";
const FILE_DIR:&str = "test_files";

// template file
lazy_static! {
    static ref TEMPLATE: String = {
        let mut file = fs::File::open("tests/template.rs").unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        content
    };
}

// regex
lazy_static! {
    static ref _EXPECTED_OUTPUT_PATTERN: Regex = Regex::new(r"// expect: ?(.*)").unwrap();
    static ref _EXPECTED_ERROR_PATTERN: Regex= Regex::new(r"// (Error.*)").unwrap();
    static ref _ERROR_LINE_PATTERN: Regex= Regex::new(r"// \[((java|c) )?line (\d+)\] (Error.*)").unwrap();
    static ref _EXPECTED_RUNTIME_ERROR_PATTERN: Regex= Regex::new(r"// expect runtime error: (.+)").unwrap();
    static ref _SYNTAX_ERROR_PATTERN: Regex= Regex::new(r"\[.*line (\d+)\] (Error.+)").unwrap();
    static ref _STACK_TRACE_PATTERN: Regex= Regex::new(r"\[line (\d+)\]").unwrap();
    static ref _NON_TEST_PATTERN: Regex= Regex::new(r"// nontest").unwrap();
}

/// Generates a rust test file for each test file in the given source code.
pub fn generate_test_file(file:&str) {
    // read content
    let src_content = read_content(file); 
    // extract comment to generate InterpreResult
    let result = extract_comment(&src_content);
    // generate test file content
    let test_file_content = generate_test_content(file, &src_content, result);
    // get test file name
    let test_file_name = generate_test_file_name(file);
    // save test file
    save_test_file(&test_file_name, &test_file_content);
}

fn generate_test_file_name(file:&str) -> String {
    let mut test_file_path = file.replace(".lox", ".rs").replace(FILE_DIR, TEST_DIR);
    let mut vec_path  = test_file_path.split(path::is_separator).collect::<Vec<&str>>();
    vec_path.remove(0);
    test_file_path = vec_path.join("_");
    test_file_path = "tests/".to_string() + &test_file_path;
    test_file_path
}

/// extract the expected output or error from a test file content
fn extract_comment(content:&str) -> InterpretResult<Vec<&str>> {
    let mut comment:Vec<&str> = vec![];
    let mut lines = content.lines();
    while let Some(line) = lines.next() {
        _EXPECTED_OUTPUT_PATTERN.captures(line).map(|captures| {
            let match_content = captures.get(1).unwrap().as_str();
            comment.push(match_content);
            // println!("get expect: {}", captures.get(1).unwrap().as_str());
        });
        if _EXPECTED_RUNTIME_ERROR_PATTERN.is_match(line) {
            return InterpretResult::<Vec<&str>>::RuntimeError("".to_string());
        }
        if _ERROR_LINE_PATTERN.is_match(line) || _SYNTAX_ERROR_PATTERN.is_match(line) {
            return InterpretResult::<Vec<&str>>::CompileError("".to_string());
        }
        if _EXPECTED_ERROR_PATTERN.is_match(line) {
            return InterpretResult::<Vec<&str>>::CompileError("".to_string());
        }
    }
    InterpretResult::<Vec<&str>>::Ok(comment)
}

/// read content from file path
fn read_content(path: &str) -> String {
    let mut file = fs::File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}

/// generate test file content from test file content and expected output/error
fn generate_test_content(src_file_name: &str, src_file_content: &String, result: InterpretResult<Vec<&str>>) -> String {
    let mut test_file_content = TEMPLATE.clone();
    // replace test name
    test_file_content = test_file_content.replace("_test_name_", &src_file_name.replace("/", "_"));
    test_file_content = test_file_content.replace(".lox", "");
    // replace SOURCE
    test_file_content = test_file_content.replace("r#\"\"#", &("r#\"\n".to_string() + src_file_content.as_str() + "\n\"#"));
    
    let origin_assert = "assert_eq!(result, InterpretResult::Ok(expected_output));";
    let origin_expect = "let expected_output = vec![];";

    match result {
        InterpretResult::Ok(expected_outputs) => {
            let outputs: Vec<String> = expected_outputs.into_iter().map(|output| {
               "\"".to_string() + output + "\"" + ".to_string()"
            }).collect();
            let outputs = outputs.join(",");
            let outputs = "vec![".to_string() + outputs.as_str() + "]";
            test_file_content = test_file_content.replace("vec![]", &outputs);
        },
        InterpretResult::CompileError(_) => {
            test_file_content = test_file_content.replace(origin_expect, "");
            test_file_content = test_file_content.replace(origin_assert, "assert!(matches!(result, InterpretResult::CompileError{..}));");
        },
        InterpretResult::LexError(_) => {
            test_file_content = test_file_content.replace(origin_expect, "");
            test_file_content = test_file_content.replace(origin_assert, "assert!(matches!(result, InterpretResult::LexError{..}));");
        },
        InterpretResult::RuntimeError(_) => {
            test_file_content = test_file_content.replace(origin_expect, "");
            test_file_content = test_file_content.replace(origin_assert, "assert!(matches!(result, InterpretResult::RuntimeError{..}));");
        },
        
    }
    test_file_content
}

fn save_test_file(path: &str, content: &str) {
    let mut file = fs::File::create(path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

fn show_all_file_paths(root:&str) -> Vec<String> {
    let mut files = Vec::new();
    for entry in WalkDir::new(root) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            files.push(entry.path().to_str().unwrap().to_string());
        }
    }

    files
}

fn main() {
    let files = show_all_file_paths(FILE_DIR).into_iter().filter(|file | {
        file.ends_with(".lox") && !file.contains("benchmark") && !file.contains("scanning")
    }).collect::<Vec<String>>();
    // generate_test_file(files[0].as_str());
    // generate_test_file("test_files/block/scope.lox")
    for file in &files { // for every test file
        println!("{file}");
        generate_test_file(file);
    }
}