use serde::{Deserialize, Serialize};
pub use crate::menus::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Client {
    pub name: String,
    pub age: u32,
    pub tests: Vec<ClientTest>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientTest {
    pub name: String,
    pub indexes: Vec<ClientIndex>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientIndex {
    pub name: String,
    pub subtests: Vec<ClientSubtest>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientSubtest {
    pub name: String,
    pub score: u32,
}

impl Client {
    pub fn new(name: String, age: u32) -> Client {
        let mut client = Client {
            name: name,
            age: age,
            tests: Vec::new(),
        };

        loop {
            println!("\nWould you like to add a test?");
            let input = get_input();
        
            match &input[..] {
                "yes" => client.add_test(),
                "no" => break,
                _ => println!("\nEnter 'yes' or 'no'."),
            }
        }

        client
    }

    pub fn is_client_loaded(clients: &Vec<Client>, client_name: &String, client_position: &mut u32) -> bool {
        *client_position = 0;
        for client in clients {
            if client.name == *client_name {
                return true
            }

            *client_position += 1;
        }

        false
    }

    pub fn add_test(&mut self) {
        let test = ClientTest::new(self);
        self.tests.push(test);
    }

    pub fn has_test(&mut self, name: &String) -> Option<&mut ClientTest> {
        for test in &mut self.tests {
            if *name == test.name {
                return Some(test)
            }
        }


        None
    }

    pub fn get_score(&mut self, test_name: &String, index_name: &String, subtest_name: &String) -> Option<u32> {
        match &mut self.has_test(test_name) {
            Some(test) =>
                match test.has_index(index_name) {
                    Some(index) =>
                        match index.has_subtest(subtest_name) {
                            Some(subtest) => return Some(subtest.score),
                            None => println!("Error: no matching subtest named {}", subtest_name)
                        }
                    None => println!("Error: no matching index named {}", index_name)
                }
            None => println!("Error: no matching test named {}", test_name)
        }

        None
    }
}

impl ClientTest {
    pub fn new(client: &mut Client) -> ClientTest {
        let mut name;
        loop {
            println!("\nPlease enter test name:");
            name = get_input();

            match client.has_test(&name) {
                Some(_) => (),
                None => break,
            }

            println!("Error: ClientTest with name '{}' already exists.", &name);
        }

        let mut test = ClientTest {
            name: name,
            indexes: Vec::new(),
        };

        loop {
            println!("\nWould you like to add an index for this test?");
            let input = get_input();

            match &input[..] {
                "yes" => test.add_index(),
                "no" => break,
                _ => println!("\nEnter 'yes' or 'no'.")
            }
        }

        test
    }

    pub fn add_index(&mut self) {
        let index = ClientIndex::new(self);
        self.indexes.push(index);
    }

    pub fn has_index(&mut self, name: &String) -> Option<&mut ClientIndex> {
        for index in &mut self.indexes {
            if *name == index.name {
                return Some(index)
            }
        }

        None
    }
}

impl ClientIndex {
    pub fn new(test: &mut ClientTest) -> ClientIndex {
        let mut name;
        loop {
            println!("\nPlease enter the index name:");
            name = get_input();

            match test.has_index(&name) {
                Some(_) => (),
                None => break,
            }

            println!("Error: Index with name'{}' already exists.", &name);
        }

        let mut index = ClientIndex {
            name: name,
            subtests: Vec::new(),
        };

        loop {
            println!("\nWould you like to add a subtest score for this index?");
            let input = get_input();

            match &input[..] {
                "yes" => index.add_subtest(),
                "no" => break,
                _ => println!("\nEnter 'yes' or 'no'.")
            }
        }

        index
    }

    pub fn add_subtest(&mut self) {
        let subtest = ClientSubtest::new(self);
        self.subtests.push(subtest);
    }

    pub fn has_subtest(&mut self, name: &String) -> Option<&mut ClientSubtest> {
        for subtest in &mut self.subtests {
            if *name == subtest.name {
                return Some(subtest)
            }
        }

        None
    }
}

impl ClientSubtest {
    pub fn new(index: &mut ClientIndex) -> ClientSubtest {
        let mut name;
        loop {
            println!("\nPlease enter the subtest name:");
            name = get_input();

            match index.has_subtest(&name) {
                Some(_) => (),
                None => break,
            }

            println!("Error: Subtest with name'{}' already exists.", &name);
        }

        println!("Please enter the score: ");
        let score: u32 = get_input().parse().expect("Please enter a number!");

        ClientSubtest {
            name: name,
            score: score,
        }
    }
}
