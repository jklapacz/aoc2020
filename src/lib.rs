pub mod utils {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;
    extern crate yaml_rust;
    use yaml_rust::{scanner::ScanError, Yaml, YamlLoader};

    pub fn read_yaml<P>(filename: P) -> Result<Vec<Yaml>, ScanError>
    where
        P: AsRef<Path>,
    {
        return YamlLoader::load_from_str(&read_lines(filename));
    }

    pub fn read_lines<P>(filename: P) -> String
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename);
        let mut contents = String::new();

        if let Ok(mut file) = file {
            match file.read_to_string(&mut contents) {
                Err(e) => println!("{:?}", e),
                _ => (),
            }
        }
        return contents;
    }
}

pub mod day01 {
    use super::utils;

    pub fn call(scenario: &str) {
        let scenario_yaml = utils::read_yaml(scenario).unwrap();
        let scenarios = &scenario_yaml[0]["scenarios"].as_vec().unwrap();
        for scenario in scenarios.iter() {
            let input_vector: Vec<i64> = scenario["input"]
                .as_str()
                .unwrap()
                .split("\n")
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect();
            let part_1 = find_sum_to_2020(&input_vector);
            assert_eq!(part_1, scenario["outputs"][0].as_i64().unwrap());
            let part_2 = find_sum_3_to_2020(&input_vector);
            assert_eq!(part_2, scenario["outputs"][1].as_i64().unwrap());
            println!("part 1 answer: {:?}", part_1);
            println!("part 2 answer: {:?}", part_2);
        }
    }

    fn find_sum_to_2020(expenses: &Vec<i64>) -> i64 {
        for x in expenses.iter() {
            for y in expenses.iter() {
                if x + y == 2020 {
                    return x * y;
                }
            }
        }
        return 0;
    }

    fn find_sum_3_to_2020(expenses: &Vec<i64>) -> i64 {
        for x in expenses.iter() {
            for y in expenses.iter() {
                for z in expenses.iter() {
                    if x + y + z == 2020 {
                        return x * y * z;
                    }
                }
            }
        }
        return 0;
    }
}
