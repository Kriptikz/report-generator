use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
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

impl Range {
    pub fn new(min: u32, max: u32) -> Range {
        Range {
            min: min,
            max: max,
        }
    }

    fn get_min(&self) -> u32 {
        self.min
    }

    fn get_max(&self) -> u32 {
        self.max
    }
}



impl Test {
    pub fn new(name: String, indexes: Vec<Index>) -> Test {
        Test { 
            name: name,
            indexes: indexes,
        }
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_index(&self, name: String) -> Option<&Index> {
        for index in &self.indexes {
            if name == index.name {
                return Some(&index)
            }
        }

        None
    }

    fn get_indexes(&self) -> Option<&Vec<Index>> {
        if self.indexes.len() > 0 {
            return Some(&self.indexes)
        }

        None
    }
}

impl Index {
    pub fn new(name: String, initials: String, chart: Chart, subtests: Vec<Subtest>) -> Index {
        Index {
            name: name,
            initials: initials,
            equivalents_chart: chart,
            subtests: subtests,
        }
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_initials(&self) -> &String {
        &self.initials
    }

    fn get_equivalents_chart(&self) -> &Chart {
        &self.equivalents_chart
    }

    fn get_subtests(&self) -> Option<&Vec<Subtest>> {
        if self.subtests.len() > 0 {
            return Some(&self.subtests)
        }

        None
    }
}

impl Subtest {
    pub fn new(name: String, initials: String, score_range: Range, optional: bool, charts: Vec<Chart>) -> Subtest {
        Subtest{
            name: name,
            initials: initials,
            score_range: score_range,
            optional: optional,
            charts: charts,
        }
    } 

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_initials(&self) -> &String {
        &self.initials
    }

    fn get_score_range(&self) -> &Range {
        &self.score_range
    }

    fn is_optional(&self) -> bool {
        self.optional
    }

    fn get_charts(&self) -> Option<&Vec<Chart>> {
        if self.charts.len() > 0 {
            return Some(&self.charts)
        }

        None
    }
}

impl Chart {
    pub fn new(age_range: Range, scaled_score_range: Range, raw_score_maxes: Vec<u32>, percentile_ranks: Vec<f32>) -> Chart {
        Chart {
            age_range: age_range,
            scaled_score_range: scaled_score_range,
            raw_score_maxes: raw_score_maxes,
            percentile_ranks: percentile_ranks,
        }
    }

    fn get_age_range(&self) -> &Range {
        &self.age_range
    }

    fn get_scaled_score_range(&self) -> &Range {
        &self.scaled_score_range
    }

    fn get_raw_score_maxes(&self) -> &Vec<u32>{
        &self.raw_score_maxes
    }

    fn get_percentile_ranks(&self) -> &Vec<f32> {
        &self.percentile_ranks
    }
}


#[cfg(test)]
mod testing
{
    use super::*;
    fn default_chart() -> Chart{
        let range = &Range { min: 0, max: 0 };

        Chart {
            age_range: *range,
            scaled_score_range: *range,
            raw_score_maxes: Vec::new(),
            percentile_ranks: Vec::new(),
        }
    }
    
    #[cfg(test)]
    mod test_methods {
        use super::*;

        #[test]
        fn new() {
            let name = "name";
    
            let test1 = Test {
                name: name.to_string(),
                indexes: Vec::new(),
            };
    
            let test2 = Test::new(name.to_string(), Vec::new());
    
            assert_eq!(test1, test2);
        }
    
        #[test]
        fn get_name() {
            let name = "name";
    
            let test = Test::new(name.to_string(), Vec::new());
            let name2 = test.get_name();
    
            assert_eq!(name.to_string(), *name2);
        }
    
