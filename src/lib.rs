use std::error::Error;
use std::io;
use docx_rs::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct Range {
    min: u32,
    max: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Test {
    name: String,
    indexes: Vec<Index>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Index {
    name: String,
    initials: String,
    subtests: Vec<Subtest>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Subtest {
    name: String,
    initials: String,
    score_range: Range,
    charts: Vec<Chart>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Chart {
    age_range: Range,
    scaled_score_range: Range,
    raw_score_maxes: Vec<u32>,
}

pub fn run() -> Result<(), Box<dyn Error>> {

    print_start_text();

    let mut tests: Vec<Test> = Vec::new();

    loop {
        println!("\nPlease enter a command, (\"help\" for a list of commands):");

        let input: &str = &get_input()[..];

        match input {
            "help" => help_menu(),
            "generate report" => 
                match generate_report() {
                    Err(_) => eprintln!("Error generating report"),
                    Ok(_) => eprintln!("Report generated successfully.")
                },
            "add test" => add_test(&mut tests),
            "save test" => save_test(&tests),
            "load test" => load_test(&mut tests),
            "show tests" => show_tests(&tests),
            "exit" => break,
            _ => println!("Error: No matching command: {}", input)
        }
    }

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
    println!("help             -    brings up this help menu.");
    println!("generate report  -    generates report for current client and chart data");
    println!("add test         -    add a new test into the program.");
    println!("save test        -    save test from program into a file.");
    println!("load test        -    load test from file into program.");
    println!("show tests       -    shows names for loaded tests.");
    println!("exit             -    exits the program.\n");

}

fn add_test(tests: &mut Vec<Test>) {
    println!("\nPlease enter a test name:");
    let input = get_input();

    if is_test_loaded(&input, &tests) {
        println!("Error: Test with name '{}' is already loaded. Maybe you want to 'add index', 'add subtest', or 'add chart'.", &input);
        return
    }

    let mut test = Test {
        name: String::from(input),
        indexes: Vec::new(),
    };

    println!("\nPlease enter an index name:");
    let name = get_input();

    if is_index_loaded(&name, &tests) {
        println!("Error: Index with name '{}' is already loaded. Maybe you want to 'add subtest' or 'add chart'.", &name);
        return
    }

    println!("\nPlease enter the index initials:");
    let initials = get_input();

    let index = Index {
        name: String::from(name),
        initials: String::from(initials),
        subtests: Vec::new(),
    };

    test.indexes.push(index);

    println!("\nPlease enter a subtest name:");
    let name = get_input();

    if is_subtest_loaded(&name, &tests) {
        println!("Error: Subtest with name '{}' is already loaded. Maybe you want to 'add index' or 'add chart'.", &name);
        return
    }

    println!("\nPlease enter the subtest initials:");
    let initials = get_input();
    println!("\nPlease enter the subtest score minimum:");
    let min: u32 = get_input().parse().expect("Please type a number!");
    println!("\nPlease enter the subtest score maximum:");
    let max: u32 = get_input().parse().expect("Please type a number!");

    let subtest = Subtest {
        name: String::from(name),
        initials: String::from(initials),
        score_range: Range{min: min, max: max},
        charts: Vec::new(),
    };

    test.indexes[0].subtests.push(subtest);

    println!("\nPlease enter the chart age range minimum:");
    let age_min: u32 = get_input().parse().expect("Please type a number!");
    println!("\nPlease enter the chart age range maximum:");
    let age_max: u32 = get_input().parse().expect("Please type a number!");

    let age_range = Range{min: age_min, max: age_max};

    if is_chart_loaded(&age_range, &tests){
        println!("Error: Chart with range '{:?}' is already loaded. Maybe you want to 'add subtest' or 'add index'.", &age_range);
        return
    }

    println!("\nPlease enter the scaled score range minimum:");
    let scaled_score_min: u32 = get_input().parse().expect("Please type a number!");
    println!("\nPlease enter the scaled score range maximum:");
    let scaled_score_max: u32 = get_input().parse().expect("Please type a number!");

    let mut maxes: Vec<u32> = Vec::new();

    for score in scaled_score_min..scaled_score_max + 1 {
        println!("Please enter the max raw score for scaled score of {}", score);
        let raw_score_max: u32 = get_input().parse().expect("Please type a number");

        maxes.push(raw_score_max);
    }

    let chart = Chart {
        age_range: age_range,
        scaled_score_range: Range{min: scaled_score_min, max: scaled_score_max},
        raw_score_maxes: maxes,
    };

    test.indexes[0].subtests[0].charts.push(chart);

    tests.push(test);
}

fn save_test(tests: &Vec<Test>) {
    println!("\nSaving tests...");
    let mut file = File::create(add_file_extension(&tests[0].name, "json")).expect("Error creating file");
    let serialized_data = serde_json::to_string(&tests[0]).expect("Error serializing data");

    file.write(serialized_data.as_bytes()).expect("Error writing file");
}

fn load_test(tests: &mut Vec<Test>) {
    println!("Loading test...");
}

fn show_tests(tests: &Vec<Test>) {
    println!("\nCurrently loaded tests:");
    for test in tests {
        println!("{}", test.name);
    }
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

fn get_input() -> String{
    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

    input.trim().to_string()
}

fn add_file_extension(name: &str, extension: &str) -> String {
    let mut filename = String::new();

    filename.push_str(name);
    filename.push_str(".");
    filename.push_str(extension);

    filename
}

fn is_test_loaded(name: &String, tests: &Vec<Test>) -> bool {
    for test in tests {
        if *name == test.name {
            return true
        }
    }

    false
}

fn is_index_loaded(name: &String, tests: &Vec<Test>) -> bool {

    for test in tests {
        for index in &test.indexes {
            if *name == index.name {
                return true
            }
        }
    }

    false
}

fn is_subtest_loaded(name: &String, tests: &Vec<Test>) -> bool {
    for test in tests {
        for index in &test.indexes {
            for subtest in &index.subtests {
                if *name == subtest.name {
                    return true
                }
            }
        }
    }

    false
}

fn is_chart_loaded(age_range: &Range, tests: &Vec<Test>) -> bool {
    for test in tests{
        for index in &test.indexes {
            for subtest in &index.subtests {
                for chart in &subtest.charts {
                    if *age_range == chart.age_range {
                        return true
                    }
                }
            }
        }
    }

    false
}
