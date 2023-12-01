use std::fs;

const DIGITS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let input = fs::read_to_string("input/example2.txt").unwrap();
    let lines = input.lines();
    //let mut sum = 0;
    let mut sum2: i32 = 0;
    for line in lines {
        //sum += get_value(line);
        let mut number_string: String = get_first(line);
        number_string.push_str(get_last(line).as_str());
        let number: i32 = number_string.parse().unwrap();
        sum2 += number;
    }
    //print!("Answer to part 1 is {}.", sum);
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
    let first_digit = line.find(|c: char| c.is_numeric());
    let mut first: (usize, &str) = (usize::MAX, "");
    for digit in DIGITS {
        let search_result = line.find(digit);
        if search_result.is_some() && search_result.unwrap() < first.0 {
            first = (search_result.unwrap(), digit);
        }
    }
    if let Some(digit) = first_digit {
        if digit < first.0 {
            return first_digit.unwrap().to_string();
        }
    }
    get_digit_string(first.1)
}

fn get_last(line: &str) -> String {
    let last_digit = line.rfind(|c: char| c.is_numeric());

    let mut last: (usize, &str) = (0, "");
    for digit in DIGITS {
        let search_result = line.find(digit);
        if search_result.is_some() && search_result.unwrap() >= last.0 {
            last = (search_result.unwrap(), digit);
        }
    }
    if let Some(digit) = last_digit {
        if digit > last.0 {
            return digit.to_string();
        }
    }
    get_digit_string(last.1)
}

fn get_digit_string(input: &str) -> String {
    match input {
        "one" => String::from("1"),
        "two" => String::from("2"),
        "three" => String::from("3"),
        "four" => String::from("4"),
        "five" => String::from("5"),
        "six" => String::from("6"),
        "seven" => String::from("7"),
        "eight" => String::from("8"),
        "nine" => String::from("9"),
        _ => String::from("time to panic"),
    }
}
