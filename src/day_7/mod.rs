use std::fs::read_to_string;
use std::collections::HashMap;

const INPUT_FILEPATH: &str = "./src/day_7/input.txt";

pub fn main() {
    let positions = parse_input(INPUT_FILEPATH);
    println!("The answer to part 1 is {}", part1(&positions));
    println!("The answer to part 2 is {}", part2(&positions));
}

fn parse_input(filepath: &str) -> Vec<i32> {
    return read_to_string(filepath).unwrap().split(",").map(|s| s.parse().unwrap()).collect();
}

fn find_smallets_fuel_consumption(positions: &Vec<i32>, f: fn(i32, i32) -> i32) -> i32 {
    let min: i32 = *positions.into_iter().min().unwrap();
    let max: i32 = *positions.into_iter().max().unwrap();
    let mut possible_positios: HashMap<i32, i32> = HashMap::new();

    for target_pos in min..max {
        let mut fuel_consumption = 0;
        for current_pos in positions {
            fuel_consumption += f(*current_pos, target_pos);
        }
        possible_positios.insert(target_pos, fuel_consumption);
    }

    let smallest_fuel_consumption = possible_positios.values().min().unwrap();
    return *smallest_fuel_consumption;
}

fn part1(positions: &Vec<i32>) -> i32 {
    return find_smallets_fuel_consumption(positions, |current_pos: i32, target_pos: i32| -> i32 {
        i32::abs(target_pos - current_pos)
    })
}

fn part2(positions: &Vec<i32>) -> i32 {
    return find_smallets_fuel_consumption(positions, |current_pos: i32, target_pos: i32| -> i32 {
        let mut consumption = 0;
        for amount in 0..i32::abs(target_pos - current_pos) {
            consumption += amount + 1;
        }
        return consumption;
    })
}
