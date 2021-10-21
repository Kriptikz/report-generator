use std::error::Error;

mod report;

pub fn run() -> Result<(), Box<dyn Error>> {

    loop {
        println!("\nPlease enter a command, (\"help\" for a list of commands):");

        let input: &str = &get_input()[..];


        match input {
            "exit" => break,
            _ => println!("Error: Not maching command: {}", input),
        }
    }

    Ok(())
}

fn get_input() -> String{
    String::from("get_input() not implemented.")
}