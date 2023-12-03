use regex::Regex;
use std::{fs, ops::Add};

#[derive(Clone)]
struct Number {
    value: i32,   //value of the number
    start: usize, //index of first digit
    end: usize,   //index of last digit
}

fn main() {
    let input = fs::read_to_string("input/example.txt").unwrap();
    let lines = input.lines();
    let lines_vec: Vec<&str> = lines.collect();
    let number_of_lines = lines_vec.len();
    let line_length = lines_vec[0].len();
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

        //let symbols = String::from(line).match_indices(|c: char| !is_symbol(c));
    }
}

fn is_symbol(input: char) -> bool {
    !(input.is_numeric() || input == '.')
}
