use std::fs::read_to_string;

type HeightMap = Vec<Vec<i32>>;

const INPUT_FILEPATH: &str = "./src/day_9/input.txt";

pub fn main() {
    let height_map = parse_input(INPUT_FILEPATH);
    println!("The answer for part 1 is {}", part1(&height_map));
    println!("The answer for part 2 is {}", part2(&height_map));
}

fn parse_input(filepath: &str) -> HeightMap {
    read_to_string(filepath).unwrap().lines().map(|line| {
        line.chars().map(|letter| letter.to_string().parse::<i32>().unwrap()).collect()
    }).collect()
}

fn for_each_neighbor(height_map: &HeightMap, y: usize, x: usize, mut func: impl FnMut(i32)) {
    if y != 0 {
        func(height_map[y - 1][x]);
    }
    if x != 0 {
        func(height_map[y][x - 1]);
    }
    if y + 1 != height_map.len() {
        func(height_map[y + 1][x]);
    }
    if x + 1 != height_map[0].len() {
        func(height_map[y][x + 1]);
    }
}

fn is_low_point(height_map: &HeightMap, y: usize, x: usize) -> bool {
    let value = height_map[y][x];
    let mut neighbors: Vec<i32> = vec![];
    for_each_neighbor(height_map, y, x, |n_value| neighbors.push(n_value));
    neighbors.into_iter().all(|neighbor| { value < neighbor })
}

fn part1(height_map: &HeightMap) -> i32 {
    let mut answer = 0;

    for (y, row) in height_map.into_iter().enumerate() {
        for (x, value) in row.into_iter().enumerate() {
            if is_low_point(height_map, y, x) {
                answer += *value + 1;
            }
        }
    }

    return answer;
}

fn part2(_height_map: &HeightMap) -> i32 {
    return 0;
}
