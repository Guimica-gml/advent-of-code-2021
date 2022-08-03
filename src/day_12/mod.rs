use std::fs::read_to_string;
use std::collections::{HashMap, HashSet};

type CaveMapping = HashMap<String, Vec<String>>;

const INPUT_FILEPATH: &str = "./src/day_12/input.txt";

pub fn main() {
    let cave_mapping = parse_input(INPUT_FILEPATH);
    println!("The answer for part 1 is {}", part1(&cave_mapping));
    println!("The answer for part 2 is {}", part2(&cave_mapping));
}

fn parse_input(filepath: &str) -> CaveMapping {
    let mut cave_mapping: CaveMapping = HashMap::new();
    let text = read_to_string(filepath).unwrap();

    for line in text.lines() {
        let (entry, exit) = line.split_once("-").map(|s| (s.0.to_string(), s.1.to_string())).unwrap();
        cave_mapping.entry(entry.clone()).or_default().push(exit.clone());
        cave_mapping.entry(exit).or_default().push(entry);
    }

    cave_mapping
}

fn is_samll_cave(cave: &str) -> bool {
    cave.chars().next().unwrap().is_lowercase()
}

fn get_paths_from(cave_mapping: &CaveMapping, dup_small_cave: bool) -> Vec<Vec<String>> {
    fn get_paths_from_impl(paths: &mut Vec<Vec<String>>, cave_mapping: &CaveMapping, mut path: Vec<String>, visited: HashSet<String>, from: &str, dup_small_cave: bool) {
        path.push(from.to_string());

        if from == "end" {
            paths.push(path);
            return;
        }

        for to in &cave_mapping[from] {
            let mut visited = visited.clone();
            let mut dup_small_cave = dup_small_cave;

            if is_samll_cave(to) {
                if visited.contains(to) {
                    if dup_small_cave && to != "start" && to != "end" {
                        dup_small_cave = false;
                    }
                    else {
                        continue;
                    }
                }
                visited.insert(to.clone());
            }

            get_paths_from_impl(paths, cave_mapping, path.clone(), visited, to, dup_small_cave);
        }
    }

    let mut paths: Vec<Vec<String>> = vec![];
    let init_visited = HashSet::from(["start".to_owned()]);

    get_paths_from_impl(&mut paths, cave_mapping, vec![], init_visited, "start", dup_small_cave);
    paths
}

fn part1(cave_mapping: &CaveMapping) -> usize {
    get_paths_from(cave_mapping, false).len()
}

fn part2(cave_mapping: &CaveMapping) -> usize {
    get_paths_from(cave_mapping, true).len()
}
