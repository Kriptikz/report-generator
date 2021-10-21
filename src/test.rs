use serde::{Deserialize, Serialize};
pub use crate::menus::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Range {
    pub min: u32,
    pub max: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Test {
    pub name: String,
    pub indexes: Vec<Index>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Index {
    pub name: String,
    pub initials: String,
    pub equivalents_chart: Chart,
    pub subtests: Vec<Subtest>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Subtest {
    pub name: String,
    pub initials: String,
    pub score_range: Range,
    pub optional: bool,
    pub charts: Vec<Chart>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chart {
    pub age_range: Range,
    pub scaled_score_range: Range,
    pub raw_score_maxes: Vec<u32>,
    pub percentile_ranks: Vec<f32>,
}

impl Test {
    pub fn new(name: String) -> Test {
    
        let mut test = Test {
            name: name,
            indexes: Vec::new(),
        };
    
        loop {
            println!("\nWould you like to add an index?");
            let input = get_input();
        
            match &input[..] {
                "yes" => test.add_index(),
                "no" => break,
                _ => println!("\nEnter 'yes' or 'no'."),
            }
        }

        test
    }

    pub fn add_index(&mut self) {
        let index = Index::new(self);
        self.indexes.push(index);
    }

    pub fn has_index(&mut self, name: &String) -> Option<&mut Index> {
        for index in &mut self.indexes {
            if *name == index.name {
                return Some(index)
            }
        }
    
        None
    }

    //pub fn has_subtest(&mut self, index_name: &String, subtest_name: &String) -> Option<&mut Subtest> {
    //    match self.has_index(index_name) {
    //        Some(index) => 
    //            match index.has_subtest(subtest_name) {
    //                Some(subtest) => return Some(subtest),
    //                None => return None,
    //            },
    //        None => return None,
    //    }
    //}
//
    //pub fn has_chart(&mut self, index_name: &String, subtest_name: &String, age_range: &Range) -> Option<&mut Chart> {
    //    match self.has_index(index_name) {
    //        Some(index) => 
    //            match index.has_subtest(subtest_name) {
    //                Some(subtest) =>
    //                    match subtest.has_chart(age_range) {
    //                        Some(chart) => return Some(chart),
    //                        None => return None,
    //                    }
    //                None => return None,
    //            },
    //        None => return None,
    //    }
    //}
}

impl Index {
    pub fn new(test: &mut Test) -> Index {

        let mut name;
        loop {
            println!("\nPlease enter an index name:");
            name = get_input();

            match test.has_index(&name) {
                Some(_) => (),
                None => break,
            }

            println!("Error: Index with name '{}' already exists. Maybe you want to 'add subtest' or 'add chart'.", &name);
        }
    
        println!("\nPlease enter the index initials:");
        let initials = get_input();
    
        let mut index = Index {
            name: String::from(name),
            initials: String::from(&initials),
            equivalents_chart: create_equivalents_chart(),
            subtests: Vec::new(),
        };
    
        loop {
            println!("\nWould you like to add a subtest?");
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
        let subtest = Subtest::new(self);
        self.subtests.push(subtest);
    }

    pub fn has_subtest(&mut self, name: &String) -> Option<&mut Subtest> {
        for subtest in &mut self.subtests {
            if *name == subtest.name {
                return Some(subtest)
            }
        }
    
        None
    }
}

fn create_equivalents_chart() -> Chart {
    println!("\nPlease enter Sum of Scaled Scores minimum:");
    let min = get_input_u32();
    println!("\nPlease enter Sum of Scaled Scores maximum:");
    let max = get_input_u32();

    let mut equivalents: Vec<u32> = Vec::new();
    let mut ranks: Vec<f32> = Vec::new();
    
    for score in min..max + 1 {
        println!("Please enter the equivalent score for scaled score sum of {}", score);
        let equivalent_score: u32 = get_input_u32();
        println!("\nPlease enter the percentile rank for scaled score sum of {}", score);
        let rank: f32 = get_input_float();

        equivalents.push(equivalent_score);
        ranks.push(rank);
    }

    let chart = Chart {
        age_range: Range{min: 0,max: 0},
        scaled_score_range: Range {min, max},
        raw_score_maxes: equivalents,
        percentile_ranks: ranks,
    };

    chart

}

fn get_input_u32() -> u32 {
    loop {
        match get_input().parse::<u32>() {
            Ok(input) => return input,
            Err(_) => println!("Please enter a valid unsigned number!"),
        }
    }
}

fn get_input_float() -> f32 {
    loop {
        match get_input().parse::<f32>() {
            Ok(input) => return input,
            Err(_) => println!("Please enter valid decimal number"),
        }
    }
}

impl Subtest {
    pub fn new(index: &mut Index) -> Subtest {

        let mut name;
        loop {
            println!("\nPlease enter a subtest name:");
            name = get_input();

            match index.has_subtest(&name) {
                Some(_) => println!("Error: Index with name '{}' already exists. Maybe you want to 'add subtest' or 'add chart'.", &name),
                None => break,
            }

            
        }
    
        println!("\nPlease enter the subtest initials:");
        let initials = get_input();
        println!("\nPlease enter the subtest score minimum:");
        let min: u32 = get_input().parse().expect("Please type a number!");
        println!("\nPlease enter the subtest score maximum:");
        let max: u32 = get_input().parse().expect("Please type a number!");

        //println!("\nIs this subtest optional? Please enter 'yes' or 'no':");
        //let input = get_input().parse().expect("Please enter a bool!");
        let optional = false;

        let mut subtest = Subtest {
            name: String::from(name),
            initials: String::from(initials),
            score_range: Range{min: min, max: max},
            optional: optional,
            charts: Vec::new(),
        };
    
        loop {
            println!("\nWould you like to add age range chart data for this subtest?");
            let input = get_input();
        
            match &input[..] {
                "yes" => subtest.add_chart(),
                "no" => break,
                _ => println!("\nEnter 'yes' or 'no'.")
            }
        }

        subtest
    }

    pub fn add_chart(&mut self) {
        let chart = Chart::new(self);
        self.charts.push(chart);
    }

    fn has_chart(&mut self, age_range: &Range) -> Option<&mut Chart> {
        for chart in &mut self.charts {
            if *age_range == chart.age_range {
                return Some(chart)
            }
        }
    
        None
    }
}

impl Chart {
    pub fn new(subtest: &mut Subtest) -> Chart {
        let mut age_range: Range;
        loop {
            println!("\nPlease enter the chart age range minimum:");
            let age_min: u32 = get_input().parse().expect("Please type a number!");
            println!("\nPlease enter the chart age range maximum:");
            let age_max: u32 = get_input().parse().expect("Please type a number!");
        
            age_range = Range{min: age_min, max: age_max};

            match subtest.has_chart(&age_range) {
                Some(_) => println!("Error: Chart with range '{:?}' already exists. Maybe you want to 'add subtest' or 'add index'.", &age_range),
                None => break, 
            }
        }
    
        println!("\nPlease enter the scaled score range minimum:");
        let scaled_score_min: u32 = get_input().parse().expect("Please type a number!");
        println!("\nPlease enter the scaled score range maximum:");
        let scaled_score_max: u32 = get_input().parse().expect("Please type a number!");
    
        let mut maxes: Vec<u32> = Vec::new();
    
        for score in scaled_score_min..scaled_score_max + 1 {
            println!("Please enter the max raw score for scaled score of {}", score);
            let raw_score_max: u32 = get_input().parse().expect("Please type a number");
    
            maxes.push(raw_score_max);
        }
    
        let chart = Chart {
            age_range: age_range,
            scaled_score_range: Range{min: scaled_score_min, max: scaled_score_max},
            raw_score_maxes: maxes,
            percentile_ranks: Vec::new(),
        };
    
        chart
    }
}
