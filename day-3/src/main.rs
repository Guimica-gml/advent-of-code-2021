use std::fs::read_to_string;

const INPUT_FILEPATH: &str = "./input.txt";

pub fn main() {
    let diagnostic = parse_input(INPUT_FILEPATH);
    println!("The answer for part 1 is {}", part1(&diagnostic));
    println!("The answer for part 2 is {}", part2(&diagnostic));
}

fn parse_input(filepath: &str) -> Vec<String> {
    return read_to_string(filepath).unwrap().lines().map(|s| s.to_owned()).collect();
}

fn get_most_relevant(diagnostic: &Vec<String>, column: usize) -> String {
    let mut ones = 0;
    let mut zeros = 0;

    for entry in diagnostic {
        if entry.chars().nth(column).unwrap() == '0' {
            zeros += 1;
        }
        else {
            ones += 1;
        }
    }

    return if ones >= zeros { "1".to_owned() } else { "0".to_owned() };
}

fn get_least_relevant(diagnostic: &Vec<String>, column: usize) -> String {
    let mut ones = 0;
    let mut zeros = 0;

    for entry in diagnostic {
        if entry.chars().nth(column).unwrap() == '0' {
            zeros += 1;
        }
        else {
            ones += 1;
        }
    }

    return if ones >= zeros { "0".to_owned() } else { "1".to_owned() };
}

fn part1(diagnostic: &Vec<String>) -> i32 {
    let mut gamma = "".to_owned();
    let mut epsilon = "".to_owned();
    let entry_size = diagnostic[0].len();

    for i in 0..entry_size {
        if get_most_relevant(diagnostic, i) == "1" {
            gamma += "1";
            epsilon += "0";
        }
        else {
            gamma += "0";
            epsilon += "1";
        }
    }

    let gamma = i32::from_str_radix(&gamma, 2).unwrap();
    let epsilon = i32::from_str_radix(&epsilon, 2).unwrap();

    return gamma * epsilon;
}

fn get_oxygen_generator_rating(diagnostic: &Vec<String>, column: usize) -> String {
    if diagnostic.len() == 1 {
        return diagnostic[0].clone();
    }

    let most_common = get_most_relevant(diagnostic, column);
    let mut new_diagnostic: Vec<String> = vec![];

    for entry in diagnostic {
        if entry.chars().nth(column).unwrap().to_string() == most_common {
            new_diagnostic.push(entry.clone());
        }
    }

    return get_oxygen_generator_rating(&new_diagnostic, column + 1);
}

fn get_co2_scrubber_rating(diagnostic: &Vec<String>, column: usize) -> String {
    if diagnostic.len() == 1 {
        return diagnostic[0].clone();
    }

    let least_common = get_least_relevant(diagnostic, column);
    let mut new_diagnostic: Vec<String> = vec![];

    for entry in diagnostic {
        if entry.chars().nth(column).unwrap().to_string() == least_common {
            new_diagnostic.push(entry.clone());
        }
    }

    return get_co2_scrubber_rating(&new_diagnostic, column + 1);
}

fn part2(diagnostic: &Vec<String>) -> i32 {
    let gamma = get_oxygen_generator_rating(diagnostic, 0);
    let epsilon = get_co2_scrubber_rating(diagnostic, 0);

    let gamma = i32::from_str_radix(&gamma, 2).unwrap();
    let epsilon = i32::from_str_radix(&epsilon, 2).unwrap();

    return gamma * epsilon;
}
