use regex::Regex;
use std::fs;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Number {
    value: i32,   //value of the number
    start: usize, //index of first digit
    end: usize,   //index of last digit
}

fn main() {
    let input = fs::read_to_string("input/input.txt").unwrap();
    let lines = input.lines();
    let lines_vec: Vec<&str> = lines.collect();
    let number_of_lines = lines_vec.len();
    let mut numbers: Vec<Vec<Number>> = vec![vec![]; number_of_lines];
    let number_regex = Regex::new(r"[0-9]+").unwrap();

    for i in 0..number_of_lines {
        let line = lines_vec[i];
        let found_numbers = Regex::find_iter(&number_regex, line);

        for number in found_numbers {
            let value: i32 = number.as_str().parse().unwrap();
            let start = line.find(number.as_str()).unwrap();
            let end = start + number.len();
            numbers[i].push(Number { value, start, end })
        }
    }

    let mut symbols: Vec<Vec<usize>> = vec![vec![]; number_of_lines];

    for i in 0..number_of_lines {
        let line = lines_vec[i];
        let found_symbols = line.match_indices(is_symbol);
        for symbol in found_symbols {
            symbols[i].push(symbol.0);
        }
    }
    let mut sum = 0;
    for i in 0..numbers.len() {
        for number in &numbers[i] {
            let minimum = if number.start > 0 {
                number.start - 1
            } else {
                number.start
            };
            let maximum = if number.end >= lines_vec[0].len() - 1 {
                number.end
            } else {
                number.end + 1
            };
            if i >= 1 && adjacent_symbol(&symbols[i - 1], minimum, maximum) {
                sum += number.value;
                continue;
            }
            if adjacent_symbol(&symbols[i], minimum, maximum) {
                sum += number.value;
                continue;
            }
            if i < lines_vec[0].len() - 1 && adjacent_symbol(&symbols[i + 1], minimum, maximum) {
                sum += number.value;
                continue;
            }
        }
    }
    println!("Answer to part 1 is {sum}.")
}

fn is_symbol(input: char) -> bool {
    !(input.is_numeric() || input == '.')
}

fn adjacent_symbol(input_vector: &[usize], minimum: usize, maximum: usize) -> bool {
    for i in minimum..=maximum {
        if input_vector.contains(&i) {
            return true;
        }
    }
    false
}
