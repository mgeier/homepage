use std::path::Path;

fn load_big_number<P: AsRef<Path>>(path: P) -> i32 {
    let file_data = std::fs::read_to_string(path).unwrap();
    parse_big_number(&file_data)
}

fn parse_big_number(data: &str) -> i32 {
    let number = data.trim().parse().unwrap();
    if number < 42 {
        panic!("the number is too small!");
    }
    number
}

fn main() {
    let path = "myfile.txt";
    let number = load_big_number(path);
    println!("the number is {}", number);
}
