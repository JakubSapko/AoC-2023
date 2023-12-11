use std::collections::HashMap;

#[derive(Debug)]
enum Directions {
    LEFT,
    RIGHT,
}

#[derive(Debug)]
struct Map {
    instructions: Vec<Directions>,
    nodes: HashMap<String, (String, String)>,
}

fn parse(input: &str) -> Map {
    let (instructions, nodes) = input.split_once("\n\n").unwrap();
    let instructions: Vec<Directions> = instructions
        .chars()
        .map(|c| match c {
            'L' => Directions::LEFT,
            'R' => Directions::RIGHT,
            _ => panic!("something went wrong"),
        })
        .collect();
    let mut nodes_m: HashMap<String, (String, String)> = HashMap::new();
    for node in nodes.lines() {
        let (root, children) = node.split_once(" = ").unwrap();
        let root = root.to_owned();
        let children: (String, String) = children
            .trim_start_matches("(")
            .trim_end_matches(")")
            .split_once(", ")
            .map(|child| (child.0.to_string(), child.1.to_string()))
            .unwrap();
        nodes_m.insert(root, children);
    }
    return Map {
        instructions,
        nodes: nodes_m,
    };
}

fn solve_a(map: Map) -> usize {
    let mut node: String = "AAA".to_owned();
    let mut instructions_index = 0;
    loop {
        let (left, right) = map.nodes.get(&node).unwrap();
        node = match map.instructions[instructions_index % map.instructions.len()] {
            Directions::LEFT => left.to_owned(),
            Directions::RIGHT => right.to_owned(),
        };
        instructions_index += 1;

        if node == "ZZZ" {
            break;
        }
    }
    return instructions_index;
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }
    (a * b) / gcd(a, b)
}

fn lcm_of_vector(numbers: &[u64]) -> u64 {
    if numbers.is_empty() {
        return 0;
    }

    let mut result = numbers[0];

    for &num in &numbers[1..] {
        result = lcm(result, num);
    }

    result
}

fn solve_b(map: Map) -> u64 {
    let mut nodes = vec![];
    for (root, _) in &map.nodes {
        if root.ends_with("A") {
            nodes.push(root.to_owned());
        }
    }
    println!("{}, {:?}", nodes.len(), nodes);
    let mut instructions_index = 0;
    let mut nodes_counts = Vec::new();

    for mut node in nodes {
        loop {
            let (left, right) = map.nodes.get(&node).unwrap();
            node = match map.instructions[instructions_index % map.instructions.len()] {
                Directions::LEFT => left.to_owned(),
                Directions::RIGHT => right.to_owned(),
            };
            instructions_index += 1;
            if node.ends_with('Z') {
                nodes_counts.push(instructions_index as u64);
                println!("done {:?}, {:?}, ", node, instructions_index,);
                instructions_index = 0;
                break;
            }
        }
    }
    println!("{:?}", nodes_counts);

    return lcm_of_vector(&nodes_counts);
}
fn main() {
    let input = include_str!("../input.txt");
    let map = parse(input);
    //    let result = solve_a(map);
    //    println!("{:?}", result);
    let result = solve_b(map);
    println!("{:?}", result);
}
