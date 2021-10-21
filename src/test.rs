use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct Range {
    min: u32,
    max: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Test {
    name: String,
    indexes: Vec<Index>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Index {
    name: String,
    initials: String,
    equivalents_chart: Chart,
    subtests: Vec<Subtest>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Subtest {
    name: String,
    initials: String,
    score_range: Range,
    optional: bool,
    charts: Vec<Chart>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Chart {
    age_range: Range,
    scaled_score_range: Range,
    raw_score_maxes: Vec<u32>,
    percentile_ranks: Vec<f32>,
}