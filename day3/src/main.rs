use std::borrow::Borrow;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Index;


fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let data = fs::read_to_string("day3.txt").unwrap();
    let lines: Vec<_> = data.lines().collect();
    let mut gamma: BTreeMap<u32, u8> = BTreeMap::new();
    let mut epsilon: BTreeMap<u32, u8> = BTreeMap::new();
    let mut value = 0;
    let mut epsi_value = 0;
    let mut zeros = 0;
    let mut ones = 0;
    for i in 1..13 {
        for line in &lines {
            value = line.chars().nth(i - 1).unwrap() as i32;
            match value {
                48 => zeros += 1, // 0
                49 => ones += 1, // 1
                _ => {}
            }
        }
        if ones > zeros {
            value = 1;
            epsi_value = 0;
        } else {
            value = 0;
            epsi_value = 1;
        }
        ones = 0;
        zeros = 0;
        gamma.insert(i as u32, value as u8);
        epsilon.insert(i as u32, epsi_value as u8);
    }
    let gamma_values: Vec<u8> = gamma.values().cloned().collect();
    let epsilon_values: Vec<u8> = epsilon.values().cloned().collect();
    let mut gamma_decimal = String::new();
    let mut epsilon_decimal = String::new();
    for gamma_value in gamma_values {
        gamma_decimal += gamma_value.to_string().borrow();
    }
    for epsilon_value in epsilon_values {
        epsilon_decimal += epsilon_value.to_string().borrow();
    }
    let gamma_decimal = isize::from_str_radix(&*gamma_decimal, 2).unwrap();
    let epsilon_decimal = isize::from_str_radix(&*epsilon_decimal, 2).unwrap();
    println!("{}", gamma_decimal * epsilon_decimal);
}

fn part_two() {
    let data = File::open("day3.txt").unwrap();
    let mut data: Vec<String> = BufReader::new(data).lines().map(|x| x.expect("No str")).collect();
    let mut oxygen_lines: Vec<String> = data.clone();
    let mut co2_lines = data.clone();
    let mut temp_vec: Vec<String> = Vec::new();
    let mut i = 0;
    while i < 13 {
        if oxygen_lines.len() == 1 { break; }
        let char: char = get_bit(&oxygen_lines, i);
        temp_vec = Vec::new();
        for oxygen_line in oxygen_lines {
            if oxygen_line.chars().nth((i) as usize).unwrap() == char {
                temp_vec.push(oxygen_line);
            }
        }
        oxygen_lines = temp_vec;
        temp_vec = Vec::new();
        i += 1;
    }
    i = 0;
    while i < 13 {
        if co2_lines.len() == 1 { break; }
        let char: char = get_bit(&co2_lines, i);
        temp_vec = Vec::new();
        for oxygen_line in co2_lines {
            if oxygen_line.chars().nth((i) as usize).unwrap() != char {
                temp_vec.push(oxygen_line);
            }
        }
        co2_lines = temp_vec;
        temp_vec = Vec::new();
        i += 1;
    }

    let oxygen_decimal = isize::from_str_radix(oxygen_lines.index(0), 2).unwrap();
    let co2_decimal = isize::from_str_radix(co2_lines.index(0), 2).unwrap();
    println!("oxy - {}, co2 - {}", oxygen_decimal, co2_decimal);
    println!("{}", oxygen_decimal * co2_decimal);
}

fn get_bit(vec: &Vec<String>, column: i32) -> char {
    let mut zero_count = 0;
    let mut one_count = 0;
    let mut char: char;
    for line in vec {
        char = line.chars().nth(column as usize).unwrap();
        match char {
            '0' => zero_count += 1,
            '1' => one_count += 1,
            _ => {}
        }
    }
    if one_count >= zero_count {
        return '1';
    }
    return '0';
}