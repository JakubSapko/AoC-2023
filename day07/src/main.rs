use ::std::collections::{HashMap, HashSet};
use itertools::Itertools;
use std::cmp::Ordering;
#[derive(Debug, Clone)]
struct Hand {
    bid: u32,
    values: Vec<u32>,
    r#type: Type,
}

const CARDS: &str = "AKQJT98765432";
const CARDS_B: &str = "AKQT98765432J";

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Type {
    High = 1,
    Pair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

fn determine_type(values: &Vec<u32>) -> Type {
    let distinct_values: HashSet<&u32> = HashSet::from_iter(values);
    let leng = distinct_values.len();
    let mut count_map = HashMap::new();
    for num in values {
        *count_map.entry(num).or_insert(0) += 1;
    }
    let max_count = count_map.values().cloned().max().unwrap();

    if leng == 1 {
        return Type::FiveOfAKind;
    };
    if leng == 4 {
        return Type::Pair;
    };
    if leng == 5 {
        return Type::High;
    };

    // leng 2 can be: FourOfAKind, FullHouse
    if leng == 2 {
        if max_count == 3 {
            return Type::FullHouse;
        }
        return Type::FourOfAKind;
    };
    // leng 3 can be: TwoPair, ThreeOfAKind
    if max_count == 3 {
        return Type::ThreeOfAKind;
    }
    return Type::TwoPair;
}

fn determine_type_for_b(values: &Vec<u32>) -> Type {
    let mut counts = [0; 13];
    for &c in values {
        counts[13 - c as usize] += 1;
    }

    let jacks = counts[12];
    let counts = counts[0..12]
        .iter()
        .copied()
        .filter(|x| *x != 0)
        .sorted()
        .rev()
        .collect::<Vec<_>>();

    if counts.len() <= 1 || counts[0] + jacks == 5 {
        return Type::FiveOfAKind;
    } else if counts[0] + jacks == 4 {
        return Type::FourOfAKind;
    } else if ((counts[0] + jacks == 3) && (counts[1] == 2))
        || ((counts[0] == 3) && (counts[1] + jacks == 2))
    {
        return Type::FullHouse;
    } else if counts[0] + jacks == 3 {
        return Type::ThreeOfAKind;
    } else if (counts[0] + jacks == 2 && counts[1] == 2)
        || (counts[0] == 2 && counts[1] + jacks == 2)
    {
        return Type::TwoPair;
    } else if counts[0] + jacks == 2 {
        return Type::Pair;
    } else {
        return Type::High;
    }
}
fn parse(input: &str, b: bool) -> Vec<Hand> {
    let hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let splt: Vec<_> = line.split_whitespace().collect();
            let values: Vec<u32> = splt[0]
                .as_bytes()
                .into_iter()
                .map(|&c| {
                    if b {
                        13 - CARDS_B.find(c as char).unwrap() as u32
                    } else {
                        13 - CARDS.find(c as char).unwrap() as u32
                    }
                })
                .collect();
            let bid = splt[1].parse::<u32>().unwrap();
            let mut r#type = Type::High;
            if b {
                r#type = determine_type_for_b(&values);
            } else {
                r#type = determine_type(&values);
            }
            return Hand {
                bid,
                values,
                r#type,
            };
        })
        .collect();

    return hands;
}

fn determine_order(a: &Hand, b: &Hand) -> Ordering {
    for (val_a, val_b) in a
        .values
        .clone()
        .into_iter()
        .zip(b.values.clone().into_iter())
    {
        if val_a != val_b {
            println!("{}, {}", val_a, val_b);
            return val_a.cmp(&val_b);
        }
    }
    Ordering::Equal
}

fn solve(mut hands: Vec<Hand>) -> usize {
    hands.sort_by(|a, b| {
        //handle conflicts when two of the same types are next to each other
        if a.r#type == b.r#type {
            determine_order(&a, &b)
        } else {
            a.r#type.cmp(&b.r#type)
        }
    });
    let result = hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| {
            let val = (i + 1) * hand.bid as usize;
            println!("{} * {}", hand.bid, i + 1);
            println!("{:?}", val);
            return val;
        })
        .sum();
    return result;
}
fn main() {
    let input = include_str!("../input.txt");
    println!("{:?}", input);
    let hands = parse(input, true);
    println!("{:?}", hands);
    let result = solve(hands);
    println!("{:?}", result);
}
