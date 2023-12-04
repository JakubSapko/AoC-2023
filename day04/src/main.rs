use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: HashSet<u32>,
    my_numbers: HashSet<u32>,
}

fn parse_input(input: &str) {
    let result: Vec<_> = input
        .lines()
        .map(|line| line.split(':').collect::<Vec<_>>())
        .map(|split_line| {
            let card_number = split_line[0].split_whitespace().collect::<Vec<_>>()[1]
                .parse::<u32>()
                .unwrap();
            let numbers: Vec<_> = split_line[1].split("|").collect();
            let winning_numbers = numbers[0]
                .split_whitespace()
                .collect::<Vec<_>>()
                .into_iter()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<HashSet<u32>>();
            let my_numbers = numbers[1]
                .split_whitespace()
                .collect::<Vec<_>>()
                .into_iter()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<HashSet<u32>>();
            return Card {
                id: card_number,
                winning_numbers,
                my_numbers,
            };
        })
        .collect();
    println!("{:?}", result);
}

fn main() {
    let input = include_str!("../test.txt");
    parse_input(input);
}
