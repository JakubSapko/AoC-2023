use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Card {
    id: u32,
    winning_numbers: HashSet<u32>,
    my_numbers: HashSet<u32>,
}

fn parse_input(input: &str) -> Vec<Card> {
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
    return result;
}

fn main() {
    let input = include_str!("../input.txt");
    let parsed_input = parse_input(input);
    let solution_a: &u32 = &parsed_input
        .clone()
        .into_iter()
        .map(|card| {
            card.clone()
                .my_numbers
                .intersection(&card.clone().winning_numbers)
                .count()
        })
        .filter(|x| x > &0)
        .map(|inter_count| u32::pow(2, (inter_count as u32) - 1))
        .sum();
    let games_won: &Vec<usize> = &parsed_input
        .clone()
        .into_iter()
        .map(|card| {
            card.clone()
                .my_numbers
                .intersection(&card.clone().winning_numbers)
                .count()
        })
        .collect::<Vec<usize>>();
    println!("{:?}", games_won);
    let mut times_played = (0..games_won.len()).collect::<Vec<_>>();
    let mut visited = 0;
    while let Some(i) = times_played.pop() {
        visited += 1;
        let card = games_won[i];
        if card == 0 as usize {
            continue;
        }
        for j in 0..card as usize {
            times_played.push(j + i + 1);
        }
    }
    println!("{:?}", visited);

    println!("{:?}", solution_a);
}
