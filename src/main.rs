extern crate nom;
use std::io::{Error};
mod lib;
use lib::{day01,day02,day03};

fn main() -> Result<(), Error> {
    day01::call("./inputs/01.yaml");
    day02::call("./inputs/02.yaml");
    day03::call("./inputs/03.yaml");
    Ok(())
}
