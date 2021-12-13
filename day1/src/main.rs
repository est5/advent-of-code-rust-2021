use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::Index;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let mut count = 0;
    let content = File::open("day1.txt").expect("file wasn't found");
    let reader = BufReader::new(content);
    let numbers: Vec<isize> = reader
        .lines()
        .map(|line| line.unwrap().parse::<isize>().unwrap())
        .collect();
    let mut first = 100;
    for number in numbers {
        if number > first { count += 1; }
        first = number;
    }
    println!("{}", count);
}

fn part_two() {
    let mut count = 0;
    let content = File::open("day1.txt").expect("file wasn't found");
    let reader = BufReader::new(content);
    let numbers: Vec<isize> = reader
        .lines()
        .map(|line| line.unwrap().parse::<isize>().unwrap())
        .collect();
    let mut prev_idx_sum = numbers.index(0) + numbers.index(1) + numbers.index(2);
    let mut sum: isize = 0;
    for i in 1..numbers.len() - 2 {
        sum = numbers.index(i) + numbers.index(i + 1) + numbers.index(i + 2);
        if sum > prev_idx_sum { count += 1; }
        prev_idx_sum = sum;
    }
    println!("{}", count);
}
