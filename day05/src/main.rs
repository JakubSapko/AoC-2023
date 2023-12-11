#[derive(Debug)]
struct Range {
    destination: u64,
    source: u64,
    steps: u64,
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}

impl Map {
    fn get(&self, val: &u64) -> u64 {
        for range in &self.ranges {
            if val >= &range.source && val <= &(range.source + range.steps) {
                return range.destination + val - range.source;
            }
        }
        *val
    }
}

fn parse_input(input: &str) -> (Vec<Map>, Vec<u64>) {
    let mut sections = input.split("\n\n");
    let seeds: Vec<u64> = sections
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|seed| seed.parse::<u64>().unwrap())
        .collect();
    let mut maps = Vec::new();
    for section in sections.filter(|sec| !sec.is_empty()) {
        let lines = section.lines();
        let mut ranges = Vec::new();
        for line in lines.skip(1) {
            let data = line.split_whitespace().collect::<Vec<&str>>();
            let destination = data[0].parse::<u64>().unwrap();
            let source = data[1].parse::<u64>().unwrap();
            let steps = data[2].parse::<u64>().unwrap();
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

fn solve_a(seeds: Vec<u64>, maps: Vec<Map>) -> u64 {
    let mut min = u64::MAX;
    for mut seed in seeds {
        for map in &maps {
            seed = map.get(&seed);
        }
        min = min.min(seed);
    }
    return min;
}

fn solve_b(seeds: Vec<u64>, maps: Vec<Map>) -> u64 {
    let result = seeds
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .map(|seed| {
            let mut min = u64::MAX;
            for mut seed in seed.0..=seed.0 + seed.1 {
                for map in &maps {
                    seed = map.get(&seed);
                }
                min = min.min(seed);
            }
            return min;
        })
        .min()
        .unwrap();
    return result;
}
fn main() {
    let input = include_str!("../input.txt");
    let (maps, seeds) = parse_input(input);
    //let result = solve_a(seeds, maps);
    let result = solve_b(seeds, maps);
    println!("{:?}", result);
}
