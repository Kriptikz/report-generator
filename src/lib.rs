use std::error::Error;
use docx_rs::*;

pub fn run() -> Result<(), Box<dyn Error>> {

    match generate_report() {
        Err(_) => eprintln!("Error generating report"),
        Ok(_) => eprintln!("Report generated successfully.")
    }

    Ok(())
}

fn generate_report() -> Result<(), DocxError> {
    let path = std::path::Path::new("./report.docx");
    let file = std::fs::File::create(&path).unwrap();

    Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("WAIS-IV:").bold()))
        .add_table(
            Table::new(vec![
                TableRow::new(vec![
                    make_cell("Verbal Comprehension"),
                    make_cell("Standard Score"),
                    make_cell("Perceptual Reasoning"),
                    make_cell("Standard Score"),
                ]),
                TableRow::new(vec![
                    make_cell("Similarities (SI)"),
                    make_cell("16"),
                    make_cell("Block Design (BD)"),
                    make_cell("12"),
                ]),
                TableRow::new(vec![
                    make_cell("Vocabulary (VC)"),
                    make_cell("13"),
                    make_cell("Matrix Reasoning (MR)"),
                    make_cell("10"),
                ]),
                TableRow::new(vec![
                    make_cell("Information (IN)"),
                    make_cell("11"),
                    make_cell("Visual Puzzles (VP)"),
                    make_cell("10"),
                ]),
                TableRow::new(vec![
                    make_cell("Comprehension (CO)"),
                    make_cell("x"),
                    make_cell("Figure Weights (FW)"),
                    make_cell("x"),
                ]),
                TableRow::new(vec![
                    make_cell(""),
                    make_cell(""),
                    make_cell("Picture Completion (PC)"),
                    make_cell("x"),
                ]),
                TableRow::new(vec![
                    make_cell(""),
                    make_cell(""),
                    make_cell(""),
                    make_cell(""),
                ]),
                TableRow::new(vec![
                    make_cell("Working Memory"),
                    make_cell("Standard Score"),
                    make_cell("Processing Speed"),
                    make_cell("Standard Score"),
                ]),
                TableRow::new(vec![
                    make_cell("Digit Span (DS)"),
                    make_cell("10"),
                    make_cell("Symbol Search (SS)"),
                    make_cell("10"),
                ]),
                TableRow::new(vec![
                    make_cell("Arithmetic (AR)"),
                    make_cell("9"),
                    make_cell("Coding (CD)"),
                    make_cell("8"),
                ]),
                TableRow::new(vec![
                    make_cell("Letter-Number Seq. (LN)"),
                    make_cell("10"),
                    make_cell("Cancellation (CA)"),
                    make_cell("x"),
                ]),
            ]) 
            .set_grid(vec![2000, 4000, 2000])
            .layout(TableLayoutType::Fixed)
            .indent(0))
        .page_margin(PageMargin {top: 10, left: 1750, bottom: 10, right: 0, header: 10, footer: 10, gutter: 10})
        .build()
        .pack(file)?;

    Ok(())
}

fn make_paragraph(text: &str) -> Paragraph {
    Paragraph::new().add_run(Run::new().add_text(text))
}

fn make_cell(text: &str) -> TableCell {
    TableCell::new().add_paragraph(make_paragraph(text))
}