        #[test]
        fn get_index() {
            let test_name = "Test_name";
            let index_name = "Index_name";
            let index_initials = "IN";
            let mut indexes: Vec<Index> = Vec::new();
    
            let index = Index::new(index_name.to_string(), index_initials.to_string(), default_chart(), Vec::new());
            indexes.push(index);
            let index = Index::new(index_name.to_string(), index_initials.to_string(), default_chart(), Vec::new());
    
    
            let test = Test::new(test_name.to_string(), indexes);
    
    
            let index_error = Index::new("Error".to_string(), "ERROR".to_string(), default_chart(), Vec::new());
    
    
            let index2: &Index;
    
            match test.get_index(index_name.to_string()) {
                Some(index) => index2 = index,
                None => index2 = &index_error,
            }
    
            assert_eq!(index, *index2);
        }
    
        #[test]
        fn get_indexes() {
            let test_name = "Test_name";
            let index_name = "Index_name";
            let index_initials = "IN";
            let mut indexes_error: Vec<Index> = Vec::new();
    
            let mut indexes: Vec<Index> = Vec::new();
            let index = Index::new(index_name.to_string(), index_initials.to_string(), default_chart(), Vec::new());
            indexes.push(index);
    
            let index_error = Index::new("Error".to_string(), "ERROR".to_string(), default_chart(), Vec::new());
            indexes_error.push(index_error);
    
            let test = Test::new(test_name.to_string(), indexes);
    
            let mut indexes: Vec<Index> = Vec::new();
            let index = Index::new(index_name.to_string(), index_initials.to_string(), default_chart(), Vec::new());
            indexes.push(index);
    
            let indexes2: &Vec<Index>;
            match test.get_indexes() {
                Some(indexes) => indexes2 = &indexes,
                None => indexes2 = &indexes_error,
            }
    
            assert_eq!(indexes, *indexes2);
        }
    }
    
    #[cfg(test)]
    mod index_methods {
        use super::*;

        #[test]
        fn new() {
            let name = "name";
            let initials = "na";
            let range = &Range { min: 0, max: 0 };
    
            let eq_chart = Chart {
                age_range: *range,
                scaled_score_range: *range,
                raw_score_maxes: Vec::new(),
                percentile_ranks: Vec::new(),
            };
    
            let index1 = Index {
                name: name.to_string(),
                initials: initials.to_string(),
                equivalents_chart: eq_chart,
                subtests: Vec::new(),
            };
    
            let eq_chart = Chart {
                age_range: *range,
                scaled_score_range: *range,
                raw_score_maxes: Vec::new(),
                percentile_ranks: Vec::new(),
            };
    
            let index2 = Index::new(name.to_string(), initials.to_string(), eq_chart, Vec::new());
    
            assert_eq!(index1, index2);
        }

        #[test]
        fn get_name() {
            let name = "name";
            let initials = "na";
    
            let index = Index::new(name.to_string(), initials.to_string(), default_chart(), Vec::new());
            let name2 = index.get_name();
    
            assert_eq!(name.to_string(), *name2);
        }

        #[test]
        fn get_initials() {
            let name = "name";
            let initials = "na";
    
            let index = Index::new(name.to_string(), initials.to_string(), default_chart(), Vec::new());
            let initials2 = index.get_initials();
    
            assert_eq!(initials.to_string(), *initials2);
        }

        #[test]
        fn get_equivalents_chart() {
            let name = "name";
            let initials = "na";

            let range = &Range { min: 5, max: 10 };

            let equiv_chart = Chart {
                age_range: *range,
                scaled_score_range: *range,
                raw_score_maxes: Vec::new(),
                percentile_ranks: Vec::new(),
            };
    
            let index = Index::new(name.to_string(), initials.to_string(), equiv_chart, Vec::new());

            let equiv_chart = Chart {
                age_range: *range,
                scaled_score_range: *range,
                raw_score_maxes: Vec::new(),
                percentile_ranks: Vec::new(),
            };

            let equiv_chart2 = index.get_equivalents_chart();
    
            assert_eq!(equiv_chart, *equiv_chart2);
        }

