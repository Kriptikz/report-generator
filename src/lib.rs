use std::error::Error;
use std::fs;
use std::fs::File;

mod report;
mod menus;
mod test;

pub use crate::test::Test;
pub use crate::menus::*;


pub fn run() -> Result<(), Box<dyn Error>> {

    print_start_text();

    let mut tests: Vec<Test> = Vec::new();

    loop {
        println!("\nPlease enter a command, (\"help\" for a list of commands):");

        let input: &str = &get_input()[..];

        // Each one of these functions will be a wrapper for the struct implementations. This keeps things consistent and easily changeable.
        match input {
            "help" => help_menu(),
            "generate report" => generate_report(),
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
fn help_menu() {
    print_help_menu();
}

fn generate_report(){
    match report::generate_report() {
        Err(_) => eprintln!("Error generating report"),
        Ok(_) => eprintln!("Report generated successfully.")
    }
} 

fn add_index(tests: &mut Vec<Test>) {
    print_select_test(&tests);

    let input = get_input();
    let mut test_position: u32 = 0;
    if is_test_loaded(&input, &tests, &mut test_position) {
        tests[test_position as usize].add_index();
    }
    else {
        println!("Test {} is not loaded, please use the 'add test' or 'load test' command first.", &input);
    }
}

fn add_subtest(tests: &mut Vec<Test>) {
    print_select_test(&tests);

    let input = get_input();
    let mut test_position: u32 = 0;
    if is_test_loaded(&input, &tests, &mut test_position) {
        let test: &mut Test = &mut tests[test_position as usize];

        print_select_index(test);
        let input = get_input();

        match test.has_index(&input) {
            None => println!("Index {} is not loaded, please use the 'add index' or 'load test' command first.", &input),
            Some(index) => index.add_subtest(),
        }
    }
    else {
        println!("Test {} is not loaded, please use the 'add test' or 'load test' command first.", &input);
    }
}

fn add_chart(tests: &mut Vec<Test>) {
    print_select_test(&tests);

    let input = get_input();
    let mut test_position: u32 = 0;
    if is_test_loaded(&input, &tests, &mut test_position) {
        let test: &mut Test = &mut tests[test_position as usize];

        print_select_index(test);
        let input = get_input();

        let result: Option<&mut test::Index> = test.has_index(&input);

        match result {
            None => println!("Index {} is not loaded, please use the 'add index' or 'load test' command first.", &input),
            Some(index) => 
                {
                    print_select_subtest(index);
                    let input = get_input();
                    match index.has_subtest(&input) {
                        None => println!("Subtest {} is not loaded, please use the 'add subtest' or 'load test' command first.", &input),
                        Some(subtest) => subtest.add_chart(),
                    }
                }
        }
    }
    else {
        println!("Test {} is not loaded, please use the 'add test' or 'load test' command first.", &input);
    }
}

fn add_test(tests: &mut Vec<Test>) {
    println!("\nPlease enter a test name:");
    let input = get_input();

    let mut test_position: u32 = 0;
    if is_test_loaded(&input, &tests, &mut test_position) {
        println!("Error: Test with name '{}' is already loaded. Maybe you want to 'add index', 'add subtest', or 'add chart'.", &input);
        return
    }

    tests.push(Test::new(input));
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
