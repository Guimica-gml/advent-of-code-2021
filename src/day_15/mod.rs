use std::fs::read_to_string;

const INPUT_FILEPATH: &str = "./src/day_15/sample.txt";

type Grid = Vec<Vec<i32>>;
type Point = (usize, usize);

#[derive(Debug)]
struct Node {
    number: i32,
    left: Box<Option<Node>>,
    right: Box<Option<Node>>,
}

impl Node {
    fn from(grid: &Grid, start: Point) -> Option<Node> {
        if start.0 >= grid[0].len() || start.1 >= grid.len() {
            return None;
        }

        Some(Node {
            number: grid[start.0][start.1],
            left: Box::from(Node::from(grid, (start.0, start.1 + 1))),
            right: Box::from(Node::from(grid, (start.0 + 1, start.1))),
        })
    }
}

pub fn main() {
    let node = parse_input(INPUT_FILEPATH);
    println!("The answer for part 1 is {}", part1(&node));
    println!("The answer for part 2 is {}", part2(&node));
}

fn parse_input(filepath: &str) -> Node {
    let grid = read_to_string(filepath).unwrap().lines().map(|line| {
        line.chars().map(|c| c.to_string().parse().unwrap()).collect()
    }).collect();

    Node::from(&grid, (0, 0)).unwrap()
}

fn get_best_path_risk(node: &Node) -> i32 {
    let num = node.number;
    let left = &*node.left;
    let right = &*node.right;

    if let Some(left) = left {
        if let Some(right) = right {
            if left.number < right.number {
                return num + get_best_path_risk(left);
            }
            else {
                return num + get_best_path_risk(right);
            }
        }

        return num + get_best_path_risk(left);
    }
    else if let Some(right) = right {
        return num + get_best_path_risk(right);
    }

    num
}

fn part1(node: &Node) -> i32 {
    get_best_path_risk(node)
}

fn part2(_node: &Node) -> i32 {
    return 0;
}
