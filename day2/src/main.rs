use std::fs;

struct Maximum {
    red: i32,
    green: i32,
    blue: i32,
}

fn main() {
    let maximum = Maximum {
        red: 11,
        green: 12,
        blue: 13,
    };

    let input = fs::read_to_string("input/example.txt").unwrap();
    let lines = input.lines();

    let sum = 0;

    for line in lines {
        let id_split = line.split(':');
        let id_strip = id_split.last().unwrap();
        let rounds = id_strip.split(';');
        for round in rounds {
            let color_split = round.split(',');
            for color_line in color_split {
                let mut split = color_line.split(' ');
                let amount: i32 = split.next().unwrap().parse().unwrap();
                let color = split.next().unwrap();
                match color {
                    _ => panic!("invalid color {color}"),
                }
            }
        }
    }
}
