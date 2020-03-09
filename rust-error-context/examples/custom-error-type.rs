use std::fs;
use std::io;
use std::num::ParseIntError;
use std::path::Path;

fn load_big_number<P: AsRef<Path>>(path: P, threshold: i32) -> Result<i32, LoadError> {
    let file_data = fs::read_to_string(path)?;
    let number = file_data.trim().parse()?;
    if number < threshold {
        Err(LoadError::TooSmall)
    } else {
        Ok(number)
    }
}

fn main() -> Result<(), anyhow::Error> {
    let path = "myfile.txt";
    let number = load_big_number(path, 42)?;
    println!("the number is {}", number);
    Ok(())
}

#[derive(thiserror::Error, Debug)]
enum LoadError {
    #[error("Error reading a file")]
    Read(#[from] io::Error),
    #[error("Error parsing a file")]
    Parse(#[from] ParseIntError),
    #[error("The value is too small")]
    TooSmall,
}
