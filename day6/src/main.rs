use std::fs;

fn main() {
    let input = fs::read_to_string("input/example.txt").unwrap();
    let mut lines = input.lines();
    let times: Vec<i32> = lines
        .next()
        .unwrap()
        .split(' ')
        .filter(|s| !s.eq_ignore_ascii_case("Time:"))
        .map(|x| x.parse().unwrap())
        .collect();
    let distances: Vec<i32> = lines
        .next()
        .unwrap()
        .split(' ')
        .filter(|s| !s.eq_ignore_ascii_case("Distance:"))
        .map(|x| x.parse().unwrap())
        .collect();
}
