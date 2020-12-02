use std::fs::File;
use std::io::Read;
use std::io::{self, BufRead, Error};
use std::path::Path;
use std::{env, fs};
use std::assert;
extern crate yaml_rust;
use yaml_rust::{Yaml, YamlEmitter, YamlLoader};

fn main() -> Result<(), Error> {
    let current_dir = env::current_dir()?;
    println!(
        "Entries modified in the last 24 hours in {:?}:",
        current_dir
    );

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
                assert_eq!(find_sum_to_2020(&input_vector), scenario["outputs"][0].as_i64().unwrap());
                assert_eq!(find_sum_3_to_2020(&input_vector), scenario["outputs"][1].as_i64().unwrap());
                // println!("sum to 2020: {:?}", find_sum_to_2020(&input_vector));
                // println!("sum 3 to 2020: {:?}", find_sum_3_to_2020(&input_vector));
                // println!("input {:?}", scenario["input"]);
                // println!("output {:?}", scenario["outputs"]);
            }
            // for scenario in &docs[0]["scenarios"] {

            // }
            // println!("{:?}", &docs.unwrap()[0]["scenarios"][0]["output"]);
            // println!("{}", c);
            // let mut file = File::open(entry.path()).expect("could not load");
            // let mut contents = String::new();

            // file.read_to_string(&mut contents).expect("could not read");

            // if let Ok(lines) = read_lines(entry.path()) {
            //     for line in lines {
            //         if let Ok(i) = line {
            //             println!("{}", i);
            //         }
            //     }
            // }
            // println!("{:?}", entry.path());
            // let buffered = BufReader::new(contents);
            // Ok(io::BufReader::new(file).lines())

            // for line in buffered.lines() {
            // println!("{}", line)
            // }
            // let input = fs::File.open(entry.path())?
            // println!("{:?}", entry)
        }
    }
    // let path = "input/01/part_01.txt";

    // let mut output = File::create(path)?;
    // write!(output, "Rust\n💖\nFun")?;

    // let input = File::open(path)?;
    // let buffered = BufReader::new(input);

    // for line in buffered.lines() {
    //     println!("{}", line?);
    // }

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
