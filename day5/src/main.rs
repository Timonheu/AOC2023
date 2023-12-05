use std::fs;

#[derive(Debug, Clone)]
struct Range {
    start: i64,
    end: i64,
    target: i64,
}

impl Range {
    fn contains(&self, input: i64) -> bool {
        input >= self.start && input <= self.end
    }

    // assumes input is in this range
    fn convert(&self, input: i64) -> i64 {
        assert!(
            self.contains(input),
            "Input {input} is not in range {:?}",
            self
        );
        input - self.start + self.target
    }
}

#[derive(Clone)]
struct Conversion {
    ranges: Vec<Range>,
}

impl Conversion {
    fn convert(&self, input: i64) -> i64 {
        for range in &self.ranges {
            if range.contains(input) {
                return range.convert(input);
            }
        }
        input
    }

    fn get_range(&self, input: i64) -> Option<Range> {
        for range in &self.ranges {
            if range.contains(input) {
                return Some(range.clone());
            }
        }
        None
    }
}

fn main() {
    let input = fs::read_to_string("input/example.txt").unwrap();
    let mut lines = input.lines();

    let seeds: Vec<i64> = lines
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();

    // Some ugly ad-hoc parsing
    assert!(lines.next().unwrap().is_empty());
    let mut conversions: Vec<Conversion> = vec![];
    for _i in 0..7 {
        // conversion title
        lines.next();
        let mut range_line = lines.next().unwrap();
        let mut conversion = Conversion { ranges: vec![] };
        while !range_line.is_empty() {
            let mut split = range_line.split(' ');
            let target: i64 = split.next().unwrap().parse().unwrap();
            let start: i64 = split.next().unwrap().parse().unwrap();
            let range: i64 = split.next().unwrap().parse().unwrap();
            let end = start + range;
            conversion.ranges.push(Range { start, end, target });
            let next = lines.next();
            if next.is_some() {
                range_line = next.unwrap();
            } else {
                break;
            }
        }
        conversions.push(conversion);
    }

    let mut minimum_location_number: i64 = i64::MAX;

    for seed in seeds.clone() {
        let mut value = seed;
        for conversion in &conversions {
            value = conversion.convert(value);
        }
        if value < minimum_location_number {
            minimum_location_number = value;
        }
    }
    println!("Answer to part 1: {minimum_location_number}.");

    // Time to optimize: collapse all conversions into one
    let mut result: Vec<Range> = vec![];
    let mut i: usize = 0;
    while i < conversions.len() - 1 {
        let conversion = conversions[i].clone();
        for range in conversion.ranges.clone() {
            let max = range.end - range.start;
            let converted_start = conversion.convert(range.start);
            let converted_end = conversion.convert(range.end);
            //println!("{max}");
            let mut j = 0;
            while j <= max {
                let current_input = converted_start + j;
                let current_value = conversion.convert(current_input);

                //println!("Current input: {current_input}, Current value: {current_value}");
                if current_value != current_input {
                    let target_range = conversion.get_range(current_value).unwrap();
                    //println!("target range: {:?}", target_range);
                    //if the target range can fit the rest of this remaining range
                    if target_range.end - current_value >= converted_end - current_input {
                        result.push(Range {
                            start: current_value,
                            end: conversion.convert(converted_end),
                            target: conversions[i + 1].convert(current_value),
                        });
                        break;
                    } else {
                        // target range can not fit the entirety of this remaining range
                        result.push(Range {
                            start: current_value,
                            end: target_range.end,
                            target: conversions[i + 1].convert(current_value),
                        });
                        //assert!(                      current_value < target_range.end,                            "Target range end: {}, Current value: {current_value}",                            target_range.end                        );
                        j += target_range.end - current_value + 1;
                        //print!("j: {j}");
                    }
                } else {
                    j += 1;
                }
            }
        }
        i += 1;
    }

    let single_conversion = Conversion { ranges: result };

    let mut minimum_location_number: i64 = i64::MAX;

    for seed in seeds.clone() {
        let mut value = seed;
        value = single_conversion.convert(value);
        if value < minimum_location_number {
            minimum_location_number = value;
        }
    }

    println!("Answer to part 1: {minimum_location_number}.");
}
