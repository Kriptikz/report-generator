use std::io;
use crate::test::*;
use crate::client::*;

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

pub fn print_select_index(test: &mut Test) {
    println!("\nPlease select an index");
    print_loaded_indexes(&test);
}

pub fn print_loaded_indexes(test: &Test) {
    let mut indexes_list: String = String::new();

    match test.get_indexes() {
        Some(indexes) => {
            for index in indexes {
                indexes_list.push_str(index.get_name());
                indexes_list.push(',');
                indexes_list.push(' ');
            }
        }
        None => indexes_list.push_str(&String::from("None")),
    }

    println!("Loaded Indexes: {}", indexes_list);
}

pub fn print_select_subtest(index: &Index) {
    println!("\nPlease select a subtest");
    print_loaded_subtests(&index);
}

pub fn print_loaded_subtests(index: &Index) {
    let mut subtests_list: String = String::new();

    match index.get_subtests() {
        Some(subtests) => {
            for subtest in subtests {
                subtests_list.push_str(subtest.get_name());
                subtests_list.push(',');
                subtests_list.push(' ');
            }
        }
        None => subtests_list.push_str(&String::from("None")),
    }

    println!("Loaded subtests: {}", subtests_list);
}

pub fn print_select_client(clients: &Vec<Client>) {
    println!("\nPlease select a client");
    print_loaded_clients(clients);
}

pub fn print_loaded_clients(clients: &Vec<Client>) {
    let mut clients_list: String = String::new();

    for client in clients {
        clients_list.push_str(client.get_name());
        clients_list.push(',');
        clients_list.push(' ');
    }

    println!("Loaded Clients: {}", clients_list);
}
