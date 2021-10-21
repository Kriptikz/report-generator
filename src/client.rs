use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Client {
    name: String,
    age: u32,
    test_scores: Vec<TestScore>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TestScore {
    test_name: String,
    index_name: String,
    subtest_name: String,
    score: u32,
}