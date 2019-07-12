use std::error;
use std::fmt;
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

fn box_error() -> Result<(), Box<dyn error::Error>> {
    //Err(Box::new(LoadError::TooSmall))
    Ok(())
}

fn main() -> Result<(), ShowErrorDisplay> {
    box_error()?;
    let path = "myfile.txt";
    let number = load_big_number(path, 42)?;
    println!("the number is {}", number);
    Ok(())
}

struct ShowErrorDisplay {
    inner: Box<dyn fmt::Display>,
}

impl fmt::Debug for ShowErrorDisplay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl<T: 'static + fmt::Display> From<T> for ShowErrorDisplay {
    fn from(error: T) -> ShowErrorDisplay {
        ShowErrorDisplay {
            inner: Box::new(error),
        }
    }
}

#[derive(Debug)]
enum LoadError {
    Read(io::Error),
    Parse(ParseIntError),
    TooSmall,
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use LoadError::*;
        match self {
            Read(e) => write!(f, "error reading file: {}", e),
            Parse(e) => write!(f, "error parsing file content: {}", e),
            TooSmall => write!(f, "the number is too small!"),
        }
    }
}

impl error::Error for LoadError {
    // TODO: source()
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
