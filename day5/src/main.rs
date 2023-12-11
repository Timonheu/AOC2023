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
    let input = fs::read_to_string("input/input.txt").unwrap();
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
            let end = start + range - 1;
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

    let mut collapsed_ranges: Vec<Range> = vec![];

    let ranges = conversions[0].clone().ranges;

    for range in ranges {
        collapsed_ranges.append(&mut collapse_range(
            range.start,
            range.start,
            range.end - range.start,
            conversions.clone(),
        ));
    }

    println!("{:?}", collapsed_ranges);

    let single_conversion = Conversion {
        ranges: collapsed_ranges,
    };

    minimum_location_number = i64::MAX;
    for seed in seeds {
        let value = single_conversion.convert(seed);
        if value < minimum_location_number {
            minimum_location_number = value;
        }
    }

    println!("Answer to part 1: {minimum_location_number}.")
}

fn collapse_range(
    original_start: i64,
    converted_start: i64,
    range_size: i64,
    conversions: Vec<Conversion>,
) -> Vec<Range> {
    let mut result = vec![];
    let mut input = converted_start;
    while input <= converted_start + range_size {
        let size_left = converted_start + range_size - input;
        let steps_taken = range_size - size_left;
        let target = conversions[0].convert(input);
        if target == input {
            input += 1;
        } else {
            let target_range = conversions[0].get_range(input).unwrap();
            // println!(
            //     "target: {target}, input: {input}, target_range: {:?}",
            //     target_range
            // );
            if target_range.end > target + size_left {
                //the entirety of the remaining range fits in the target range
                if conversions.len() == 1 {
                    // base case
                    result.push(Range {
                        start: original_start + steps_taken,
                        end: original_start + range_size,
                        target,
                    });
                } else {
                    result.append(
                        collapse_range(
                            original_start + steps_taken,
                            target,
                            size_left,
                            conversions[1..].to_vec(),
                        )
                        .as_mut(),
                    );
                }
                break;
            } else {
                // println!("target_range.end: {}, target: {target}", target_range.end);
                let target_size_left = target_range.end - input;
                //the remaining range does not entirely fit in the target range
                if conversions.len() == 1 {
                    // base case
                    result.push(Range {
                        start: original_start + steps_taken,
                        end: original_start + steps_taken + target_size_left,
                        target,
                    });
                } else {
                    result.append(
                        collapse_range(
                            original_start + steps_taken,
                            target,
                            target_size_left,
                            conversions[1..].to_vec(),
                        )
                        .as_mut(),
                    );
                }
                // println!(
                //     "input: {input}, loop end: {}, target_size_left: {target_size_left}",
                //     converted_start + range_size
                // );
                input += target_size_left + 1;
            }
        }
    }
    result
}
