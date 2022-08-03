use std::fs::read_to_string;
use std::collections::HashSet;

const INPUT_FILEPATH: &str = "./src/day_13/input.txt";

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct FoldPuzzle {
    paper: HashSet<Point>,
    instructions: Vec<(String, i32)>,
}

pub fn main() {
    let fold_puzzle = parse_input(INPUT_FILEPATH);
    println!("The answer for part 1 is {}", part1(fold_puzzle.clone()));
    println!("The answer for part 2 is:");
    part2(fold_puzzle.clone());
}

fn parse_input(filepath: &str) -> FoldPuzzle {
    let text = read_to_string(filepath).unwrap();
    let mut lines = text.lines();
    let mut paper: HashSet<Point> = HashSet::new();
    let mut instructions: Vec<(String, i32)> = vec![];

    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        let (x, y) = line.split_once(",").unwrap();
        paper.insert(Point { x: x.parse().unwrap(), y: y.parse().unwrap() });
    }

    for line in &mut lines {
        let usefull_line = line.split(" ").last().unwrap();
        let (axis, num) = usefull_line.split_once("=").unwrap();
        instructions.push((axis.to_string(), num.parse().unwrap()));
    }

    FoldPuzzle { paper, instructions }
}

fn fold_paper(paper: HashSet<Point>, instructions: Vec<(String, i32)>) -> HashSet<Point> {
    let mut paper = paper.clone();

    for (axis, index) in &instructions {
        let mut new_paper: HashSet<Point> = HashSet::new();
        for point in paper.into_iter() {
            let mut new_point = point.clone();
            if axis == "x" {
                if new_point.x > *index {
                    let a = new_point.x - *index;
                    new_point.x = *index - a;
                }
            }
            else if axis == "y" {
                if new_point.y > *index {
                    let a = new_point.y - *index;
                    new_point.y = *index - a;
                }
            }
            else {
                panic!();
            }
            new_paper.insert(new_point);
        }
        paper = new_paper;
    }

    return paper;
}

fn print_paper(paper: &HashSet<Point>) {
    let width = paper.into_iter().max_by(|a, b| a.x.partial_cmp(&b.x).unwrap()).unwrap().x;
    let height = paper.into_iter().max_by(|a, b| a.y.partial_cmp(&b.y).unwrap()).unwrap().y;

    for y in 0..height + 1 {
        for x in 0..width + 1 {
            if paper.contains(&Point {x, y}) {
                print!("#");
            }
            else {
                print!(".");
            }
        }
        print!("\n");
    }
}

fn part1(fold_puzzle: FoldPuzzle) -> usize {
    let paper = fold_paper(fold_puzzle.paper, fold_puzzle.instructions[0..1].to_vec());
    paper.len()
}

fn part2(fold_puzzle: FoldPuzzle) {
    let paper = fold_paper(fold_puzzle.paper, fold_puzzle.instructions);
    print_paper(&paper);
}