        #[test]
        fn get_subests() {
            let index_name = "Index_name";
            let index_initials = "IN";
            let subtest_name = "Subtest_name";
            let subtest_initials = "SN";
            let range = &Range {min: 1, max: 12};
            let optional = false;
            let mut subtests_error: Vec<Subtest> = Vec::new();
    
            let mut subtests: Vec<Subtest> = Vec::new();
            let subtest = Subtest::new(subtest_name.to_string(), subtest_initials.to_string(), *range, optional, Vec::new());
            subtests.push(subtest);
    
            let subtest_error = Subtest::new("Error".to_string(), "ERROR".to_string(), *range, optional, Vec::new());
            subtests_error.push(subtest_error);
    
            let index = Index::new(index_name.to_string(), index_initials.to_string(), default_chart(), subtests);
    
            let mut subtests: Vec<Subtest> = Vec::new();
            let subtest = Subtest::new(subtest_name.to_string(), subtest_initials.to_string(), *range, optional, Vec::new());
            subtests.push(subtest);
    
            let subtests2: &Vec<Subtest>;
            match index.get_subtests() {
                Some(subtests) => subtests2 = &subtests,
                None => subtests2 = &subtests_error,
            }
    
            assert_eq!(subtests, *subtests2);
        }
    
    }

    #[cfg(test)]
    mod subtest_methods {
        use super::*;
        
        #[test]
        fn new() {
            let name = "name";
            let initials = "na";
            let range = &Range { min: 0, max: 0 };
            let optional = false;
    
            let subtest1 = Subtest {
                name: name.to_string(),
                initials: initials.to_string(),
                score_range: *range,
                optional: optional,
                charts: Vec::new(),
            };
    
            let subtest2 = Subtest::new(name.to_string(), initials.to_string(), *range, optional, Vec::new());
    
            assert_eq!(subtest1, subtest2);
        }

        #[test]
        fn get_name() {
            let name = "name";
            let initials = "na";

            let score_range = Range::new(2, 9);
            let optional = false;
    
            let subtest = Subtest::new(name.to_string(), initials.to_string(), score_range, optional, Vec::new());
            let name2 = subtest.get_name();
    
            assert_eq!(name.to_string(), *name2);
        }

        #[test]
        fn get_initials() {
            let name = "name";
            let initials = "na";
            let score_range = Range::new(2, 9);
            let optional = false;
    
            let subtest = Subtest::new(name.to_string(), initials.to_string(), score_range, optional, Vec::new());
            let initials2 = subtest.get_initials();
    
            assert_eq!(initials.to_string(), *initials2);
        }

        #[test]
        fn get_score_range() {
            let range = &Range::new(2, 9);
            
            let name = "name";
            let initials = "na";
            let score_range = *range;
            let optional = false;
    
            let subtest = Subtest::new(name.to_string(), initials.to_string(), score_range, optional, Vec::new());
            let score_range2 = subtest.get_score_range();
    
            assert_eq!(*range, *score_range2);
        }

        #[test]
        fn is_optional() {
            let range = &Range::new(2, 9);
            
            let name = "name";
            let initials = "na";
            let score_range = *range;
            let optional = false;
    
            let subtest = Subtest::new(name.to_string(), initials.to_string(), score_range, optional, Vec::new());
            let optional2 = subtest.is_optional();
    
            assert_eq!(optional, optional2);
        }

        #[test]
        fn get_charts() {
            let subtest_name = "Subtest_name";
            let subtest_initials = "SN";
            let range = &Range::new(1, 12);
            let optional = false;
            let raw_score_maxes: Vec<u32> = Vec::new();
            let percentile_ranks: Vec<f32> = Vec::new();

            let mut charts_error: Vec<Chart> = Vec::new();
    
            let mut charts: Vec<Chart> = Vec::new();
            let chart = Chart::new(*range, *range, raw_score_maxes, percentile_ranks);
            charts.push(chart);

            let raw_score_maxes: Vec<u32> = Vec::new();
            let percentile_ranks: Vec<f32> = Vec::new();
    
            let range2 = &Range::new(2,9);
            let chart_error = Chart::new(*range2, *range2, raw_score_maxes, percentile_ranks);
            charts_error.push(chart_error);

            let raw_score_maxes: Vec<u32> = Vec::new();
            let percentile_ranks: Vec<f32> = Vec::new();
    
            let subtest = Subtest::new(subtest_name.to_string(), subtest_initials.to_string(), *range, optional, charts);
    
            let mut charts: Vec<Chart> = Vec::new();
            let chart = Chart::new(*range, *range, raw_score_maxes, percentile_ranks);
            charts.push(chart);
    
            let charts2: &Vec<Chart>;
            match subtest.get_charts() {
                Some(charts) => charts2 = &charts,
                None => charts2 = &charts_error,
            }
    
            assert_eq!(charts, *charts2);
        }
        
    }

