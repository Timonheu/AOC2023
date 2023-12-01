use std::fs;

fn main() {
    let input = fs::read_to_string("input/input.txt").unwrap();
    let lines = input.lines();
    let mut sum = 0;
    let mut sum2: i32 = 0;
    for line in lines {
        sum += get_value(line);
        let mut number_string: String = get_first(line);
        number_string.push_str(get_last(line).as_str());
        let number: i32 = number_string.parse().unwrap();
        sum2 += number;
    }
    print!("Answer to part 1 is {}.", sum);
    print!("Answer to part 2 is {}.", sum2);
}

fn get_value(line: &str) -> i32 {
    let mut numbers = line.to_string();
    numbers.retain(|c| c.is_numeric());
    let mut first = numbers.chars().next().unwrap().to_string();
    let last = numbers.chars().last().unwrap().to_string();
    first.push_str(&last);
    first.parse().unwrap()
}

fn get_first(line: &str) -> String {
    if line.chars().next().unwrap().is_numeric() {
        line.chars().next().unwrap().to_string()
    } else {
        //TODO: find first written out number
        String::from("1")
    }
}

fn get_last(line: &str) -> String {
    if line.chars().last().unwrap().is_numeric() {
        line.chars().next().unwrap().to_string()
    } else {
        //TODO: find last written out number
        String::from("1")
    }
}
