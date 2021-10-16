use std::error::Error;
use std::io;
use docx_rs::*;

pub fn run() -> Result<(), Box<dyn Error>> {

    print_start_text();

    loop {
        println!("Please enter a command, (\"help\" for a list of commands):");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: &str = input.trim();

        match input {
            "help" => help_menu(),
            "exit" => break,
            _ => println!("Error: No matching command: {}", input)
        }
    }

    //match generate_report() {
    //    Err(_) => eprintln!("Error generating report"),
    //    Ok(_) => eprintln!("Report generated successfully.")
    //}

    Ok(())
}

fn print_start_text() {
    println!("===================================================");
    println!("=============== Report Generator ==================");
    println!("===================================================\n");
}

fn help_menu() {
    println!("\n---------------------------------------------------");
    println!("---------------------  Help  ----------------------");
    println!("---------------------------------------------------");
    println!("\nreport_generator, version 0.0.1 - beta");
    println!("\nSimple docx report generator using custom chart data.\n");
    println!("\nCommands List:\n");
    println!("help    -    brings up this help menu.");
    println!("exit    -    exits the program.\n");

}

fn generate_report() -> Result<(), DocxError> {
    let path = std::path::Path::new("./report.docx");
    let file = std::fs::File::create(&path).unwrap();

    let description = "This is a simple description paragraph that is work in progress. This will be updated with actual text that the user will include. This is also a test for how line wrapping works.";

    Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("WAIS-IV:").bold()))
        .add_paragraph(make_paragraph(description, AlignmentType::Left, "none"))
        .add_paragraph(Paragraph::new())
        .add_table(
            Table::new(vec![
                TableRow::new(vec![
                    make_cell("Index", AlignmentType::Left, "single"),
                    make_cell("Standard Score", AlignmentType::Center, "single"),
                    make_cell("Percentile", AlignmentType::Center, "single"),
                    make_cell("Qualitative Description", AlignmentType::Center, "single"),
                ]),
                TableRow::new(vec![
                    make_cell("Verbal Comprehension (VCI)", AlignmentType::Left, "none"),
                    make_cell("118", AlignmentType::Center, "none"),
                    make_cell("88", AlignmentType::Center, "none"),
                    make_cell("High Average", AlignmentType::Center, "none"),
                ]),
                TableRow::new(vec![
                    make_cell("Perceptual Reasoning (PRI)", AlignmentType::Left, "none"),
                    make_cell("104", AlignmentType::Center, "none"),
                    make_cell("61", AlignmentType::Center, "none"),
                    make_cell("Average", AlignmentType::Center, "none"),
                ]),
                TableRow::new(vec![
                    make_cell("Working Memory (WMI)", AlignmentType::Left, "none"),
                    make_cell("97", AlignmentType::Center, "none"),
                    make_cell("42", AlignmentType::Center, "none"),
                    make_cell("Average", AlignmentType::Center, "none"),
                ]),
                TableRow::new(vec![
                    make_cell("Processing Speed (PSI)", AlignmentType::Left, "none"),
                    make_cell("94", AlignmentType::Center, "none"),
                    make_cell("34", AlignmentType::Center, "none"),
                    make_cell("Average", AlignmentType::Center, "none"),
                ]),
                TableRow::new(vec![
                    make_cell("Full Scale IQ (FSIQ)", AlignmentType::Left, "none"),
                    make_cell("106", AlignmentType::Center, "none"),
                    make_cell("66", AlignmentType::Center, "none"),
                    make_cell("Average", AlignmentType::Center, "none"),
                ]),
            ]) 
            .set_grid(vec![2000, 6500, 2000])
            .layout(TableLayoutType::Fixed)
            .indent(0))
        .add_paragraph(Paragraph::new())
        .add_table(
            Table::new(vec![
                TableRow::new(vec![
                    make_cell("Verbal Comprehension", AlignmentType::Left, "single"),
                    make_cell("Standard Score", AlignmentType::Center, "single"),
                    make_cell("Perceptual Reasoning", AlignmentType::Left, "single"),
                    make_cell("Standard Score", AlignmentType::Center, "single"),
                ]),
                TableRow::new(vec![
                    make_cell("Similarities (SI)", AlignmentType::Left, "none"),
                    make_cell("16", AlignmentType::Center, "none"),
                    make_cell("Block Design (BD)", AlignmentType::Left, "none"),
                    make_cell("12", AlignmentType::Center, "none"),
                ]),
                TableRow::new(vec![
                    make_cell("Vocabulary (VC)", AlignmentType::Left, "none"),
                    make_cell("13", AlignmentType::Center, "none"),
                    make_cell("Matrix Reasoning (MR)", AlignmentType::Left, "none"),
                    make_cell("10", AlignmentType::Center, "none"),
                ]),
                TableRow::new(vec![
                    make_cell("Information (IN)", AlignmentType::Left, "none"),
                    make_cell("11", AlignmentType::Center, "none"),
                    make_cell("Visual Puzzles (VP)", AlignmentType::Left, "none"),
                    make_cell("10", AlignmentType::Center, "none"),
                ]),
                TableRow::new(vec![
                    make_cell("Comprehension (CO)", AlignmentType::Left, "none"),
                    make_cell("x", AlignmentType::Center, "none"),
                    make_cell("Figure Weights (FW)", AlignmentType::Left, "none"),
                    make_cell("x", AlignmentType::Center, "none"),
                ]),
                TableRow::new(vec![
                    make_cell("", AlignmentType::Left, "none"),
                    make_cell("", AlignmentType::Left, "none"),
                    make_cell("Picture Completion (PC)", AlignmentType::Left, "none"),
                    make_cell("x", AlignmentType::Center, "none"),
                ]),
                TableRow::new(vec![
                    make_cell("Working Memory", AlignmentType::Left, "single"),
                    make_cell("Standard Score", AlignmentType::Center, "single"),
                    make_cell("Processing Speed", AlignmentType::Left, "single"),
                    make_cell("Standard Score", AlignmentType::Center, "single"),
                ]),
                TableRow::new(vec![
                    make_cell("Digit Span (DS)", AlignmentType::Left, "none"),
                    make_cell("10", AlignmentType::Center, "none"),
                    make_cell("Symbol Search (SS)", AlignmentType::Left, "none"),
                    make_cell("10", AlignmentType::Center, "none"),
                ]),
                TableRow::new(vec![
                    make_cell("Arithmetic (AR)", AlignmentType::Left, "none"),
                    make_cell("9", AlignmentType::Center, "none"),
                    make_cell("Coding (CD)", AlignmentType::Left, "none"),
                    make_cell("8", AlignmentType::Center, "none"),
                ]),
                TableRow::new(vec![
                    make_cell("Letter-Number Seq. (LN)", AlignmentType::Left, "none"),
                    make_cell("10", AlignmentType::Center, "none"),
                    make_cell("Cancelation (CA)", AlignmentType::Left, "none"),
                    make_cell("x", AlignmentType::Center, "none"),
                ]),
            ]) 
            .set_grid(vec![2000, 6500, 2000])
            .layout(TableLayoutType::Fixed)
            .indent(0))
        .page_margin(PageMargin {top: 10, left: 500, bottom: 10, right: 500, header: 10, footer: 10, gutter: 10})
        .build()
        .pack(file)?;

    Ok(())
}

fn make_paragraph(text: &str, text_alignment: AlignmentType, underline_line_type: &str) -> Paragraph {
    Paragraph::new().add_run(Run::new().add_text(text).underline(underline_line_type)).align(text_alignment)
}

fn make_cell(text: &str, text_alignment: AlignmentType, underline_line_type: &str) -> TableCell {
    TableCell::new().add_paragraph(make_paragraph(text, text_alignment, underline_line_type))
}