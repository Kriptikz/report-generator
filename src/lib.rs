use std::error::Error;
use std::io;
use std::fs;
use docx_rs::*;
use serde::{Deserialize, Serialize};
use std::fs::File;

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
            "show loaded data" => show_loaded_data(&tests),
            "add index" => add_index(&mut tests),
            "add subtest" => add_subtest(&mut tests),
            "add chart" => add_chart(&mut tests),
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
    println!("help              -    brings up this help menu.");
    println!("generate report   -    generates report for current client and chart data");
    println!("add test          -    add a new test into the program.");
    println!("add index         -    add a new index into a loaded test.");
    println!("add subtest       -    add a new subtest into a loaded index.");
    println!("add chart         -    add a new chart into a loaded subtest.");
    println!("save test         -    save test from program into a file.");
    println!("load test         -    load test from file into program.");
    println!("show loaded data  -    shows all currently loaded tests, indexes, subtests, and charts.");
    println!("exit              -    exits the program.\n");

}

fn add_index(tests: &mut Vec<Test>) {
    print_select_test(&tests);

    let input = get_input();
    let mut test_position: u32 = 0;
    if is_test_loaded(&input, &tests, &mut test_position) {
        add_index_to_test(&mut tests[test_position as usize]);
    }
    else {
        println!("Test {} is not loaded, please use the 'add test' or 'load test' command first.", &input);
    }
}

fn add_index_to_test(test: &mut Test) {
    println!("\nPlease enter an index name:");
    let name = get_input();
    let mut index_position: u32 = 0;
    if is_index_loaded(&name, &test, &mut index_position) {
        println!("Error: Index with name '{}' is already loaded. Maybe you want to 'add subtest' or 'add chart'.", &name);
        return
    }

    println!("\nPlease enter the index initials:");
    let initials = get_input();

    let mut index = Index {
        name: String::from(name),
        initials: String::from(initials),
        subtests: Vec::new(),
    };

    loop {
        println!("\nWould you like to add a subtest?");
        let input = get_input();
    
        match &input[..] {
            "yes" => add_subtest_to_index(&mut index),
            "no" => break,
            _ => println!("\nEnter 'yes' or 'no'.")
        }
    }

    test.indexes.push(index);
}

fn print_select_test(tests: &Vec<Test>) {
    println!("Please select a test: ");
    print_loaded_tests(&tests);
}

fn print_loaded_tests(tests: &Vec<Test>) {
    let mut tests_list: String = String::new();

    for test in tests {
        tests_list.push_str(&test.name);
        tests_list.push(',');
        tests_list.push(' ');
    }

    println!("Loaded Tests: {}", tests_list);
}

fn add_subtest(tests: &mut Vec<Test>) {
    print_select_test(&tests);

    let input = get_input();
    let mut test_position: u32 = 0;
    if is_test_loaded(&input, &tests, &mut test_position) {
        print_select_index(&tests[test_position as usize]);
        let input = get_input();

        let mut index_position = 0;
        let test = &mut tests[test_position as usize];
        if is_index_loaded(&input, &test, &mut index_position){
            let mut index = &mut test.indexes[index_position as usize];
            add_subtest_to_index(&mut index);
        }
        else {
            println!("Index {} is not loaded, please use the 'add index' or 'load test' command first.", &input);
        }
    }
    else {
        println!("Test {} is not loaded, please use the 'add test' or 'load test' command first.", &input);
    }
}

fn print_select_index(test: &Test) {
    println!("Please select an index: ");
    print_loaded_indexes(&test);
}

fn print_loaded_indexes(test: &Test) {
    let mut indexes_list: String = String::new();

    for index in &test.indexes {
        indexes_list.push_str(&index.name);
        indexes_list.push(',');
        indexes_list.push(' ');
    }

    println!("Loaded Indexes: {}", indexes_list);
}


fn add_subtest_to_index(index: &mut Index) {
    println!("\nPlease enter a subtest name:");
    let name = get_input();
    let mut subtest_position: u32 = 0;
    if is_subtest_loaded(&name, &index, &mut subtest_position) {
        println!("Error: Subtest with name '{}' is already loaded. Maybe you want to 'add index' or 'add chart'.", &name);
        return
    }

    println!("\nPlease enter the subtest initials:");
    let initials = get_input();
    println!("\nPlease enter the subtest score minimum:");
    let min: u32 = get_input().parse().expect("Please type a number!");
    println!("\nPlease enter the subtest score maximum:");
    let max: u32 = get_input().parse().expect("Please type a number!");

    let mut subtest = Subtest {
        name: String::from(name),
        initials: String::from(initials),
        score_range: Range{min: min, max: max},
        charts: Vec::new(),
    };

    loop {
        println!("\nWould you like to add age range chart data for this subtest?");
        let input = get_input();
    
        match &input[..] {
            "yes" => add_chart_to_subtest(&mut subtest),
            "no" => break,
            _ => println!("\nEnter 'yes' or 'no'.")
        }
    }

    index.subtests.push(subtest);
}

