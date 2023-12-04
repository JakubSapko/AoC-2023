use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Number {
    value: u32,
    start_index: usize,
    row: usize,
    length: usize,
    checked: bool,
}

fn get_all_numbers(input: &str) -> HashSet<Number> {
    let mut numbers: HashSet<Number> = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        let mut start_index = None;
        let mut end_index = 0;
        for (x, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                start_index.get_or_insert(x);
                end_index = x;
            } else if let Some(start) = start_index.take() {
                let value = line[start..=end_index].to_string();
                let value_casted = value.parse::<u32>().unwrap();
                let number = Number {
                    value: value_casted,
                    start_index: start,
                    row: y,
                    length: value.chars().count(),
                    checked: false,
                };
                numbers.insert(number);
            }
        }
        if let Some(start) = start_index.take() {
            let value = line[start..=end_index].to_string();
            let value_casted = value.parse::<u32>().unwrap();
            let number = Number {
                value: value_casted,
                start_index: start,
                row: y,
                length: value.chars().count(),
                checked: false,
            };
            numbers.insert(number);
        }
    }
    numbers
}

fn main() {
    let input = include_str!("../test.txt");
    let numbers = get_all_numbers(input);
    println!("{:?}", numbers);
}
