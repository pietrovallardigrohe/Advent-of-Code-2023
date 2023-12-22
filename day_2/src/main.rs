use core::panic;
use std::{fs, vec};

use regex::Regex;

fn main() {
    let input_text = fs::read_to_string("src/input.txt").unwrap();
    let mut lines = input_text.lines();

    let mut total_id = 0;
    let parser_regex = Regex::new(r"([0-9]+ (r|g|b))").unwrap();

    for (index, line) in lines.enumerate() {
        let cubes_vector: Vec<&str> = parser_regex.find_iter(line).map(|m| m.as_str()).collect();
        let mut passed = true;
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
                }
                'g' => {
                    if amount > 13 {
                        passed = false;
                    }
                }
                'b' => {
                    if amount > 14 {
                        passed = false;
                    }
                }
                _ => panic!("shouldn't happen"),
            }
        }
        if passed {
            total_id += index + 1;
        }
    }
    println!("{:?}", total_id);

    // let parser_string = lines.for_each(|line| {});
}
