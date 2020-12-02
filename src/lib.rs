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

pub mod parser {

}

pub mod day02 {
    use super::utils;
    use nom::bytes::complete::tag;
    use nom::character::complete::{alpha1, char, multispace0, one_of};
    use nom::combinator::map_res;
    use nom::error::ParseError;
    use nom::multi::{many0, many1};
    use nom::sequence::{delimited, terminated};

    use nom::{combinator::recognize, sequence::separated_pair, IResult};

    #[derive(Debug)]
    struct Problem {
        min: u32,
        max: u32,
        password_char: String,
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
        let (input, (max, min)) = ws(password_criteria)(input)?;
        let (input, password_char) = ws(password_character)(input)?;
        let (_, password) = ws(password_contents)(input)?;

        println!("input: {:?}", input);
        println!("max: {:?}", max);
        println!("min: {:?}", min);
        println!("pw character: {:?}", password_char);
        println!("password: {:?}", password);
        
        Ok((input, Problem { max, min, password: String::from(password), password_char: String::from(password_char) }))
    }

    fn parse(input: &str) {
        println!("my parser output: #{:?}", parser(input))
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
            break;
            // println!("{:?}", scenario);
        }
    }
}
