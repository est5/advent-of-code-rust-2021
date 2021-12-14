use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let file = File::open("day2.txt").expect("unable to read file");
    let file = BufReader::new(file);
    let mut depth = 0;
    let mut horizon = 0;
    for f in file.lines() {
        let f = f.expect("no input");
        let mut temp = f.split(" ");
        let pair = (temp.next().unwrap(), temp.next().unwrap());
        let direction: &str = pair.0;
        let value: i32 = pair.1.parse().unwrap();
        match direction.to_lowercase().as_str() {
            "forward" => horizon += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => panic!("Wrong direction : {}", direction)
        }
    }
    println!("{}", horizon * depth);
}

fn part_two() {
    let file = File::open("day2.txt").expect("unable to read file");
    let file = BufReader::new(file);
    let mut aim = 0;
    let mut depth = 0;
    let mut horizon = 0;
    for f in file.lines() {
        let f = f.expect("no input");
        let mut temp = f.split(" ");
        let pair = (temp.next().unwrap(), temp.next().unwrap());
        let direction: &str = pair.0;
        let value: i32 = pair.1.parse().unwrap();
        match direction.to_lowercase().as_str() {
            "forward" => {
                horizon += value;
                if aim > 0 { depth += value * aim; }
            }
            "down" => aim += value,
            "up" => aim -= value,
            _ => panic!("Wrong direction : {}", direction)
        }
    }
    println!("{}", horizon * depth);
}