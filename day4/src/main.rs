use num_traits::pow;
use std::fs;

fn main() {
    let input = fs::read_to_string("input/input.txt").unwrap();
    let lines = input.lines();
    let mut sum = 0;

    let mut copies = vec![1; lines.clone().count()];

    for (i, line) in lines.enumerate() {
        let number_lines = line.split(':').last().unwrap().trim();
        let number_lines_split = &mut number_lines.split('|');
        let winning_numbers: Vec<i32> = number_lines_split
            .next()
            .unwrap()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|n| n.parse().unwrap())
            .collect();
        let own_numbers: Vec<i32> = number_lines_split
            .next()
            .unwrap()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|n| n.parse().unwrap())
            .collect();

        let mut match_count = 0;
        for winning_number in winning_numbers {
            if own_numbers.contains(&winning_number) {
                match_count += 1;
            }
        }
        if match_count > 0 {
            sum += pow(2, match_count - 1);
        }

        //part 2
        let current_copies = copies[i];
        for j in 1..=match_count {
            if i + j >= copies.len() {
                break;
            }
            copies[i + j] += current_copies;
        }
    }
    println!("Answer to part 1 is {sum}.");
    let mut sum2 = 0;
    for amount in copies {
        sum2 += amount;
    }
    println!("Answer to part 2 is {sum2}.");
}
