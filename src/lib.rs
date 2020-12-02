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

pub mod day02 {
    use super::utils;
    use nom::character::complete::{char, digit1, one_of, alpha1,alpha0, multispace0};
    use nom::multi::{many0, many1};
    use nom::bytes::complete::tag;
    use nom::combinator::{map_res, opt, not};
    use nom::sequence::{terminated, tuple, delimited};
    use nom::number::complete::double;
    use std::num::ParseIntError;
    use nom::branch::alt;
    use nom::error::ParseError;

    use nom::{combinator::recognize, sequence::separated_pair, IResult};

    struct Problem {
        min: u8,
        max: u8,
        password_char: char,
        password: String,
    }

    fn decimal(input: &str) -> IResult<&str, &str> {
        recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
    }

    fn to_int(input: &str) -> Result<u32, std::num::ParseIntError> {
        return u32::from_str_radix(input, 10);
    }

    fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
    where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
    {
        delimited(
            multispace0,
            inner,
            multispace0
        )
    }

    fn identifier(input: &str) -> IResult<&str, &str> {
        recognize(
                alpha1,
            
        )(input)
    }

    fn a0(input: &str) -> IResult<&str, &str> {
        recognize(
                alpha0,
            
        )(input)
    }

    fn a1(input: &str) -> IResult<&str, &str> {
        alt((alpha1, tag(":")))(input)
    }

    fn parse(line: &str) {
        let mut parser = separated_pair(decimal, tag("-"), decimal);
        
        // let mut parser = recognize(separated_pair(digit1, char('-'), digit1));
        // let mut char_parser = recognize
        let (next, res) = parser(line).unwrap();
        let (left, right) = res;
        let (almost_a, almost_b) = ws(identifier)(next).unwrap();
        // let this_is_it = separated_pair(opt(a0), tag(":"), alpha1)(almost_a);
        let this_is_it = ws(a1)(almost_a);
        println!("left: {:?}", left);
        println!("right: {:?}", right);
        println!("almost_a: {:?}", almost_a);
        println!("almost_b: {:?}", almost_b);
        println!("{:?}", this_is_it);
        // if let Ok(res) = res {
        // let (rest, min_max) = res;
        // println!("{:?}", to_int(left));
        // assert_eq!(min_max, "456");
        // }
        
        // return res;
        // return Ok((line, {}));
    }

    pub fn call(scenario: &str) {
        let scenario_yaml = utils::read_yaml(scenario).unwrap();
        let scenarios = &scenario_yaml[0]["scenarios"].as_vec().unwrap();
        for scenario in scenarios.iter() {
            let input_vector: Vec<&str> = scenario["input"]
                .as_str()
                .unwrap()
                .split("\n")
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect();
            for iv in input_vector {
                parse(&iv);
            }
            // println!("{:?}", scenario);
        }
    }
}
