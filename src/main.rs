use std::io::{Error};
mod lib;
use lib::{day01};

fn main() -> Result<(), Error> {
    day01::call("./inputs/01.yaml");

    Ok(())
}
