use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct Client {
    name: String,
    age: u32,
    test_scores: Vec<TestScore>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct TestScore {
    test_name: String,
    index_name: String,
    subtest_name: String,
    score: u32,
}

impl Client {
    pub fn new(name: String, age: u32, test_scores: Vec<TestScore>) -> Client {
        Client { 
            name: name,
            age: age,
            test_scores: test_scores,
        }
    }

    fn get_name(&self) -> &String {
        &self.name
    }
}

#[cfg(test)]
mod testing
{
    use super::*;
    
    #[cfg(test)]
    mod client_methods {
        use super::*;

        #[test]
        pub fn new() {
            let name = "name";
            let age: u32 = 25;
    
            let client1 = Client {
                name: name.to_string(),
                age: age,
                test_scores: Vec::new(),
            };
    
            let client2 = Client::new(name.to_string(), age, Vec::new());
    
            assert_eq!(client1, client2);
        }
    
        #[test]
        fn get_name() {
            let name = "name";
            let age: u32 = 25;
    
            let client = Client::new(name.to_string(), age, Vec::new());
            let name2 = client.get_name();
    
            assert_eq!(name.to_string(), *name2);
        }
    
    }
}