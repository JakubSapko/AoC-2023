use itertools::Itertools;
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

#[derive(Debug)]
struct Subset {
    green: u32,
    blue: u32,
    red: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    subsets: Vec<Subset>,
}

fn get_game_id(game_string: &str) -> u32 {
    return game_string[5..].parse::<u32>().unwrap();
}

fn get_game_subsets(subsets_string: &str) -> Vec<Subset> {
    let game_subsets: Vec<&str> = subsets_string.split(";").collect();
    let game_subsets_individual: Vec<_> = game_subsets
        .into_iter()
        .map(|subset| subset.split(",").collect::<Vec<_>>())
        .collect();
    let game_subsets_parsed = game_subsets_individual
        .into_iter()
        .map(|vector_subset| {
            let mut result = Subset {
                red: 0,
                green: 0,
                blue: 0,
            };
            for item in vector_subset {
                let parts: Vec<&str> = item.split_whitespace().collect();
                let number = parts[0].parse::<u32>().unwrap();
                match parts[1] {
                    "red" => result.red += number,
                    "green" => result.green += number,
                    "blue" => result.blue += number,
                    _ => {}
                }
            }
            return result;
        })
        .collect::<Vec<Subset>>();
    return game_subsets_parsed;
}

fn build_game(line: String) -> Game {
    let parsed_line: Vec<&str> = line.split(":").collect();
    let game_id = get_game_id(parsed_line[0]);
    let subsets = get_game_subsets(parsed_line[1]);
    return Game {
        id: game_id,
        subsets,
    };
}

fn filter_impossible(game: &Game) -> bool {
    let mut is_possible = true;
    for subset in &game.subsets {
        if (subset.red > 12 || subset.green > 13 || subset.blue > 14) {
            is_possible = false;
        }
    }
    return is_possible;
}
fn main() {
    let input = read_lines("input.txt").unwrap();
    let col: Vec<_> = input.map(|line| build_game(line.unwrap())).collect();
    println!("{:?}", col);
    let result_games = col.into_iter().filter(|game| filter_impossible(game));
    let solution: u32 = result_games.into_iter().map(|game| game.id).sum();
    println!("{:?}", solution);
}
