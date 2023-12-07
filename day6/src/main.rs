use std::fs;

struct Race {
    time: i64,
    distance: i64,
}

impl Race {
    fn get_winning_possibilities(&self) -> i64 {
        let mut result = 0;
        for i in 0..=self.time {
            if i * (self.time - i) > self.distance {
                result += 1;
            }
        }
        result
    }
}

fn main() {
    let input = fs::read_to_string("input/input.txt").unwrap();
    let mut lines = input.lines();
    let times: Vec<i64> = lines
        .next()
        .unwrap()
        .split(' ')
        .filter(|s| !s.eq_ignore_ascii_case("Time:") && !s.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();
    let distances: Vec<i64> = lines
        .next()
        .unwrap()
        .split(' ')
        .filter(|s| !s.eq_ignore_ascii_case("Distance:") && !s.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();

    let mut races: Vec<Race> = vec![];
    for (i, time) in times.clone().into_iter().enumerate() {
        races.push(Race {
            time,
            distance: distances[i],
        });
    }

    let mut product = 1;
    for race in races {
        product *= race.get_winning_possibilities();
    }
    println!("Answer to part 1: {product}.");

    let single_time_string = String::from_iter(times.iter().map(|t| t.to_string()));
    let single_time: i64 = single_time_string.parse().unwrap();

    let single_distance_string = String::from_iter(distances.iter().map(|d| d.to_string()));
    let single_distance: i64 = single_distance_string.parse().unwrap();

    let answer = Race {
        time: single_time,
        distance: single_distance,
    }
    .get_winning_possibilities();

    println!("Answer to part 2: {answer}.");
}
