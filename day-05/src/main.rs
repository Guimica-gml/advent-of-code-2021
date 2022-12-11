use std::fs::read_to_string;
use std::collections::HashMap;

const INPUT_FILEPATH: &str = "./input.txt";

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Point {
	x: i32,
	y: i32,
}

type Path = (Point, Point);

pub fn main() {
	let paths = parse_input(INPUT_FILEPATH);
	println!("The answer for part 1 is {}", part1(&paths));
	println!("The answer for part 2 is {}", part2(&paths));
}

fn parse_input(filepath: &str) -> Vec<Path> {
	return read_to_string(filepath).unwrap().lines().map(|s| {
		let mut points = s.split(" -> ");
		let mut start = points.next().unwrap().split(",");
		let mut end = points.next().unwrap().split(",");
		return (
			Point {
				x: start.next().unwrap().parse().unwrap(),
				y: start.next().unwrap().parse().unwrap(),
			},
			Point {
				x: end.next().unwrap().parse().unwrap(),
				y: end.next().unwrap().parse().unwrap(),
			},
		);
	}).collect();
}

fn add_to_points(points: &mut HashMap<Point, i32>, key: Point) {
	if points.contains_key(&key) {
		points.insert(key.clone(), points.get(&key).unwrap() + 1);
	}
	else {
		points.insert(key.clone(), 1);
	}
}

fn part1(paths: &Vec<Path>) -> i32 {
	let mut points: HashMap<Point, i32> = HashMap::new();

	for path in paths {
		let (start, end) = path;
		let mut current = start.clone();

		if start.x != end.x && start.y != end.y { continue; }
		add_to_points(&mut points, start.clone());

		loop {
			current.x += i32::signum(end.x - current.x);
			current.y += i32::signum(end.y - current.y);
			add_to_points(&mut points, current.clone());

			if current == *end { break; }
		}
	}

	let mut answer = 0;

	for (_, value) in points {
		if value > 1 {
			answer += 1;
		}
	}

	return answer;
}

fn part2(paths: &Vec<Path>) -> i32 {
	let mut points: HashMap<Point, i32> = HashMap::new();

	for path in paths {
		let (start, end) = path;
		let mut current = start.clone();
		add_to_points(&mut points, start.clone());

		loop {
			current.x += i32::signum(end.x - current.x);
			current.y += i32::signum(end.y - current.y);
			add_to_points(&mut points, current.clone());

			if current == *end { break; }
		}
	}

	let mut answer = 0;

	for (_, value) in points {
		if value > 1 {
			answer += 1;
		}
	}

	return answer;
}
