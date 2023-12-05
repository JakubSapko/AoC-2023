#[derive(Debug)]
struct Range {
    destination: u32,
    source: u32,
    steps: u32,
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}
fn parse_input(input: &str) -> (Vec<Map>, Vec<u32>) {
    let mut sections = input.split("\n\n");
    let seeds: Vec<u32> = sections
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|seed| seed.parse::<u32>().unwrap())
        .collect();
    let mut maps = Vec::new();
    for section in sections.filter(|sec| !sec.is_empty()) {
        let lines = section.lines();
        let mut ranges = Vec::new();
        for line in lines.skip(1) {
            let data = line.split_whitespace().collect::<Vec<&str>>();
            let destination = data[0].parse::<u32>().unwrap();
            let source = data[1].parse::<u32>().unwrap();
            let steps = data[2].parse::<u32>().unwrap();
            ranges.push(Range {
                destination,
                source,
                steps,
            });
        }
        maps.push(Map { ranges });
    }
    return (maps, seeds);
}

fn solve_a(seeds: &Vec<u32>, maps: &Vec<Map>) -> u32 {
    let min = u32::MAX;
    for mut seed in seeds {
        for map in maps {
            seed = map.get(seed);
        }
        min = min.min(&seed);
    }
    return min;
}
fn main() {
    let input = include_str!("../test.txt");
    let (maps, seeds) = parse_input(input);
    println!("{:?}", seeds);
    println!("{:?}", maps);
    let result = solve_a(&seeds, &maps);
}
