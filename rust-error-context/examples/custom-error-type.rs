use std::fs;
use std::io;
use std::num::ParseIntError;
use std::path::Path;

fn load_big_number<P: AsRef<Path>>(path: P, threshold: isize) -> Result<isize, LoadError> {
    let file_data = fs::read_to_string(path)?;
    let number = file_data.trim().parse::<isize>()?;
    if number < threshold {
        Err(LoadError::TooSmall)
    } else {
        Ok(number)
    }
}

fn main() -> Result<(), LoadError> {
    let path = "myfile.txt";
    let number = load_big_number(path, 42)?;
    println!("the number is {}", number);
    Ok(())
}

#[derive(Debug)]
enum LoadError {
    Read(io::Error),
    Parse(ParseIntError),
    TooSmall,
}

impl From<io::Error> for LoadError {
    fn from(error: io::Error) -> LoadError {
        LoadError::Read(error)
    }
}

impl From<ParseIntError> for LoadError {
    fn from(error: ParseIntError) -> LoadError {
        LoadError::Parse(error)
    }
}
