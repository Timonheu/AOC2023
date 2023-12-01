use std::fs;

fn main() {
    let input = fs::read_to_string("input/example.txt").unwrap();
    let lines = input.lines();
    let mut sum = 0;
    for line in lines {
        sum += get_value(line);
    }
    print!("Answer to part 1 is {}.", sum);
}

fn get_value(line: &str) -> i32 {
    let mut numbers = line.to_string();
    numbers.retain(|c| c.is_numeric());
    let first: i32 = numbers.chars().next().unwrap().to_digit(10).unwrap() as i32;
    let last: i32 = numbers.chars().last().unwrap().to_digit(10).unwrap() as i32;
    first + last
}
