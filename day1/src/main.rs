use std::fs;

fn main() {
    let input = fs::read_to_string("input/example.txt").unwrap();
    let lines = input.lines();
    let mut sum = 0;
    for line in lines {
        sum += get_value(line);
    }
}

fn get_value(line: &str) -> i32 {
    0
}
