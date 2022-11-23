use std::fs::read_to_string;
use std::collections::HashMap;

const INPUT_FILEPATH: &str = "./input.txt";

#[derive(Debug)]
struct Entry {
    inputs: Vec<String>,
    outputs: Vec<String>,
}

pub fn main() {
    let entries = parse_input(INPUT_FILEPATH);
    println!("The answer to part 1 is {}", part1(&entries));
    println!("The answer to part 2 is {}", part2(&entries));
}

fn parse_input(filepath: &str) -> Vec<Entry> {
    read_to_string(filepath).unwrap().lines().map(|line| {
        let mut info = line.split(" | ");
        let inputs = info.next().unwrap().split(" ").map(|s| s.to_owned()).collect();
        let outputs = info.next().unwrap().split(" ").map(|s| s.to_owned()).collect();
        Entry { inputs, outputs }
    }).collect()
}

fn part1(entries: &Vec<Entry>) -> i32 {
    let mut answer = 0;

    for entry in entries {
        for digit in &entry.outputs {
            if [2, 3, 4, 7].contains(&digit.len()) {
                answer += 1;
            }
        }
    }

    return answer;
}

fn part2(entries: &Vec<Entry>) -> i32 {
    let mut answer = 0;
    let known_lens: HashMap<usize, i32> = HashMap::from([
        (2, 1),
        (4, 4),
        (3, 7),
        (7, 8),
    ]);

    for entry in entries {
        let mut key_mapping: HashMap<i32, String> = HashMap::new();

        for digit in &entry.inputs {
            if [2, 3, 4, 7].contains(&digit.len()) {
                for _ in digit.chars() {
                    if key_mapping.contains_key(&known_lens[&digit.len()]) { continue; }
                    key_mapping.insert(known_lens[&digit.len()], digit.clone());
                }
            }
        }

        let mut num_str = "".to_owned();
        for digit in &entry.outputs {
            if [2, 3, 4, 7].contains(&digit.len()) {
                num_str += &known_lens[&digit.len()].to_string();
            }
            else if digit.len() == 5 {
                let digit_1 = &key_mapping[&1];
                if is_subset(&digit.chars().collect(), &digit_1.chars().collect()) {
                    num_str += "3";
                    continue;
                }

                let digit_4 = &key_mapping[&4];
                let similarity = get_similarity_to(&digit.chars().collect(), &digit_4.chars().collect());
                if similarity == 3 {
                    num_str += "5";
                }
                else {
                    num_str += "2";
                }
            }
            else if digit.len() == 6 {
                let digit_7 = &key_mapping[&7];
                if !is_subset(&digit.chars().collect(), &digit_7.chars().collect()) {
                    num_str += "6";
                    continue;
                }

                let digit_4 = &key_mapping[&4];
                if is_subset(&digit.chars().collect(), &digit_4.chars().collect()) {
                    num_str += "9";
                    continue;
                }

                num_str += "0";
            }
        }
        answer += num_str.parse::<i32>().unwrap();
    }

    return answer;
}

fn is_subset(a: &Vec<char>, b: &Vec<char>) -> bool {
    return b.iter().all(|item| a.contains(item));
}

fn get_similarity_to(a: &Vec<char>, b: &Vec<char>) -> i32 {
    let mut answer = 0;
    b.iter().all(|item| {
        if a.contains(item) {
            answer += 1;
        }
        true
    });
    return answer;
}
