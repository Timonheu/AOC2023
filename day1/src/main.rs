use std::fs;

fn main() {
    let input = fs::read_to_string("input/input.txt").unwrap();
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
    let mut first = numbers.chars().next().unwrap().to_string();
    let last = numbers.chars().last().unwrap().to_string();
    first.push_str(&last);
    first.parse().unwrap()
}
