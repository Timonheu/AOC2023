use std::fs;

struct Maximum {
    red: i32,
    green: i32,
    blue: i32,
}

fn main() {
    let maximum = Maximum {
        red: 12,
        green: 13,
        blue: 14,
    };

    let input = fs::read_to_string("input/input.txt").unwrap();
    let lines = input.lines();

    let mut sum = 0;
    let mut id = 1;

    for line in lines {
        let mut valid = true;
        let id_split = line.split(':');
        let id_strip = id_split.last().unwrap();
        let rounds = id_strip.split(';');
        for round in rounds {
            let color_split = round.split(',');
            for color_line in color_split {
                let mut split = color_line.trim().split(' ');

                let amount: i32 = split.next().unwrap().parse().unwrap();
                let color = split.next().unwrap();
                match color {
                    "red" => {
                        if amount > maximum.red {
                            valid = false;
                        }
                    }
                    "green" => {
                        if amount > maximum.green {
                            valid = false;
                        }
                    }
                    "blue" => {
                        if amount > maximum.blue {
                            valid = false;
                        }
                    }
                    _ => panic!("invalid color {color}"),
                }
            }
        }
        if valid {
            sum += id;
        }
        id += 1;
    }
    println!("Answer to part 1 is {sum}.");
}
