use std::error::Error;
use std::fs;
use std::fs::File;

mod report;
mod menus;
mod test;
mod client_data;

pub use crate::test::Test;
pub use crate::menus::*;
pub use crate::client_data::*;


pub fn run() -> Result<(), Box<dyn Error>> {

    print_start_text();

    let mut tests: Vec<Test> = Vec::new();
    let mut clients: Vec<Client> = Vec::new();

    loop {
        println!("\nPlease enter a command, (\"help\" for a list of commands):");

        let input: &str = &get_input()[..];

        // Each one of these functions will be a wrapper for the struct implementations. This keeps things consistent and easily changeable.
        match input {
            "help" => help_menu(),
            "generate report" => generate_report(&mut clients, &mut tests),
            "add test" => add_test(&mut tests),
            "save test" => save_test(&tests),
            "load test" => load_test(&mut tests),
            "show loaded data" => show_loaded_data(&tests),
            "add index" => add_index(&mut tests),
            "add subtest" => add_subtest(&mut tests),
            "add chart" => add_chart(&mut tests),
            "add client" => add_client(&mut clients),
            "edit client" => edit_client(&mut clients),
            "save client" => save_client(&clients),
            "load client" => load_client(&mut clients),
            "exit" => break,
            _ => println!("Error: No matching command: {}", input)
        }
    }

    Ok(())
}
fn help_menu() {
    print_help_menu();
}

fn generate_report(clients: &mut Vec<Client>, tests: &mut Vec<Test>){
    print_select_test(tests);

    let input = get_input();
    let mut test_position: u32 = 0;
    if is_test_loaded(&input, &tests, &mut test_position) {
        let test: &mut Test = &mut tests[test_position as usize];

        println!("\nPlease enter a client name:");
        print_loaded_clients(clients);
        let name = get_input();
    
        let mut client_position: u32 = 0;
        if Client::is_client_loaded(&clients, &name, &mut client_position) {
            let client: &mut Client = &mut clients[client_position as usize];

            match report::generate_report(client, test) {
                Err(_) => eprintln!("Error generating report"),
                Ok(_) => eprintln!("Report generated successfully.")
            }
            
        } else {
            println!("Error: No client named {} is loaded.", name);
        }
        
    } else {
        println!("Test {} is not loaded, please use the 'add test' or 'load test' command first.", &input);
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
    print_loaded_tests(tests);
    let name = get_input();

    for test in tests {
        if test.name == name {
            let serialized_data = serde_json::to_string(&test).expect("Error serializing data");

            let mut full_file_path: String = String::new();
            full_file_path.push_str(&String::from("./saves/"));
            full_file_path.push_str(&test.name);
            full_file_path.push_str(&String::from(".json"));

            match fs::write(&full_file_path, serialized_data) {
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

    let mut full_file_path: String = String::new();
    full_file_path.push_str(&String::from("./saves/"));
    full_file_path.push_str(&input);
    full_file_path.push_str(&String::from(".json"));

    let file = File::open(full_file_path).expect("Error");

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

fn add_client(clients: &mut Vec<Client>) {
    //println!("\nPlease enter a client name:");
    //let name = get_input();
    //println!("\nPlease enter the clients age:");
    //let age = get_input().parse().expect("\nPlease enter a number!");
//
    //let mut client_position: u32 = 0;
    //if Client::is_client_loaded(&clients, &name, &mut client_position) {
    //    println!("Error: Client with name '{}' is already loaded.", &name);
    //    return
    //}
//
    //clients.push(Client::new(name, age));


    println!("\nPlease enter client info, use initials for Test, Index, and Subtest:");
    println!("\nFirst Last Age Test Index Subtest Score");
    let input = get_input();
    let mut client_name = String::new();
    let mut cname_is_set = false;
    let mut age_string = String::new();
    let mut age_is_set = false;
    let mut test_name = String::new();
    let mut tname_is_set = false;
    let mut index_name = String::new();
    let mut iname_is_set = false;
    let mut subtest_name = String::new();
    let mut sname_is_set = false;
    let mut score_string = String::new();
    let mut score_is_set = false;
    let mut skipped = false;

    for i in input.chars() {
        if i == ' ' {
            if skipped == false {
                skipped = true;
                client_name.push(i);
            }

            if !cname_is_set {
                cname_is_set = true;
            } else if !age_is_set {
                age_is_set = true;
            } else if !tname_is_set {
                tname_is_set = true;
            } else if !iname_is_set {
                iname_is_set = true;
            } else if !sname_is_set {
                sname_is_set = true;
            } else if !score_is_set {
                score_is_set = true;
            }

            continue;
        }


        if !cname_is_set {
            client_name.push(i);
            continue;
        } else if !age_is_set {
            age_string.push(i);
            continue;
        } else if !tname_is_set {
            test_name.push(i);
            continue;
        } else if !iname_is_set {
            index_name.push(i);
            continue;
        } else if !sname_is_set {
            subtest_name.push(i);
            continue;
        } else if !score_is_set {
            score_string.push(i);
        }

    }

    let score: u32 = score_string.parse().expect("Error: score_string is NaN!");
    let subtest = ClientSubtest::new(&subtest_name, score);
    let mut subtests = Vec::new();
    subtests.push(subtest);

    let index = ClientIndex::new(&index_name, subtests);
    let mut indexes = Vec::new();
    indexes.push(index);

    let test = ClientTest::new(&test_name, indexes);
    let mut tests = Vec::new();
    tests.push(test);

    let age: u32 = age_string.parse().expect("Error: age_string is NaN!");
    let client = Client::new(client_name, age, tests);
}

fn edit_client(clients: &mut Vec<Client>) {
    println!("\nWIP {:?}", clients);
}

fn save_client(clients: &Vec<Client>) {
    println!("\nPlease enter name of client to save (this will overwrite a saved client file with the same name):");
    print_loaded_clients(clients);
    let name = get_input();

    for client in clients {
        if client.name == name {
            let mut full_file_path: String = String::new();
            full_file_path.push_str(&String::from("./saves/"));
            full_file_path.push_str(&client.name);
            full_file_path.push_str(&String::from(".json"));
            let serialized_data = serde_json::to_string(&client).expect("Error serializing data");
            match fs::write(&full_file_path, serialized_data) {
                Ok(_) => println!("Save successful!"),
                Err(_) => eprintln!("Error saving test '{}'.", &name),
            }
        }
        else {
            println!("Error: No client named '{}' is loaded, you need to 'add client' first.", &name);
        }
    }
}

fn load_client(clients: &mut Vec<Client>) {
    println!("\nPlease enter the name of the client to load:");
    let input = get_input();

    let mut client_position: u32 = 0;
    if Client::is_client_loaded(&clients, &input, &mut client_position) {
        println!("Error: Test with name '{}' is already loaded. Maybe you want to 'add index', 'add subtest', or 'add chart'.", &input);
        return
    }

    let mut full_file_path: String = String::new();
    full_file_path.push_str(&String::from("./saves/"));
    full_file_path.push_str(&input);
    full_file_path.push_str(&String::from(".json"));

    let file = File::open(&full_file_path).expect("Error");

    let deserialized_data: Client = serde_json::from_reader(file).expect("Error");

    clients.push(deserialized_data);
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
