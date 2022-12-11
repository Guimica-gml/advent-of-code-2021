use std::fs::read_to_string;

const INPUT_FILEPATH: &str = "./input.txt";

type Command = (String, i32);

pub fn main() {
	let commands = parse_input(INPUT_FILEPATH);
	println!("The answer for part 1 is {}", part1(&commands));
	println!("The answer for part 2 is {}", part2(&commands));
}

fn parse_input(filepath: &str) -> Vec<Command> {
	let text = read_to_string(filepath).unwrap();
	let commands: Vec<Command> = text.lines().map(|s| {
		let mut command = s.split(" ");
		return (command.next().unwrap().to_owned(), command.next().unwrap().parse::<i32>().unwrap());
	}).collect();
	return commands;
}

fn part1(commands: &Vec<Command>) -> i32 {
	let mut position = 0;
	let mut depth = 0;

	for command in commands {
		if command.0 == "forward" {
			position += command.1;
		}
		else if command.0 == "up" {
			depth -= command.1;
		}
		else if command.0 == "down" {
			depth += command.1;
		}
	}

	return position * depth;
}

fn part2(commands: &Vec<Command>) -> i32 {
	let mut position = 0;
	let mut depth = 0;
	let mut aim = 0;

	for command in commands {
		if command.0 == "forward" {
			position += command.1;
			depth += command.1 * aim;
		}
		else if command.0 == "up" {
			aim -= command.1;
		}
		else if command.0 == "down" {
			aim += command.1;
		}
	}

	return position * depth;
}
