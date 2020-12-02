use std::fs::File;
use std::io::{Error, Read};
use std::path::Path;
use std::fs;

use yaml_rust::YamlLoader;

fn main() -> Result<(), Error> {
    for entry in fs::read_dir("inputs")? {
        if let Ok(entry) = entry {
            let c = read_lines(entry.path());
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
                assert_eq!(
                    find_sum_to_2020(&input_vector),
                    scenario["outputs"][0].as_i64().unwrap()
                );
                assert_eq!(
                    find_sum_3_to_2020(&input_vector),
                    scenario["outputs"][1].as_i64().unwrap()
                );
            }
        }
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

fn read_lines<P>(filename: P) -> String
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
