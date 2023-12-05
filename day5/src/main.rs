#[derive(Debug)]
struct Range {
    start: i32,
    end: i32,
    target: i32,
}

impl Range {
    fn contains(&self, input: i32) -> bool {
        input >= self.start && input <= self.end
    }

    // assumes input is in this range
    fn convert(&self, input: i32) -> i32 {
        assert!(
            self.contains(input),
            "Input {input} is not in range {:?}",
            self
        );
        input - self.start + self.target
    }
}

struct Conversion {
    ranges: Vec<Range>,
}

impl Conversion {
    fn convert(&self, input: i32) -> i32 {
        for range in &self.ranges {
            if range.contains(input) {
                return range.convert(input);
            }
        }
        input
    }
}

fn main() {}
