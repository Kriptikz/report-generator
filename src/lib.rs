use std::error::Error;

mod report;

pub fn run() -> Result<(), Box<dyn Error>> {

    match report::generate_report() {
        Err(_) => eprintln!("Error generating report"),
        Ok(_) => eprintln!("Report generated successfully.")
    }

    Ok(())
}