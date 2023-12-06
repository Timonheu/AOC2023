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
    for range in &conversions[0].ranges {
        result.append(collapse_range(
            range.start,
            range.end,
            range.start,
            range.end,
            &conversions,
            result.clone().as_mut(),
        ));
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

fn collapse_range<'a>(
    start: i64,
    end: i64,
    current_start: i64,
    current_end: i64,
    conversions: &'a Vec<Conversion>,
    result: &'a mut Vec<Range>,
) -> &'a mut Vec<Range> {
    let max = end - start;

    let conversion = &conversions[0];
    let converted_start = conversion.convert(current_start);
    let converted_end = conversion.convert(current_end);
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
                if conversions.len() == 1 {
                    // base case
                    result.push(Range {
                        start: start + j,
                        target: conversions[0].convert(start + j),
                        end,
                    });
                } else {
                    // recurse
                    result.append(collapse_range(
                        start + j,
                        end,
                        current_value,
                        converted_end,
                        &conversions[1..].to_vec(),
                        result.clone().as_mut(),
                    ));
                }
                break;
            } else {
                let this_range_end = start + (target_range.end - current_value);
                if conversions.len() == 1 {
                    result.push(Range {
                        start: start + j,
                        end: this_range_end,
                        target: conversions[0].convert(start + j),
                    });
                } else {
                    result.append(collapse_range(
                        start + j,
                        this_range_end,
                        current_value,
                        target_range.end,
                        &conversions[1..].to_vec(),
                        result.clone().as_mut(),
                    ));
                }
                j += target_range.end - current_value + 1;
                //print!("j: {j}");
            }
        } else {
            j += 1;
        }
    }
    result
}
