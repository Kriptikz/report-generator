use std::error::Error;
use std::io;

mod report;

pub fn run() -> Result<(), Box<dyn Error>> {

    loop {
        println!("\nPlease enter a command, (\"help\" for a list of commands):");

        let input = get_input();



        let input = &input[..];
        match input {
            "exit" => break,
            _ => println!("Error: Not maching command: {}", input),
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