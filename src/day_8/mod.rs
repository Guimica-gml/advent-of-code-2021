use std::fs::read_to_string;

const INPUT_FILEPATH: &str = "./src/day_8/input.txt";

#[derive(Debug)]
#[allow(dead_code)]
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

fn part2(_entries: &Vec<Entry>) -> i32 {
    return 0;
}
