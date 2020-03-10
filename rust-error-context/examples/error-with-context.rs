use std::num::ParseIntError;
use std::path::{Path, PathBuf};

use rust_error_context::FromSourceAndContext;
use rust_error_context::ResultExt;

fn load_big_number<P: AsRef<Path>>(path: P) -> Result<i32, LoadError> {
    let path = path.as_ref();
    let file_data = std::fs::read_to_string(path).err_context(path)?;
    let number = parse_big_number(&file_data).err_context(path)?;
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
    #[error("Error reading {context:?}")]
    Read {
        source: std::io::Error,
        context: PathBuf,
    },
    #[error("Error parsing content of {context:?}")]
    Parse {
        source: ParseError,
        context: PathBuf,
    },
}

#[derive(thiserror::Error, Debug)]
enum ParseError {
    #[error("Invalid data")]
    Invalid(#[from] ParseIntError),
    #[error("The value is too small")]
    TooSmall,
}

impl FromSourceAndContext<std::io::Error, PathBuf> for LoadError {
    fn from_source_and_context(source: std::io::Error, context: PathBuf) -> Self {
        LoadError::Read {
            source,
            context,
        }
    }
}

impl FromSourceAndContext<ParseError, PathBuf> for LoadError {
    fn from_source_and_context(source: ParseError, context: PathBuf) -> Self {
        LoadError::Parse {
            source,
            context,
        }
    }
}
