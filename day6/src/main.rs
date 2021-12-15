use std::fs;

struct Fish {
    state: i32,
}

impl Fish {
    fn get_new() -> Self {
        Fish {
            state: 8
        }
    }

    fn day_passed(&mut self) -> bool {
        if self.state == 0 {
            self.state = 6;
            return true;
        } else {
            self.state -= 1;
        }
        false
    }
}

fn fish(numbers: Vec<i32>) -> Vec<Fish> {
    let mut fishes: Vec<Fish> = Vec::new();
    for number in numbers {
        fishes.push(
            Fish {
                state: number
            }
        )
    }
    fishes
}

fn get_numbers() -> Vec<i32> {
    let data = fs::read_to_string("day6.txt").expect("wrong file");
    let data: Vec<i32> = data.trim().split(",").map(|x| x.parse().expect("numbers")).collect();
    data
}

fn part_one() {
    let mut fishes = fish(get_numbers());
    let days = 80;
    for _day in 0..days {
        for fish in 0..fishes.len() {
            let create_new = fishes[fish].day_passed();
            if create_new {
                fishes.push(Fish::get_new())
            }
        }
    }
    println!("{}", fishes.len());
}

fn main() {
    part_one();
}
