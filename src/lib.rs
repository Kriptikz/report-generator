use std::error::Error;
use std::io;

//mod report;
mod test;
mod client;
mod prints;

use crate::prints::*;
use crate::test::*;
use crate::client::*;

pub fn run() -> Result<(), Box<dyn Error>> {

    print_start_text();

    let mut tests: Vec<Test> = Vec::new();
    let mut clients: Vec<Client> = Vec::new();

    loop {
        print_enter_a_command();

        let input = get_input();

        let input = &input[..];

        match input {
            "exit" => break,
            "help" => print_help_menu(),
            "generate report" => generate_report(&clients, &tests),
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

fn generate_report(clients: &Vec<Client>, tests: &Vec<Test>) {
    
}