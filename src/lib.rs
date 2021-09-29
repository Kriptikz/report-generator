use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    generate_report();
    Ok(())
}

fn generate_report() {
    let path = std::path::Path::new("./report.docx");
    let file = std::fs::File::create(&path).unwrap();

    
}