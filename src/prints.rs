use std::io;
use crate::test::*;

pub fn print_enter_a_command() {
    println!("\nPlease enter a command, (\"help\" for a list of commands):");
}

pub fn print_start_text() {
    println!("===================================================");
    println!("=============== Report Generator ==================");
    println!("===================================================\n");
}

pub fn print_help_menu() {
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
    println!("show list         -    shows all currently loaded tests, indexes, subtests, and charts.");
    println!("add client        -    add a new client into the program.");
    println!("save client       -    save a client from program into a file." );
    println!("load client       -    load a client from a file into the program.");
    println!("exit              -    exits the program.\n");
}

pub fn print_select_a_test(tests: &Vec<Test>) {
    println!("\nPlease select a test");
    print_loaded_tests(tests);
}

pub fn print_loaded_tests(tests: &Vec<Test>) {
    let mut tests_list: String = String::new();

    for test in tests {
        tests_list.push_str(&test.get_name());
        tests_list.push(',');
        tests_list.push(' ');
    }

    println!("Loaded Tests: {}", tests_list);
}