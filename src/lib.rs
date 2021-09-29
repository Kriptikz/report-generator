use std::error::Error;
use docx_rs::*;

pub fn run() -> Result<(), Box<dyn Error>> {
    generate_report();
    Ok(())
}

fn generate_report() {
    let path = std::path::Path::new("./report.docx");
    let file = std::fs::File::create(&path).unwrap();

    Docx::new()
        .add_table(
            Table::new(vec![
                TableRow::new(vec![
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Simple"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Table")))
                ]),
                TableRow::new(vec![
                    TableCell::new().add_paragraph(Paragraph::new()),
                    TableCell::new().add_paragraph(Paragraph::new())
                ]),
            ]) 
            .set_grid(vec![2000, 4000, 2000])
            .layout(TableLayoutType::Fixed)
            .indent(0));
}