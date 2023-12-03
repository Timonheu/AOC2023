use regex::Regex;
use std::fs;

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
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

    // Find all numbers and put them in a two dimensional vector
    for i in 0..number_of_lines {
        let line = lines_vec[i];
        let found_numbers: Vec<&str> = Regex::find_iter(&number_regex, line)
            .map(|m| m.as_str())
            .collect();

        let mut remaining_line = String::from(line);
        let mut current_index = 0;
        for number_slice in found_numbers {
            let value: i32 = number_slice.parse().unwrap();
            let start = remaining_line.find(number_slice).unwrap() + current_index;
            let end = start + number_slice.len() - 1;
            numbers[i].push(Number { value, start, end });
            remaining_line = remaining_line
                .get_mut(end + 1 - current_index..)
                .unwrap()
                .to_string();
            current_index = end + 1;
        }
    }
    assert!(overlap_check(numbers.clone()));

    let mut symbols: Vec<Vec<usize>> = vec![vec![]; number_of_lines];
    let mut cog_sum = 0;
    // find all symbols and put them in a two dimensional vector
    for i in 0..number_of_lines {
        let line = lines_vec[i];
        let found_symbols: Vec<usize> = line.match_indices(is_symbol).map(|s| s.0).collect();
        for symbol in found_symbols {
            symbols[i].push(symbol);
        }

        let found_cogs: Vec<usize> = line.match_indices('*').map(|c| c.0).collect();
        // not the prettiest, but for each cog collect all unique adjacent cogs, if there are
        // exactly two add their gear ratio to the sum
        for cog in found_cogs {
            let mut adjacent_numbers: Vec<Number> = vec![];
            let minimum = if cog > 0 { cog - 1 } else { cog };
            let maximum = if cog >= lines_vec[0].len() - 1 {
                cog
            } else {
                cog + 1
            };
            if i > 0 {
                for number in &numbers[i - 1] {
                    if overlap(minimum, maximum, number.start, number.end) {
                        adjacent_numbers.push(number.clone());
                    }
                }
            }
            for number in &numbers[i] {
                if overlap(minimum, maximum, number.start, number.end) {
                    adjacent_numbers.push(number.clone());
                }
            }
            if i < number_of_lines - 1 {
                for number in &numbers[i + 1] {
                    if overlap(minimum, maximum, number.start, number.end) {
                        adjacent_numbers.push(number.clone());
                    }
                }
            }
            if adjacent_numbers.len() == 2 {
                cog_sum += adjacent_numbers[0].value * adjacent_numbers[1].value;
            }
        }
    }

    // Check for each number if it has an ajacent symbol, if yes add the number to the sum.
    let mut sum = 0;
    for i in 0..number_of_lines {
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
    println!("Answer to part 1 is {sum}.");
    println!("Answer to part 2 is {cog_sum}.");
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

fn overlap_check(numbers: Vec<Vec<Number>>) -> bool {
    for set in numbers {
        for number in set.clone() {
            for number2 in &set {
                if number != *number2
                    && overlap(number.start, number.end, number2.start, number2.end)
                {
                    println!("Overlap! number1: {:#?}", number);
                    println!("number2: {:#?}", number2);
                    return false;
                }
            }
        }
    }
    true
}

fn overlap(start1: usize, end1: usize, start2: usize, end2: usize) -> bool {
    // 1 starts or ends in 2
    ((start1 >= start2 && start1 <= end2) || (end1 >= start2 && end1 <= end2))
    // 2 starts or ends in 1
    || ((start2 >= start1 && start2 <= end1) || (end2 >= start1 && end2 <= end1))
    // 1 contains 2
    || (start1 <= start2 && end1 >= end2)
    // 2 contains 1
    || (start2 <= start1 && end2 >= end1)
}
