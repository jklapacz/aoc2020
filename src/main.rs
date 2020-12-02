use std::io::{Error};

use yaml_rust::YamlLoader;

mod lib;
use lib::utils;

fn main() -> Result<(), Error> {
    let c = utils::read_lines("./inputs/01.yaml");
    let docs = YamlLoader::load_from_str(&c).unwrap();
    let scenarios = &docs[0]["scenarios"].as_vec().unwrap();
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

    Ok(())
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
