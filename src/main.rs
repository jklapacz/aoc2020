extern crate nom;
extern crate env_logger;
use std::io::{Error};
mod lib;
use lib::{day01,day02,day03, day04};

fn main() -> Result<(), Error> {
    env_logger::init();
    day01::call("./inputs/01.yaml");
    day02::call("./inputs/02.yaml");
    day03::call("./inputs/03.yaml");
    day04::call("ecl:gry pid:860033327 eyr:2020 hcl:fffffd
    byr:1937 iyr:2017 cid:147 hgt:183cm");
    day04::call("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
    hcl:cfa07d byr:1929");
    day04::call("hcl:ae17e1 iyr:2013 eyr:2024 ecl:brn pid:760753108 byr:1931 hgt:179cm");
    day04::call("hcl:#cfa07d eyr:2025 pid:166559648 iyr:2011 ecl:brn hgt:59in");
    Ok(())
}
