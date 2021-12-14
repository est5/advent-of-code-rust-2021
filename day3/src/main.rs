use std::borrow::Borrow;
use std::collections::{BTreeMap, HashMap};
use std::fs;


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
    let epsilon_decimal =isize::from_str_radix(&*epsilon_decimal, 2).unwrap();
    println!("{}", gamma_decimal * epsilon_decimal);
}