    #[cfg(test)]
    mod chart_methods {
        use super::*;

        #[test]
        fn new() {
            let range = &Range { min: 0, max: 0 };
    
            let chart1 = Chart {
                age_range: *range,
                scaled_score_range: *range,
                raw_score_maxes: Vec::new(),
                percentile_ranks: Vec::new(),
            };
    
            let chart2 = Chart::new(*range, *range, Vec::new(), Vec::new());
    
            assert_eq!(chart1, chart2);
        }

        #[test]
        fn get_age_range() {
            let age_range = &Range::new(20, 24);
            
            let scaled_score_range = &Range::new(1, 19);
            let raw_score_maxes: Vec<u32> = Vec::new();
            let percentile_ranks: Vec<f32> = Vec::new();
    
            let chart = Chart::new(*age_range, *scaled_score_range, raw_score_maxes, percentile_ranks);
            let age_range2 = chart.get_age_range();
    
            assert_eq!(*age_range, *age_range2);
        }

        #[test]
        fn get_scaled_score_range() {
            let age_range = &Range::new(20, 24);
            
            let scaled_score_range = &Range::new(1, 19);
            let raw_score_maxes: Vec<u32> = Vec::new();
            let percentile_ranks: Vec<f32> = Vec::new();
    
            let chart = Chart::new(*age_range, *scaled_score_range, raw_score_maxes, percentile_ranks);
            let scaled_score_range2 = chart.get_scaled_score_range();
    
            assert_eq!(*scaled_score_range, *scaled_score_range2);
        }

        #[test]
        fn get_raw_score_maxes() {
            let age_range = &Range::new(20, 24);
            
            let scaled_score_range = &Range::new(1, 19);
            let mut raw_score_maxes: Vec<u32> = Vec::new();
            raw_score_maxes.push(10);
            raw_score_maxes.push(12);
            let percentile_ranks: Vec<f32> = Vec::new();
    
            let chart = Chart::new(*age_range, *scaled_score_range, raw_score_maxes, percentile_ranks);

            let mut raw_score_maxes: Vec<u32> = Vec::new();
            raw_score_maxes.push(10);
            raw_score_maxes.push(12);

            let raw_score_maxes2 = chart.get_raw_score_maxes();
    
            assert_eq!(raw_score_maxes, *raw_score_maxes2);
        }

        #[test]
        fn get_percentile_ranks() {
            let age_range = &Range::new(20, 24);
            
            let scaled_score_range = &Range::new(1, 19);
            let raw_score_maxes: Vec<u32> = Vec::new();
            let mut percentile_ranks: Vec<f32> = Vec::new();
            percentile_ranks.push(10.0);
            percentile_ranks.push(12.3);
    
            let chart = Chart::new(*age_range, *scaled_score_range, raw_score_maxes, percentile_ranks);

            let mut percentile_ranks: Vec<f32> = Vec::new();
            percentile_ranks.push(10.0);
            percentile_ranks.push(12.3);

            let percentile_ranks2 = chart.get_percentile_ranks();
    
            assert_eq!(percentile_ranks, *percentile_ranks2);
        }
    
    }

    #[cfg(test)]
    mod range_methods {
        use super::super::*;

        #[test]
        fn new() {
            let range1 = Range { min: 0, max: 0};
    
            let range2 = Range::new(0,0);
    
            assert_eq!(range1, range2);
        }

        #[test]
        fn get_min() {
            let min: u32 = 4;
            let range1 = Range { min: min, max: 5};
    
            assert_eq!(range1.get_min(), min);
        }

        #[test]
        fn get_max() {
            let max: u32 = 8;
            let range1 = Range { min: 4, max: max};
    
            assert_eq!(range1.get_max(), max);
        }
    }
}
