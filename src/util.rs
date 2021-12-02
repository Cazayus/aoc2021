use std::fs;

pub fn read_lines_as_ints(filename: &str) -> Vec<i32> {
    fs::read_to_string(&filename)
        .expect("Unable to read file")
        .split("\r\n")
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}

pub fn read_lines_as_str(filename: &str) -> Vec<String> {
    fs::read_to_string(&filename)
        .expect("Unable to read file")
        .split("\r\n")
        .map(|line| line.parse::<String>().unwrap())
        .collect()
}
