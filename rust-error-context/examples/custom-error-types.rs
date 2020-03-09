use std::num::ParseIntError;
use std::path::Path;

fn load_big_number<P: AsRef<Path>>(path: P) -> Result<i32, LoadError> {
    let file_data = std::fs::read_to_string(path)?;
    let number = parse_big_number(&file_data)?;
    Ok(number)
}

fn parse_big_number(data: &str) -> Result<i32, ParseError> {
    let number = data.trim().parse()?;
    if number < 42 {
        Err(ParseError::TooSmall)
    } else {
        Ok(number)
    }
}

fn main() -> Result<(), anyhow::Error> {
    let path = "myfile.txt";
    let number = load_big_number(path)?;
    println!("the number is {}", number);
    Ok(())
}

#[derive(thiserror::Error, Debug)]
enum LoadError {
    #[error("Error reading a file")]
    Read(#[from] std::io::Error),
    #[error("Error parsing file content")]
    Parse(#[from] ParseError),
}

#[derive(thiserror::Error, Debug)]
enum ParseError {
    #[error("Invalid data")]
    Invalid(#[from] ParseIntError),
    #[error("The value is too small")]
    TooSmall,
}
