use std::collections::HashMap;

macro_rules! regex {
    ($raw:expr) => {{
        static REGEX: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        REGEX.get_or_init(|| regex::Regex::new($raw).unwrap())
    }};
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Number {
    value: u32,
    part_number: bool,
}

fn parse_input(input: &str) -> (Vec<Number>, HashMap<Vec<usize>, Vec<u32>>) {
    let mut symbols = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.char_indices() {
            if !c.is_ascii_digit() && c != '.' {
                symbols.insert(vec![x, y], c);
            }
        }
    }
    let mut numbers = Vec::new();
    let mut ratios = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for val in regex!(r"\d+").find_iter(line) {
            let value: u32 = val.as_str().parse().unwrap();
            let mut part_number = false;
            for nx in val.start().saturating_sub(1)..=val.end() {
                for ny in y.saturating_sub(1)..=y + 1 {
                    let pos = vec![nx, ny];
                    let symbol = symbols.get(&pos);
                    part_number |= symbol.is_some();

                    if symbol == Some(&'*') {
                        ratios.entry(pos).or_insert(Vec::new()).push(value);
                    }
                }
            }
            numbers.push(Number { value, part_number });
        }
    }
    (numbers, ratios)
}
fn main() {
    let input = include_str!("../domin_input.txt");
    let parsed = parse_input(input);
    let part_a = parsed
        .0
        .into_iter()
        .filter(|x| x.part_number)
        .map(|x| x.value)
        .sum::<u32>();
    println!("{:?}", part_a);
    let part_b: u32 = parsed
        .1
        .into_iter()
        .filter(|(_, x)| x.len() == 2)
        .map(|(_, vals)| vals[0] * vals[1])
        .sum();
    println!("{:?}", part_b);
}
