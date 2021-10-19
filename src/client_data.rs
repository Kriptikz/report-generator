#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Client {
    pub name: String,
    pub age: u32,
    pub test_scores: TestScore,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TestScore {
    pub test: String,
    pub indexes: Vec<IndexScore>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IndexScore {
    pub name: String,
    pub subtests: Vec<SubtestScore>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubtestScore {
    pub name: String,
    pub score: u32,
}