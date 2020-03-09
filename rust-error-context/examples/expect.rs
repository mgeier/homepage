use std::fs;
use std::path::Path;

fn load_big_number<P: AsRef<Path>>(path: P, threshold: i32) -> i32 {
    let file_data = fs::read_to_string(path).expect("error reading file");
    let number = file_data.trim().parse().expect("error parsing data");
    if number < threshold {
        panic!("the number is too small!");
    }
    number
}

fn main() {
    let path = "myfile.txt";
    let number = load_big_number(path, 42);
    println!("the number is {}", number);
}
