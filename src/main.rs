use std::process::exit;
use std::{fs, env};

use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

use rlox::interpreter::run;

fn repl() -> Result<()> {
    let mut rl = Editor::<()>::new()?;
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                run(&line);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history("history.txt")
}

fn run_file(path:&str) {
    let content = fs::read_to_string(path)
        .expect("No such file!");
    run(content.as_str());
}

fn main(){
    let args:Vec<String> = env::args().collect();
    if args.len() == 1 {
        repl().unwrap();
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        eprintln!("Usage: rlox [path]");
        exit(64);
    }

}
