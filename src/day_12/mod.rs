use std::fs::read_to_string;
use std::collections::HashMap;

const INPUT_FILEPATH: &str = "./src/day_12/input.txt";

pub fn main() {
    let cave_mapping = parse_input(INPUT_FILEPATH);
    println!("The answer for part 1 is {}", part1(&cave_mapping));
    println!("The answer for part 2 is {}", part2(&cave_mapping));
}

fn parse_input(filepath: &str) -> HashMap<String, Vec<String>> {
    let mut cave_mapping: HashMap<String, Vec<String>> = HashMap::new();
    let text = read_to_string(filepath).unwrap();

    for line in text.lines().map(|s| s.to_string()).collect::<Vec<String>>() {
        let mut path = line.split("-");
        let entry = path.next().unwrap().to_string();
        let exit = path.next().unwrap().to_string();

        if cave_mapping.contains_key(&entry) {
            cave_mapping.get_mut(&entry).unwrap().push(exit.clone());
        }
        else {
            cave_mapping.insert(entry.clone(), vec![exit.clone()]);
        }

        if cave_mapping.contains_key(&exit) {
            cave_mapping.get_mut(&exit).unwrap().push(entry);
        }
        else {
            cave_mapping.insert(exit, vec![entry]);
        }
    }

    cave_mapping
}

fn is_big_cave(cave: &str) -> bool {
    cave.chars().next().unwrap().is_uppercase()
}

fn get_paths_from(cave_mapping: &HashMap<String, Vec<String>>) -> Vec<Vec<String>> {
    let mut paths: Vec<Vec<String>> = vec![];

    fn get_paths_from_impl(paths: &mut Vec<Vec<String>>, path_info: (&mut Vec<String>, &mut Vec<String>), cave_mapping: &HashMap<String, Vec<String>>, from: &str) {
        let (path, already_visited) = path_info;

        for possible_path in &cave_mapping[from] {
            let mut branch = path.clone();
            let mut already_visited_branch = already_visited.clone();

            if already_visited_branch.contains(&possible_path) {
                if possible_path == "end" {
                    path.push("end".to_string());
                }
                continue;
            }

            if !is_big_cave(&possible_path) {
                already_visited_branch.push(possible_path.clone());
            }

            branch.push(possible_path.clone());
            get_paths_from_impl(paths, (&mut branch, &mut already_visited_branch), cave_mapping, &possible_path);
            paths.push(branch);
        }
    }

    get_paths_from_impl(&mut paths, (&mut vec!["start".to_string()], &mut vec!["start".to_string(), "end".to_string()]), cave_mapping, "start");
    paths.into_iter().filter(|e| e.last().unwrap() == "end").collect()
}

fn part1(cave_mapping: &HashMap<String, Vec<String>>) -> i32 {
    let paths = get_paths_from(cave_mapping);
    return paths.len() as i32;
}

fn part2(_cave_mapping: &HashMap<String, Vec<String>>) -> i32 {
    return 0;
}
