use std::process;

fn main() {
    if let Err(e) = jareds_program::run() {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