fn add_chart(tests: &mut Vec<Test>) {
    println!("\nWIP");
}

fn add_chart_to_subtest(subtest: &mut Subtest) {
    println!("\nPlease enter the chart age range minimum:");
    let age_min: u32 = get_input().parse().expect("Please type a number!");
    println!("\nPlease enter the chart age range maximum:");
    let age_max: u32 = get_input().parse().expect("Please type a number!");

    let age_range = Range{min: age_min, max: age_max};

    let mut chart_position: u32 = 0;
    if is_chart_loaded(&age_range, &subtest, &mut chart_position){
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

    subtest.charts.push(chart);
}

fn add_test(tests: &mut Vec<Test>) {
    println!("\nPlease enter a test name:");
    let input = get_input();

    let mut test_position: u32 = 0;
    if is_test_loaded(&input, &tests, &mut test_position) {
        println!("Error: Test with name '{}' is already loaded. Maybe you want to 'add index', 'add subtest', or 'add chart'.", &input);
        return
    }

    let mut test = Test {
        name: String::from(input),
        indexes: Vec::new(),
    };

    loop {
        println!("\nWould you like to add an index?");
        let input = get_input();
    
        match &input[..] {
            "yes" => add_index_to_test(&mut test),
            "no" => break,
            _ => println!("\nEnter 'yes' or 'no'."),
        }
    }

    tests.push(test);
}

fn save_test(tests: &Vec<Test>) {
    println!("\nPlease enter name of test to save (this will overwrite a saved test file with the same name):");
    let name = get_input();

    for test in tests {
        if test.name == name {
            let serialized_data = serde_json::to_string(&test).expect("Error serializing data");
            match fs::write(add_file_extension(&test.name, "json"), serialized_data) {
                Ok(_) => println!("Save successful!"),
                Err(_) => eprintln!("Error saving test '{}'.", &name),
            }
        }
        else {
            println!("Error: No test named '{}' is loaded, you need to 'add test' first.", &name);
        }
    }
}

fn load_test(tests: &mut Vec<Test>) {
    println!("\nPlease enter the name of the test to load:");
    let input = get_input();

    let mut test_position: u32 = 0;
    if is_test_loaded(&input, &tests, &mut test_position) {
        println!("Error: Test with name '{}' is already loaded. Maybe you want to 'add index', 'add subtest', or 'add chart'.", &input);
        return
    }

    let file = File::open(add_file_extension(&input[..], "json")).expect("Error");

    let deserialized_data: Test = serde_json::from_reader(file).expect("Error");

    tests.push(deserialized_data);
}

fn show_loaded_data(tests: &Vec<Test>) {
    println!("\nCurrently loaded data: ");
    println!("Test");
    println!("    Index");
    println!("        Subtest");
    println!("            Chart");

    for test in tests {
        println!("\n{}", test.name);
        for index in &test.indexes {
            println!("    {} ({})", index.name, index.initials);
            for subtest in &index.subtests {
                println!("        {} ({})", subtest.name, subtest.initials);
                for chart in &subtest.charts {
                    println!("            {}-{}", chart.age_range.min, chart.age_range.max);
                }
            }
        }
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

fn is_test_loaded(name: &String, tests: &Vec<Test>, test_position: &mut u32) -> bool {
    *test_position = 0;
    for test in tests {
        if *name == test.name {
            return true
        }
        *test_position += 1;
    }

    false
}

fn is_index_loaded(name: &String, test: &Test, index_position: &mut u32) -> bool {
    *index_position = 0;
    for index in &test.indexes {
        if *name == index.name {
            return true
        }
        *index_position += 1;
    }

    false
}

fn is_subtest_loaded(name: &String, index: &Index, subtest_position: &mut u32) -> bool {
    *subtest_position = 0;
    for subtest in &index.subtests {
        if *name == subtest.name {
            return true
        }
        *subtest_position += 1;
    }


    false
}

fn is_chart_loaded(age_range: &Range, subtest: &Subtest, chart_position: &mut u32) -> bool {
    *chart_position = 0;
    for chart in &subtest.charts {
        if *age_range == chart.age_range {
            return true
        }
        *chart_position += 1;
    }

    false
}
