use std::path::Path;

use anyhow::Context;

fn load_big_number<P: AsRef<Path>>(path: P) -> Result<i32, anyhow::Error> {
    let file_data = std::fs::read_to_string(path).context("error reading file")?;
    parse_big_number(&file_data)
}

fn parse_big_number(data: &str) -> Result<i32, anyhow::Error> {
    let number = data.trim().parse().context("error parsing number")?;
    if number < 42 {
        anyhow::bail!("the number is too small!");
    }
    Ok(number)
}

fn main() -> Result<(), anyhow::Error> {
    let path = "myfile.txt";
    let number = load_big_number(path)?;
    println!("the number is {}", number);
    Ok(())
}
