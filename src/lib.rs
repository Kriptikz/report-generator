use std::error::Error;
use docx_rs::*;

pub fn run() -> Result<(), Box<dyn Error>> {
    generate_report();
    Ok(())
}

fn generate_report() {
    let path = std::path::Path::new("./report.docx");
    let file = std::fs::File::create(&path).unwrap();

    Docx::new();
}