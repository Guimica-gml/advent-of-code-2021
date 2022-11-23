use std::fs::read_to_string;
use std::collections::HashMap;
use lazy_static::lazy_static;

const INPUT_FILEPATH: &str = "./input.txt";

lazy_static! {
    static ref OPPOSITE: HashMap<char, char> = HashMap::from([
        (')', '('),
        (']', '['),
        ('}', '{'),
        ('>', '<'),
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
    ]);
}

pub fn main() {
    let lines = parse_input(INPUT_FILEPATH);
    println!("The answer for part 1 is {}", part1(&lines));
    println!("The answer for part 2 is {}", part2(&lines));
}

fn parse_input(filepath: &str) -> Vec<String> {
    read_to_string(filepath).unwrap().lines().map(|s| s.to_string()).collect()
}

fn is_corrupted(line: &String) -> (bool, Vec<char>) {
    let mut chunck: Vec<char> = vec![];

    for letter in line.chars() {
        if ['[', '{', '(', '<'].contains(&letter) {
            chunck.push(letter);
        }
        else if [']', '}', ')', '>'].contains(&letter) {
            if *chunck.last().unwrap() == OPPOSITE[&letter] {
                chunck.pop();
            }
            else {
                chunck.push(letter);
                return (true, chunck);
            }
        }
    }

    return (false, chunck);
}

fn part1(lines: &Vec<String>) -> i64 {
    let mut answer = 0;

    let points: HashMap<char, i64> = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137),
    ]);

    for line in lines {
        let (is_corrupted, chunck) = is_corrupted(line);
        if is_corrupted {
            answer += points[&chunck.last().unwrap()];
        }
    }

    return answer;
}

fn part2(lines: &Vec<String>) -> i64 {
    let mut scores: Vec<i64> = vec![];

    let points: HashMap<char, i64> = HashMap::from([
        (')', 1),
        (']', 2),
        ('}', 3),
        ('>', 4),
    ]);

    for line in lines {
        let mut score = 0;
        let (is_corrupted, chunck) = is_corrupted(line);

        if !is_corrupted {
            for letter in chunck.into_iter().rev() {
                score *= 5;
                score += points[&OPPOSITE[&letter]];
            }
            scores.push(score);
        }
    }

    scores.sort();
    return scores[scores.len() / 2];
}
