use std::fs::read_to_string;
use std::collections::HashMap;

const INPUT_FILEPATH: &str = "./input.txt";

struct Polymer {
    template: String,
    connections: HashMap<(char, char), char>,
}

pub fn main() {
    let polymer = parse_input(INPUT_FILEPATH);
    println!("The answer for part 1 is {}", part1(&polymer));
    println!("The answer for part 2 is {}", part2(&polymer));
}

fn parse_input(filepath: &str) -> Polymer {
    let text = read_to_string(filepath).unwrap();
    let mut lines = text.lines();

    let template = lines.next().unwrap().to_string();
    let mut connections = HashMap::new();

    for line in lines.skip(1) {
        let (start, end) = line.split_once(" -> ").unwrap();
        let mut start = start.chars();
        let mut end = end.chars();
        connections.insert((start.next().unwrap(), start.next().unwrap()), end.next().unwrap());
    }

    Polymer { template, connections }
}

fn count(polymer: &Polymer, memo: &mut HashMap<(char, char, i64), HashMap<char, i64>>, pair: (char, char), steps: i64) -> HashMap<char, i64> {
    let (char1, char2) = pair;
    let letter = polymer.connections[&(char1, char2)].clone();

    if steps <= 0 {
        return HashMap::new();
    }

    if let Some(letter_map) = memo.get(&(char1, char2, steps)) {
        return letter_map.clone();
    }

    let mut counts = HashMap::new();

    for (letter, amount) in count(polymer, memo, (char1, letter), steps - 1) {
        *counts.entry(letter).or_default() += amount;
    }
    for (letter, amount) in count(polymer, memo, (letter, char2), steps - 1) {
        *counts.entry(letter).or_default() += amount;
    }

    *counts.entry(letter.clone()).or_default() += 1;
    *memo.entry((char1, char2, steps)).or_default() = counts.clone();
    counts
}

fn simulate(polymer: &Polymer, steps: i64) -> i64 {
    let mut letter_map: HashMap<char, i64> = HashMap::new();
    let mut memo: HashMap<(char, char, i64), HashMap<char, i64>> = HashMap::new();

    for letter in polymer.template.chars() {
        *letter_map.entry(letter).or_default() += 1;
    }

    for i in 0..polymer.template.len() - 1 {
        let mut pair = polymer.template[i..i+2].chars();
        let char1 = pair.next().unwrap();
        let char2 = pair.next().unwrap();

        for (letter, amount) in count(&polymer, &mut memo, (char1, char2), steps) {
            *letter_map.entry(letter).or_default() += amount;
        }
    }

    let values: Vec<&i64> = letter_map.values().collect();
    let smallest = *values.iter().min().unwrap();
    let greatest = *values.iter().max().unwrap();
    greatest - smallest
}

fn part1(polymer: &Polymer) -> i64 {
    simulate(polymer, 10)
}

fn part2(polymer: &Polymer) -> i64 {
    simulate(polymer, 40)
}
