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
    Err(Box::new(LoadError::TooSmall))
}

fn main() -> Result<(), Box<dyn DisplayError>> {
    box_error()?;
    let path = "myfile.txt";
    let number = load_big_number(path, 42)?;
    println!("the number is {}", number);
    Ok(())
}

trait DisplayError: fmt::Display {
}

impl fmt::Debug for DisplayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}

//impl<T: fmt::Display> DisplayError for T {}
impl<T: error::Error> DisplayError for T {}

/*
impl<T: error::Error> From<Box<T>> for Box<dyn DisplayError> {
    fn from(error: Box<T>) -> Box<dyn DisplayError> {
        error
    }
}
*/

/*
impl<T: error::Error + 'static> From<T> for Box<dyn DisplayError> {
    fn from(error: T) -> Box<dyn DisplayError> {
        // TODO: what if T is already a Box?
        Box::new(error)
    }
}
*/

/*
impl From<Box<dyn error::Error>> for Box<dyn DisplayError> {
    fn from(error: Box<dyn error::Error>) -> Box<dyn DisplayError> {
        error
    }
}
*/

impl From<LoadError> for Box<dyn DisplayError> {
    fn from(error: LoadError) -> Box<dyn DisplayError> {
        Box::new(error)
    }
}

/*
impl<T: DisplayError> From<T> for Box<dyn DisplayError> {
    fn from(error: T) -> Box<dyn DisplayError> {
        Box::new(error)
    }
}
*/

/*
impl<T: error::Error + 'static> From<Box<T>> for Box<dyn DisplayError> {
    fn from(error: Box<T>) -> Box<dyn DisplayError> {
        error
    }
}
*/

/*
impl<T: 'static + std::ops::Deref<Target = error::Error>> From<T> for Box<dyn DisplayError> {
    fn from(error: T) -> Box<dyn DisplayError> {
        Box::new(error.deref())
    }
}
*/

/*
impl<T: 'static + AsRef<error::Error>> From<T> for Box<dyn DisplayError> {
    fn from(error: T) -> Box<dyn DisplayError> {
        Box::new(error.as_ref())
    }
}
*/

#[derive(Debug)]
enum LoadError {
    Read(io::Error),
    Parse(ParseIntError),
    TooSmall,
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "load error: TODO")
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
