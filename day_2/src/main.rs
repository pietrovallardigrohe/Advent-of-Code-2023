use core::panic;
use std::{fs, vec};

use regex::Regex;

fn main() {
    let input_text = fs::read_to_string("src/input.txt").unwrap();
    let lines = input_text.lines();

    let mut total_id = 0;
    let mut total_power_biggest_amount: u32 = 0;
    let parser_regex = Regex::new(r"([0-9]+ (r|g|b))").unwrap();

    for (index, line) in lines.enumerate() {
        let cubes_vector: Vec<&str> = parser_regex.find_iter(line).map(|m| m.as_str()).collect();
        let mut passed = true;
        let mut rgb: Vec<u32> = vec![0, 0, 0];
        // Game loop
        for vector in cubes_vector {
            let amount = Regex::new("[0-9]+")
                .unwrap()
                .find(vector)
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap();
            let color = vector.chars().last().unwrap();
            match color {
                'r' => {
                    if amount > 12 {
                        passed = false;
                    }
                    if amount > rgb[0] {
                        rgb[0] = amount;
                    }
                }
                'g' => {
                    if amount > 13 {
                        passed = false;
                    }
                    if amount > rgb[1] {
                        rgb[1] = amount;
                    }
                }
                'b' => {
                    if amount > 14 {
                        passed = false;
                    }
                    if amount > rgb[2] {
                        rgb[2] = amount;
                    }
                }
                _ => panic!("shouldn't happen"),
            }
        }
        if passed {
            total_id += index + 1;
        }
        total_power_biggest_amount += rgb[0] * rgb[1] * rgb[2];
    }
    println!(
        "Total: {:?} Total Power: {:?}",
        total_id, total_power_biggest_amount
    );

    // let parser_string = lines.for_each(|line| {});
}
