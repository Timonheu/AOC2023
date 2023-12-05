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
                return Some(Range {
                    start: range.start,
                    end: range.end,
                    target: range.target,
                });
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

    //find the optimal range from the ground up
    let minimal_range = conversions.last().unwrap().ranges[0].clone();
    let minimal_start_end = (minimal_range.start, minimal_range.end);

    let mut reverse_conversions = conversions.clone();
    reverse_conversions.reverse();

    for conversion in reverse_conversions {
        for range in conversion.ranges {}
    }

    println!("Answer to part 2: {minimum_location_number}.");
}
