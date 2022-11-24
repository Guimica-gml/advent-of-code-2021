use std::fs::read_to_string;

const INPUT_FILEPATH: &str = "./input.txt";

pub fn main() {
	let numbers = parse_input(INPUT_FILEPATH);
	println!("The answer to part 1 is {}", part1(&numbers));
	println!("The answer to part 2 is {}", part2(&numbers));
}

fn parse_input(filepath: &str) -> Vec<i32> {
	let text = read_to_string(filepath).unwrap();
	let numbers: Vec<i32> = text.lines().map(|s| s.parse().unwrap()).collect();
	return numbers;
}

fn part1(numbers: &Vec<i32>) -> i32 {
	let mut answer = 0;

	for i in 1..numbers.len() {
		let num = numbers[i];
		if num > numbers[i - 1] { answer += 1; }
	}

	return answer;
}

fn part2(numbers: &Vec<i32>) -> i32 {
	let mut answer = 0;

	let mut last_number: i32 = numbers[0..3].iter().sum();
	for i in 1..numbers.len() - 2 {
		let currect_number: i32 = numbers[i..i+3].iter().sum();
		if currect_number > last_number { answer += 1; }
		last_number = currect_number;
	}

	return answer;
}
