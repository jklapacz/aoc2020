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

pub mod parser {}

pub mod day02 {
    use super::utils;
    use nom::bytes::complete::tag;
    use nom::character::complete::{alpha1, char, multispace0, one_of};
    use nom::combinator::map_res;
    use nom::error::ParseError;
    use nom::multi::{many0, many1};
    use nom::sequence::{delimited, terminated};
    use nom::Finish;
    use std::convert::TryFrom;

    use nom::{combinator::recognize, sequence::separated_pair, IResult};

    #[derive(Debug)]
    struct Problem {
        min: u32,
        max: u32,
        password_char: char,
        password: String,
    }

    fn decimal_helper(input: &str) -> IResult<&str, &str> {
        recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
    }

    fn decimal(input: &str) -> IResult<&str, u32> {
        map_res(decimal_helper, |s: &str| s.parse())(input)
    }

    fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
        inner: F,
    ) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
    where
        F: Fn(&'a str) -> IResult<&'a str, O, E>,
    {
        delimited(multispace0, inner, multispace0)
    }

    fn password_criteria(input: &str) -> IResult<&str, (u32, u32)> {
        separated_pair(decimal, tag("-"), decimal)(input)
    }

    fn password_character(input: &str) -> IResult<&str, &str> {
        terminated(alpha1, tag(":"))(input)
    }

    fn password_contents(input: &str) -> IResult<&str, &str> {
        alpha1(input)
    }

    fn parser(input: &str) -> IResult<&str, Problem> {
        let (input, (min, max)) = ws(password_criteria)(input)?;
        let (input, password_char) = ws(password_character)(input)?;
        let (_, password) = ws(password_contents)(input)?;

        let pwd_char = String::from(password_char)
            .chars()
            .next()
            .expect("should have found a password constraint...");

        // println!("input: {:?}", input);
        // println!("max: {:?}", max);
        // println!("min: {:?}", min);
        // println!("pw character: {:?}", password_char);
        // println!("password: {:?}", password);

        Ok((
            input,
            Problem {
                max,
                min,
                password: String::from(password),
                password_char: pwd_char,
            },
        ))
    }

    fn is_valid(pwd: &Problem) -> bool {
        let constraint = pwd.password_char;
        let mut chars = pwd.password.chars();
        let total = chars.by_ref().filter(|&c| c == constraint).count() as u32;
        let valid = total >= pwd.min && total <= pwd.max;
        // println!("total {:?}", total);
        // println!("min {:?}", pwd.min);
        // println!("max {:?}", pwd.max);
        // println!("valid {:?}", valid);
        return valid;
    }

    fn is_valid_part_2(p: &Problem) -> bool {
        let mut password_chars = p.password.chars();
        let first_idx = p.min - 1;
        let last_idx = p.max - 1;
        println!("{:?}", password_chars);
        if password_chars.find(|&c| c == p.password_char).is_none() {
            return false;
        }

        let first_char = p.password.clone().chars()
            .nth(usize::try_from(first_idx).unwrap())
            .unwrap();
        let last_char = p.password.clone().chars()
            .nth(usize::try_from(last_idx).unwrap())
            .unwrap();
        println!("{:?} {:?}", first_idx, first_char);
        println!("{:?}", last_char);
        let valid = (p.password_char == first_char || p.password_char == last_char) && (first_char != last_char);
        println!("{:?}", valid);
        valid
    }

    fn solve(passwords: Vec<&str>) -> (u32, u32) {
        let mut total_valid_1 = 0;
        let mut total_valid_2 = 0;

        for p in passwords {
            if let Ok(parsed) = parser(p).finish() {
                let (_, p) = parsed;

                if is_valid(&p) {
                    total_valid_1 = total_valid_1 + 1;
                }
                if is_valid_part_2(&p) {
                    total_valid_2 = total_valid_2 + 1;
                }
            }
        }

        (total_valid_1, total_valid_2)
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
            let part_1 = scenario["outputs"][0].as_i64().unwrap();
            let part_2 = scenario["outputs"][1].as_i64().unwrap();
            let (sol_1, sol_2) = solve(input_vector);
            assert_eq!(i64::from(sol_1), part_1);
            assert_eq!(i64::from(sol_2), part_2);
        }
    }
}
