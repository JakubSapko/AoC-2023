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

fn main() {
    let input = read_lines("input.txt").unwrap();
    let collection: u32 = input
        .map(|line| {
            let line = line.unwrap();
            let first_digit = line
                .chars()
                .find(|ch| ch.is_digit(10))
                .unwrap()
                .to_digit(10)
                .unwrap_or(0);
            let last_digit = line
                .chars()
                .rev()
                .find(|ch| ch.is_digit(10))
                .unwrap()
                .to_digit(10)
                .unwrap_or(0);
            let sum = first_digit * 10 + last_digit;
            sum
        })
        .collect::<Vec<u32>>()
        .into_iter()
        .sum();
    println!("{:?}", collection);
}
