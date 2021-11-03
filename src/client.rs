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

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_test_score(&self, test_name: String, index_name: String, subtest_name: String) -> Option<u32> {
        for test_score in &self.test_scores {
            if test_score.test_name == test_name && test_score.index_name == index_name && test_score.subtest_name == subtest_name {
                return Some(test_score.score)
            }
        }

        None
    }

    pub fn get_test_scores(&self) -> Option<&Vec<TestScore>> {
        if self.test_scores.len() > 0 {
            return Some(&self.test_scores)
        }

        None
    }
}

impl TestScore {
    pub fn new(test_name: String, index_name: String, subtest_name: String, score: u32) -> TestScore {
        TestScore {
            test_name: test_name,
            index_name: index_name,
            subtest_name: subtest_name,
            score: score,
        }
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

        #[test]
        fn get_test_score() {
            let client_name = "client name";
            let client_age = 25;

            let test_name = "test name";
            let index_name = "index name";
            let subtest_name = "subtest name";
            let score: u32 = 35;

            let mut test_scores: Vec<TestScore> = Vec::new();
            let test_score1 = TestScore {
                test_name: test_name.to_string(),
                index_name: index_name.to_string(),
                subtest_name: subtest_name.to_string(),
                score: score,
            };
            test_scores.push(test_score1);

            let client = Client::new(client_name.to_string(), client_age, test_scores);

            let score2: u32;

            match client.get_test_score(String::from(test_name), String::from(index_name), String::from(subtest_name)) {
                Some(score) => score2 = score,
                None => score2 = 404,
            }

            assert_eq!(score, score2);

        }

        #[test]
        fn get_test_scores() {
            let client_name = "client name";
            let client_age = 25;

            let test_name = "test name";
            let index_name = "index name";
            let subtest_name = "subtest name";
            let score: u32 = 35;
            let mut test_scores_error: Vec<TestScore> = Vec::new();

            let mut test_scores: Vec<TestScore> = Vec::new();
            let test_score1 = TestScore {
                test_name: test_name.to_string(),
                index_name: index_name.to_string(),
                subtest_name: subtest_name.to_string(),
                score: score,
            };
            test_scores.push(test_score1);

            let client = Client::new(client_name.to_string(), client_age, test_scores);

            let test_score_error = TestScore {
                test_name: String::from("Error"),
                index_name: String::from("Error"),
                subtest_name: String::from("Error"),
                score: 0,
            };

            test_scores_error.push(test_score_error);

            let test_scores2: &Vec<TestScore>;

            match client.get_test_scores() {
                Some(test_scores) => test_scores2 = &test_scores,
                None => test_scores2 = &test_scores_error,
            }

            let mut test_scores: Vec<TestScore> = Vec::new();
            let test_score1 = TestScore {
                test_name: test_name.to_string(),
                index_name: index_name.to_string(),
                subtest_name: subtest_name.to_string(),
                score: score,
            };
            test_scores.push(test_score1);

            assert_eq!(test_scores, *test_scores2);

        }
    
    }

    #[cfg(test)]
    mod test_score_methods {
        use super::*;

        fn new() {
            let test_name = "test name";
            let index_name = "index name";
            let subtest_name = "subtest name";
            let score: u32 = 35;

            let test_score = TestScore {
                test_name: test_name.to_string(),
                index_name: index_name.to_string(),
                subtest_name: subtest_name.to_string(),
                score: score,
            };

            let test_score2 = TestScore::new(test_name.to_string(), index_name.to_string(), subtest_name.to_string(), score);

            assert_eq!(test_score, test_score2);

        }
    }
}