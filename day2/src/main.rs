use std::fs;

struct Maximum {
    red: i32,
    green: i32,
    blue: i32,
}

impl Maximum {
    fn get_power(self) -> i32 {
        self.red * self.green * self.blue
    }
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
    let mut powersum = 0;

    for line in lines {
        let mut game_maximum = Maximum {
            red: 1,
            green: 1,
            blue: 1,
        };
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
                        if amount > game_maximum.red {
                            game_maximum.red = amount;
                        }
                    }
                    "green" => {
                        if amount > maximum.green {
                            valid = false;
                        }
                        if amount > game_maximum.green {
                            game_maximum.green = amount;
                        }
                    }
                    "blue" => {
                        if amount > maximum.blue {
                            valid = false;
                        }
                        if amount > game_maximum.blue {
                            game_maximum.blue = amount;
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
        powersum += game_maximum.get_power();
    }
    println!("Answer to part 1 is {sum}.");
    println!("Answer to part 2 is {powersum}.");
}
