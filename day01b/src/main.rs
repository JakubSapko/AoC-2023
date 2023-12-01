use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn convert_string_to_number(line: String) -> u32 {
    let converted = line
        .chars()
        .enumerate()
        .into_iter()
        .map(|(index, character)| {
            if character.is_digit(10) {
                return Some(character.to_digit(10).unwrap());
            }
            let chars = line.chars().skip(index).take(5).collect::<String>();
            if chars.starts_with("one") {
                return Some(1);
            } else if chars.starts_with("two") {
                return Some(2);
            } else if chars.starts_with("three") {
                return Some(3);
            } else if chars.starts_with("four") {
                return Some(4);
            } else if chars.starts_with("five") {
                return Some(5);
            } else if chars.starts_with("six") {
                return Some(6);
            } else if chars.starts_with("seven") {
                return Some(7);
            } else if chars.starts_with("eight") {
                return Some(8);
            } else if chars.starts_with("nine") {
                return Some(9);
            } else {
                return None;
            }
        })
        .filter(|o| match o {
            Some(_val) => true,
            None => false,
        })
        .map(|val| val.unwrap())
        .collect::<Vec<u32>>();
    let first = converted.first().unwrap_or(&0).to_owned();
    let last = converted.last().unwrap_or(&0).to_owned();
    let calculated = first * 10 + last;
    return calculated;
}

fn main() {
    let input = read_lines("input.txt").unwrap();
    let result: u32 = input
        .map(|line| convert_string_to_number(line.unwrap()))
        .collect::<Vec<u32>>()
        .into_iter()
        .sum();
}
