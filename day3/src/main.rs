use regex::Regex;
use std::collections::HashSet;
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
    let mut numbers: Vec<HashSet<Number>> = vec![HashSet::new(); number_of_lines]; // using a hash
                                                                                   // set to avoid duplicates
    let number_regex = Regex::new(r"[0-9]+").unwrap();

    // Find all numbers and put them in a two dimensional vector
    for i in 0..number_of_lines {
        let line = lines_vec[i];
        let found_numbers: Vec<&str> = Regex::find_iter(&number_regex, line)
            .map(|m| m.as_str())
            .collect();

        let mut remaining_line = String::from(line);
        for number_slice in found_numbers {
            let value: i32 = number_slice.parse().unwrap();
            let start = remaining_line.find(number_slice).unwrap();
            let end = start + number_slice.len() - 1;
            numbers[i].insert(Number { value, start, end });
            remaining_line = remaining_line.get_mut(end + 1..).unwrap().to_string();
        }
    }
    //assert!(overlap_check(numbers.clone()));

    let mut symbols: Vec<Vec<usize>> = vec![vec![]; number_of_lines];

    // find all symbols and put them in a two dimensional vector
    for i in 0..number_of_lines {
        let line = lines_vec[i];
        let found_symbols = line.match_indices(is_symbol);
        for symbol in found_symbols {
            symbols[i].push(symbol.0);
        }
    }

    // Check for each number if it has an ajacent symbol, if yes add the number to the sum.
    let mut sum = 0;
    for i in 0..numbers.len() {
        '_symbol_check: for number in &numbers[i] {
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
            //check line above
            if i > 0 && adjacent_symbol(&symbols[i - 1], minimum, maximum) {
                sum += number.value;
                continue '_symbol_check;
            }
            //check same line: check for a symbol to the left or to the right
            if symbols[i].contains(&minimum) || symbols[i].contains(&maximum) {
                sum += number.value;
                continue '_symbol_check;
            }
            //check line below
            if i < lines_vec[0].len() - 1 && adjacent_symbol(&symbols[i + 1], minimum, maximum) {
                sum += number.value;
                continue '_symbol_check;
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

fn overlap_check(numbers: Vec<HashSet<Number>>) -> bool {
    for set in numbers {
        for number in set.clone() {
            for number2 in &set {
                if number != *number2
                    && overlap(number.start, number.end, number2.start, number2.end)
                {
                    return false;
                }
            }
        }
    }
    true
}

fn overlap(start1: usize, end1: usize, start2: usize, end2: usize) -> bool {
    (start1 >= start2 && start1 <= end2) || (end1 >= start2 && end1 <= end2)
}
