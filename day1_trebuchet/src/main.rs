use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn get_input(filename: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn get_left_number(line: &String) -> Option<u32> {
    for c in line.chars() {
        if c.is_ascii_digit() {
            return c.to_digit(10);
        }
    }
    None
}

/**
 * Ugly, but general idea is to insert digit the middle (alas second position),
 * of matched words to prevent the issue of overwriting other valid words
 */
fn substitute_digits(mut line: String) -> String {
    let digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "zero",
    ];

    for i in 0..digits.len() {
        let mut location = line.find(digits[i]);
        while location.is_some() {
            let location_val = location.unwrap();
            line.replace_range(
                location_val + 1..location_val + 2,
                &(char::from_digit((i + 1) as u32, 10).unwrap().to_string()[..]),
            );
            location = line.find(digits[i]);
        }
    }

    line
}

fn solve_part_1(input: &Vec<String>) -> u32 {
    let mut result: u32 = 0;
    for line in input {
        let left = match get_left_number(line) {
            Some(num) => num,
            None => panic!("Could not find number in line: {line}"),
        };

        let right = match get_left_number(&(line.chars().rev().collect::<String>())) {
            Some(num) => num,
            None => panic!("Could not find number in line: {line}"),
        };

        result += left * 10 + right
    }

    result
}

fn solve_part_2(input: &Vec<String>) -> u32 {
    let mut result: u32 = 0;
    for line_unprocessed in input {
        let line = substitute_digits(line_unprocessed.clone());
        //println!("{line}");

        let left = match get_left_number(&line) {
            Some(num) => num,
            None => panic!("Could not find number in line: {line}"),
        };

        let right = match get_left_number(&(line.chars().rev().collect::<String>())) {
            Some(num) => num,
            None => panic!("Could not find number in line: {line}"),
        };
        //println!("Left: {left}, Right: {right}");
        result += left * 10 + right
    }

    result
}

fn main() {
    let filename = "input.txt";
    let lines = match get_input(filename) {
        Ok(lines) => lines,
        Err(_) => panic!("Failed reading input file... exiting."),
    };

    let part_1_result = solve_part_1(&lines);
    println!("Result of Part 1: {part_1_result}");

    let part_2_result = solve_part_2(&lines);
    println!("Result of Part 2: {part_2_result}");
}
