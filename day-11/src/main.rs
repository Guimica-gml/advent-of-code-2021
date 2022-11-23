use std::fs::read_to_string;

type Grid = Vec<Vec<i32>>;
type Point = (usize, usize);

const INPUT_FILEPATH: &str = "./input.txt";

pub fn main() {
    let grid = parse_input(INPUT_FILEPATH);
    println!("The answer for part 1 is {}", part1(grid.clone()));
    println!("The answer for part 2 is {}", part2(grid.clone()));
}

fn parse_input(filepath: &str) -> Grid {
    read_to_string(filepath).unwrap().lines().map(|line| {
        line.chars().map(|letter| letter.to_string().parse::<i32>().unwrap()).collect()
    }).collect()
}

fn for_each_neighbor(grid: &mut Grid, point: Point, mut func: impl FnMut(Point, &mut i32)) {
    if point.1 != 0 {
        func((point.0, point.1 - 1), &mut grid[point.1 - 1][point.0]);
    }
    if point.0 != 0 {
        func((point.0 - 1, point.1), &mut grid[point.1][point.0 - 1]);
    }
    if point.1 + 1 != grid.len() {
        func((point.0, point.1 + 1), &mut grid[point.1 + 1][point.0]);
    }
    if point.0 + 1 != grid[0].len() {
        func((point.0 + 1, point.1), &mut grid[point.1][point.0 + 1]);
    }
    if point.1 != 0 && point.0 != 0 {
        func((point.0 - 1, point.1 - 1), &mut grid[point.1 - 1][point.0 - 1]);
    }
    if point.1 + 1 != grid.len() && point.0 != 0 {
        func((point.0 - 1, point.1 + 1), &mut grid[point.1 + 1][point.0 - 1]);
    }
    if point.0 + 1 != grid[0].len() && point.1 != 0 {
        func((point.0 + 1, point.1 - 1), &mut grid[point.1 - 1][point.0 + 1]);
    }
    if point.1 + 1 != grid.len() && point.0 + 1 != grid[0].len() {
        func((point.0 + 1, point.1 + 1), &mut grid[point.1 + 1][point.0 + 1]);
    }
}

fn should_flash(grid: &Grid) -> bool {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] > 9 {
                return true;
            }
        }
    }

    return false;
}

fn increase_light_rate(grid: &mut Grid) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            grid[y][x] += 1;
        }
    }
}

fn get_flash_amount(grid: &Grid) -> i32 {
    let mut amount = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 0 {
                amount += 1;
            }
        }
    }

    return amount;
}

fn all_flashed(grid: &Grid) -> bool {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] != 0 {
                return false;
            }
        }
    }

    return true;
}

fn part1(mut grid: Grid) -> i32 {
    let mut answer = 0;

    for _ in 0..100 {
        increase_light_rate(&mut grid);
        let mut exploded: Vec<Point> = vec![];

        loop {
            for y in 0..grid.len() {
                for x in 0..grid[y].len() {
                    if grid[y][x] > 9 {
                        for_each_neighbor(&mut grid, (x, y), |npoint, value| {
                            if !exploded.contains(&npoint) {
                                *value += 1;
                            }
                        });
                        grid[y][x] = 0;
                        exploded.push((x, y));
                    }
                }
            }

            if !should_flash(&grid) {
                break;
            }
        }

        answer += get_flash_amount(&grid);
    }

    return answer;
}

fn part2(mut grid: Grid) -> i32 {
    let mut answer = 0;

    loop {
        increase_light_rate(&mut grid);
        let mut exploded: Vec<Point> = vec![];

        loop {
            for y in 0..grid.len() {
                for x in 0..grid[y].len() {
                    if grid[y][x] > 9 {
                        for_each_neighbor(&mut grid, (x, y), |npoint, value| {
                            if !exploded.contains(&npoint) {
                                *value += 1;
                            }
                        });
                        grid[y][x] = 0;
                        exploded.push((x, y));
                    }
                }
            }

            if !should_flash(&grid) {
                break;
            }
        }

        answer += 1;
        if all_flashed(&grid) {
            break;
        }
    }

    return answer;
}
