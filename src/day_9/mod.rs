use std::fs::read_to_string;

type HeightMap = Vec<Vec<i32>>;
type LowPoint = (usize, usize);
type Basin = Vec<(usize, usize)>;

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

fn for_each_neighbor(height_map: &HeightMap, x: usize, y: usize, mut func: impl FnMut(usize, usize)) {
    if y != 0 {
        func(x, y - 1);
    }
    if x != 0 {
        func(x - 1, y);
    }
    if y + 1 != height_map.len() {
        func(x, y + 1);
    }
    if x + 1 != height_map[0].len() {
        func(x + 1, y);
    }
}

fn is_low_point(height_map: &HeightMap, x: usize, y: usize) -> bool {
    let value = height_map[y][x];
    let mut neighbors: Vec<i32> = vec![];
    for_each_neighbor(height_map, x, y, |nx, ny| neighbors.push(height_map[ny][nx]));
    neighbors.into_iter().all(|neighbor| value < neighbor)
}

fn part1(height_map: &HeightMap) -> i32 {
    let mut answer = 0;

    for (y, row) in height_map.into_iter().enumerate() {
        for (x, value) in row.into_iter().enumerate() {
            if is_low_point(height_map, x, y) {
                answer += *value + 1;
            }
        }
    }

    return answer;
}

fn get_basin_from(height_map: &HeightMap, low_point: &LowPoint) -> Basin {
    let mut basin: Basin = vec![];

    fn get_basin_from_impl(vec: &mut Basin, height_map: &HeightMap, low_point: &LowPoint) {
        vec.push((low_point.0, low_point.1));

        for_each_neighbor(height_map, low_point.0, low_point.1, |nx, ny| {
            let nvalue = height_map[ny][nx];
            if nvalue != 9 && !vec.contains(&(nx, ny)) {
                get_basin_from_impl(vec, height_map, &(nx, ny));
            }
        });
    }

    get_basin_from_impl(&mut basin, height_map, low_point);
    basin
}

fn part2(height_map: &HeightMap) -> i32 {
    let mut low_points: Vec<LowPoint> = vec![];
    let mut basins: Vec<Basin> = vec![];

    for (y, row) in height_map.into_iter().enumerate() {
        for (x, _) in row.into_iter().enumerate() {
            if is_low_point(height_map, x, y) {
                low_points.push((x, y));
            }
        }
    }

    for low_point in &low_points {
        basins.push(get_basin_from(height_map, low_point));
    }

    basins.sort_by(|a, b| b.len().cmp(&a.len()));
    let mut answer = 1;

    for i in 0..3 {
        answer *= basins[i].len() as i32;
    }

    return answer;
}
