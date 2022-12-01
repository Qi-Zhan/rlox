use walkdir::WalkDir;
use tabled::{Table, Tabled, Style};

const SRC: &str = "src";

#[derive(Tabled)]
struct FileStats {
    path: String,
    lines: usize,
    code: usize,
    comments: usize,
    blanks: usize,
    tests: usize,
}

fn main () {

    let mut files = vec![];
    let mut filestats = vec![];
    for entry in WalkDir::new(SRC) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            files.push(entry.path().to_str().unwrap().to_string());
        }
    }

    let mut lines_all = 0;
    let mut code_all = 0;
    let mut comments_all = 0;
    let mut blanks_all = 0;
    let mut tests_all = 0;

    for file in files {
        let mut lines = 0;
        let mut code = 0;
        let mut comments = 0;
        let mut blanks = 0;
        let mut tests = 0;
        let mut in_comment = false;
        let mut in_test = false;

        let content = std::fs::read_to_string(&file).unwrap();

        for line in content.lines() {
            lines += 1;
            if line.trim().is_empty() {
                blanks += 1;
            } else if line.trim().starts_with("//") {
                comments += 1;
            } else if line.trim().starts_with("/*") {
                comments += 1;
                in_comment = true;
                if line.trim().ends_with("*/") {
                    in_comment = false;
                }
            } else if line.trim().ends_with("*/") {
                comments += 1;
                in_comment = false;
            } else if in_comment {
                comments += 1;
            } else if line.trim().starts_with("#[cfg(test)]") || in_test {
                in_test = true;
                tests += 1;
            } else {
                code += 1;
            }
        }

        filestats.push(FileStats {
            path: file,
            lines,
            code,
            comments,
            blanks,
            tests,
        });

        lines_all += lines;
        code_all += code;
        comments_all += comments;
        blanks_all += blanks;
        tests_all += tests;

    }

    filestats.push(FileStats {
        path: "Total".to_string(),
        lines: lines_all,
        code: code_all,
        comments: comments_all,
        blanks: blanks_all,
        tests: tests_all,
    });

    filestats.sort_by(|a, b| b.lines.cmp(&a.lines));
    let mut table = Table::new(&filestats);
    table.with(Style::modern());
    println!("{}", table);

}