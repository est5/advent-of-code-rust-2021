use std::fs;

struct Fish {
    state: usize,
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

fn fish(numbers: Vec<u64>) -> Vec<Fish> {
    let mut fishes: Vec<Fish> = Vec::new();
    for number in numbers {
        fishes.push(
            Fish {
                state: number as usize
            }
        )
    }
    fishes
}

fn get_numbers() -> Vec<u64> {
    let data = fs::read_to_string("day6.txt").expect("wrong file");
    let data: Vec<u64> = data.trim().split(",").map(|x| x.parse().expect("numbers")).collect();
    data
}

fn stupid_part_one() {
    let mut fishes = fish(get_numbers());
    let days = 256;
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

fn clever_part_one_and_two(days: usize) {
    let mut array:[usize;9] = fs::read_to_string("day6.txt").expect("wrong file")// 9 cuz 0 day
        // counts too
        .split(",")
        .fold([0; 9], |mut array, fish| { //add each fish to it day
            array[fish.parse::<usize>().unwrap()] += 1; // number of fishes per day
            // ex 0 fishes for 7th day, 110 fishes for 1st day etc
            array
        });

    for day in 1..days {
        array[(day + 7) % 9] += array[day % 9]; // 7 number of days to spawn new fish ( 0 incl)
        // %9 - day to put fish (idx)
        //day 1 , on 8 day += 1st day
        //day 2 , on 0 day += 7day
        //day 3 , on 1 += as on 6 etc... 1 fish == 1 fish so just sum each day
    }
    println!("{:?}", array); // num of fishes per day
    println!("{:?}", array.iter().sum::<usize>()); // sum of all fishes
}

fn main() {
    clever_part_one_and_two(256);
}
