use std::error::Error;
use std::io;

mod report;
mod test;
mod client;
mod prints;

use crate::prints::*;

pub fn run() -> Result<(), Box<dyn Error>> {

    loop {
        print_enter_a_command();

        let input = get_input();

        let input = &input[..];
        match input {
            "exit" => break,
            _ => println!("Error: No maching command: {}", input),
        }
    }

    Ok(())
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    input.trim().to_string()
}