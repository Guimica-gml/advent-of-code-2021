use std::fs::read_to_string;

type HeightMap = Vec<Vec<i32>>;
type Point = (usize, usize);
type Basin = Vec<Point>;

const INPUT_FILEPATH: &str = "./input.txt";

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

fn for_each_neighbor(height_map: &HeightMap, point: Point, mut func: impl FnMut(Point)) {
    if point.1 != 0 {
        func((point.0, point.1 - 1));
    }
    if point.0 != 0 {
        func((point.0 - 1, point.1));
    }
    if point.1 + 1 != height_map.len() {
        func((point.0, point.1 + 1));
    }
    if point.0 + 1 != height_map[0].len() {
        func((point.0 + 1, point.1));
    }
}

fn is_low_point(height_map: &HeightMap, point: Point) -> bool {
    let value = height_map[point.1][point.0];
    let mut neighbors: Vec<i32> = vec![];
    for_each_neighbor(height_map, point, |npoint| neighbors.push(height_map[npoint.1][npoint.0]));
    neighbors.into_iter().all(|neighbor| value < neighbor)
}

fn get_low_points(height_map: &HeightMap) -> Vec<Point> {
    let mut low_points = vec![];

    for (y, row) in height_map.into_iter().enumerate() {
        for (x, _) in row.into_iter().enumerate() {
            if is_low_point(height_map, (x, y)) {
                low_points.push((x, y));
            }
        }
    }

    low_points
}

fn get_basin_from(height_map: &HeightMap, low_point: &Point) -> Basin {
    let mut basin: Basin = vec![];

    fn get_basin_from_impl(vec: &mut Basin, height_map: &HeightMap, low_point: &Point) {
        vec.push(*low_point);

        for_each_neighbor(height_map, *low_point, |npoint| {
            let nvalue = height_map[npoint.1][npoint.0];
            if nvalue != 9 && !vec.contains(&npoint) {
                get_basin_from_impl(vec, height_map, &npoint);
            }
        });
    }

    get_basin_from_impl(&mut basin, height_map, low_point);
    basin
}

fn part1(height_map: &HeightMap) -> i32 {
    let mut answer = 0;
    let low_points = get_low_points(height_map);

    for low_point in low_points {
        answer += height_map[low_point.1][low_point.0] + 1;
    }

    return answer;
}

fn part2(height_map: &HeightMap) -> i32 {
    let low_points = get_low_points(height_map);
    let mut basins: Vec<Basin> = vec![];
    let mut answer = 1;

    for low_point in &low_points {
        basins.push(get_basin_from(height_map, low_point));
    }

    basins.sort_by(|a, b| b.len().cmp(&a.len()));

    for i in 0..3 {
        answer *= basins[i].len() as i32;
    }

    return answer;
}
