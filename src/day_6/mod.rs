use std::fs::read_to_string;
use std::collections::HashMap;

const INPUT_FILEPATH: &str = "./src/day_6/input.txt";

pub fn main() {
    let population: Vec<i64> = parse_input(INPUT_FILEPATH);
    println!("The anwer for part 1 is {}", part1(&population));
    println!("The anwer for part 2 is {}", part2(&population));
}

fn parse_input(filepath: &str) -> Vec<i64> {
    return read_to_string(filepath)
        .unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
}

fn simulate_population(initial_population: &Vec<i64>, days: i64) -> i64 {
    let mut population: HashMap<i64, i64> = HashMap::from([
        (0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0)
    ]);

    for fish in initial_population {
        population.insert(*fish, population[fish] + 1);
    }

    let mut population_growth = 0;

    for _ in 0..days {
        for key in 0..9 {
            if key == 0 {
                population_growth += population[&0];
                population.insert(7, population[&7] + population[&0]);
                population.insert(0, 0);
            }
            else {
                population.insert(key - 1, population[&(key - 1)] + population[&key]);
                population.insert(key, 0);
            }
        }

        population.insert(8, population[&8] + population_growth);
        population_growth = 0;
    }

    return population.values().sum();
}

fn part1(initial_population: &Vec<i64>) -> i64 {
    return simulate_population(initial_population, 80);
}

fn part2(initial_population: &Vec<i64>) -> i64 {
    return simulate_population(initial_population, 256);
}
