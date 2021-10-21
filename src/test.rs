use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct Range {
    min: u32,
    max: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct Test {
    name: String,
    indexes: Vec<Index>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct Index {
    name: String,
    initials: String,
    equivalents_chart: Chart,
    subtests: Vec<Subtest>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct Subtest {
    name: String,
    initials: String,
    score_range: Range,
    optional: bool,
    charts: Vec<Chart>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct Chart {
    age_range: Range,
    scaled_score_range: Range,
    raw_score_maxes: Vec<u32>,
    percentile_ranks: Vec<f32>,
}

impl Test {
    pub fn new(new_name: String, new_indexes: Vec<Index>) -> Test {
        Test { 
            name: new_name,
            indexes: new_indexes,
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test_no_indexes() {
        let name = "name";

        let test1 = Test {
            name: name.to_string(),
            indexes: Vec::new(),
        };

        let test2 = Test::new(name.to_string());

        assert_eq!(test1, test2);
    }
}
