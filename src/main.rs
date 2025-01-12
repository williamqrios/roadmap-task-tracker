use std::process;
use task_tracker::run;

fn main() {
    if let Err(e) = run() {
        println!("Application error: {e}");
        process::exit(1);
    }
}

// TODO: things to do 
// 2. create tests (TDD)