use std::fs::read_to_string;
use std::fmt::Display;
use pad::{self, PadStr};

const INPUT_FILEPATH: &str = "./input.txt";

#[derive(Debug, Clone)]
struct Table {
    width: usize,
    height: usize,
    values: Vec<i32>,
    marked_values: Vec<i32>,
    closed: bool,
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text = String::new();

        let mut i = 0;
        for value in &self.values {
            text += &format!("{}", value.to_string().pad_to_width_with_alignment(3, pad::Alignment::Right));
            text += " |";
            i += 1;
            if i >= self.height {
                text += "\n";
                i = 0;
            }
        }

        return write!(f, "{}", text);
    }
}

impl Table {
    fn from(text: &str) -> Table {
        return Table {
            values: text.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<i32>>(),
            marked_values: vec![],
            width: 5,
            height: 5,
            closed: false,
        };
    }

    fn mark(&mut self, number: i32) {
        if self.values.contains(&number) {
            self.marked_values.push(number);
        }
    }

    fn won(&mut self) -> bool {
        for y in 0..self.height {
            let mut bingo = true;
            for x in 0..self.width {
                let value = self.values[(y * self.width) + x];
                if !self.marked_values.contains(&value) {
                    bingo = false;
                }
                if !bingo { break; }
            }
            if bingo {
                self.closed = true;
                return true;
            }
        }

        for x in 0..self.width {
            let mut bingo = true;
            for y in 0..self.height {
                let value = self.values[(y * self.width) + x];
                if !self.marked_values.contains(&value) {
                    bingo = false;
                }
                if !bingo { break; }
            }
            if bingo {
                self.closed = true;
                return true;
            }
        }

        return false;
    }

    fn get_unmarked_value(&self) -> Vec<i32> {
        let mut unmarked = vec![];
        for value in &self.values {
            if !self.marked_values.contains(value) {
                unmarked.push(value.clone());
            }
        }
        return unmarked;
    }
}

#[derive(Debug, Clone)]
struct BingoGame {
    tables: Vec<Table>,
    steps: Vec<i32>,
    step_index: usize,
}

impl BingoGame {
    fn from_file(filepath: &str) -> BingoGame {
        let text_bocks: Vec<String> = read_to_string(filepath)
            .unwrap()
            .split("\r\n\r\n")
            .map(|s| s.to_owned())
            .collect();

        let steps = text_bocks[0].split(",").map(|s| s.parse().unwrap()).collect();
        let tables = text_bocks[1..].into_iter().map(|text| Table::from(text)).collect();

        return BingoGame { tables, steps, step_index: 0 };
    }

    fn game_ended(&self) -> bool {
        for table in &self.tables {
            if !table.closed {
                return false;
            }
        }
        return true;
    }

    fn play(&mut self) -> (Vec<Table>, Vec<i32>) {
        let mut tables: Vec<Table> = vec![];
        let mut numbers: Vec<i32> = vec![];

        loop {
            let number = self.steps[self.step_index];
            self.step_index += 1;

            for table in &mut self.tables {
                if !table.closed {
                    table.mark(number);
                }
            }

            for table in &mut self.tables {
                if !table.closed && table.won() {
                    tables.push(table.clone());
                    numbers.push(number);
                }
            }

            if self.game_ended() { break; }
        }

        return (tables, numbers);
    }
}

pub fn main() {
    let bingo_game = BingoGame::from_file(INPUT_FILEPATH);
    println!("The answer for part 1 is {}", part1(bingo_game.clone()));
    println!("The answer for part 2 is {}", part2(bingo_game.clone()));
}

fn part1(mut bingo_game: BingoGame) -> i32 {
    let unmarked_values: i32;
    let last_number: i32;

    let (tables, numbers) = bingo_game.play();
    unmarked_values = tables.first().unwrap().get_unmarked_value().into_iter().sum();
    last_number = numbers.first().unwrap().clone();

    return unmarked_values * last_number;
}

fn part2(mut bingo_game: BingoGame) -> i32 {
    let unmarked_values: i32;
    let last_number: i32;

    let (tables, numbers) = bingo_game.play();
    unmarked_values = tables.last().unwrap().get_unmarked_value().into_iter().sum();
    last_number = numbers.last().unwrap().clone();

    return unmarked_values * last_number;
}
