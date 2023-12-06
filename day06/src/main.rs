use std::iter::zip;

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn win_combinations(&self) -> i64 {
        let mut wins = 0;
        for i in 0..self.time {
            let dist_travelled = i * (self.time - i);
            if (dist_travelled > self.distance) {
                wins += 1;
            }
        }
        return wins;
    }
}
fn parse_input(input: &str) -> Vec<Race> {
    let res: Vec<_> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .map(|val| val.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    let times = &res[0];
    let distances = &res[1];
    let races: Vec<Race> = zip(times, distances)
        .map(|race| Race {
            time: *race.0,
            distance: *race.1,
        })
        .collect();

    return races;
}

fn solve_a(races: Vec<Race>) -> i64 {
    let solution = races
        .into_iter()
        .map(|race| race.win_combinations())
        .fold(1, |acc, x| acc * x);
    return solution;
}

fn solve_b(races: Vec<Race>) -> i64 {
    println!("{:?}", races);
    let mut temp_time = String::new();
    let mut temp_distance = String::new();
    for race in races {
        let x = race.time.to_string();
        let y = race.distance.to_string();
        temp_time = format!("{temp_time}{x}");
        temp_distance = format!("{temp_distance}{y}");
    }
    let temp = Race {
        time: temp_time.parse::<u64>().unwrap(),
        distance: temp_distance.parse::<u64>().unwrap(),
    };
    println!("{:?}", temp);
    let solution = temp.win_combinations();
    return solution;
}
fn main() {
    let input = include_str!("../input.txt");
    println!("{:?}", input);
    let parsed = parse_input(input);
    //let a = solve_a(parsed);
    let b = solve_b(parsed);
    println!("{:?}", b);
}
