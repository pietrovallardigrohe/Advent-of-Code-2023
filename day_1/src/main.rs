use std::{
    error::Error,
    fs::{self},
};

fn main() -> Result<(), Box<dyn Error>> {
    let input_text = fs::read_to_string("src/input.txt").unwrap();
    let lines = input_text.lines();

    let total: i32 = lines
        .map(|line| get_first_and_last(line))
        .collect::<Vec<i32>>()
        .iter()
        .sum();

    println!("{total}");

    Ok(())
}

fn get_first_and_last(line: &str) -> i32 {
    println!("Line input: \"{}\"", line);
    let current = line.chars().filter(|c| c.is_ascii_digit()).map(|c| c);

    let first = current.clone().next();
    let last = current.last();

    match first {
        Some(first) => format!("{}{}", first, last.unwrap()).parse().unwrap(),
        None => return 0,
    }
}
