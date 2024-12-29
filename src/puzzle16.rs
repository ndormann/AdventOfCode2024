use crate::prelude::Aoc2024;
use std::fmt::{Debug, Formatter, Write};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    UP,
    RIGHT, // Each bit represents a direction
    DOWN,
    LEFT,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Field {
    FREE,
    BLOCKED,
    END,
    START,
    BEST,
}

impl Debug for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Field::FREE => f.write_char('.'),
            Field::BLOCKED => f.write_char('#'),
            Field::END => f.write_char('E'),
            Field::START => f.write_char('S'),
            Field::BEST => f.write_char('O'),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Node {
    pub field: Field,
    pub score: usize,
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.field))
    }
}

impl From<u8> for Node {
    fn from(value: u8) -> Self {
        Self {
            field: Field::from(value),
            score: usize::MAX,
        }
    }
}
impl From<u8> for Field {
    fn from(value: u8) -> Self {
        match value {
            b'#' => Field::BLOCKED,
            b'E' => Field::END,
            b'S' => Field::START,
            _ => Field::FREE,
        }
    }
}

impl Direction {
    fn delta(&self) -> (i32, i32) {
        match self {
            Direction::UP => (-1, 0),   // UP
            Direction::RIGHT => (0, 1), // RIGHT
            Direction::DOWN => (1, 0),  // DOWN
            Direction::LEFT => (0, -1), // LEFT
        }
    }

    fn clockwise(&self) -> Self {
        match self {
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
        }
    }

    fn counter_clockwise(&self) -> Self {
        match self {
            Direction::UP => Direction::LEFT,
            Direction::RIGHT => Direction::UP,
            Direction::DOWN => Direction::RIGHT,
            Direction::LEFT => Direction::DOWN,
        }
    }
}

fn get_position(board: &Vec<Vec<Node>>, field: Field) -> Option<(i32, i32)> {
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j].field == field {
                return Some((i as i32, j as i32));
            }
        }
    }
    None
}

fn dfs(board: &mut Vec<Vec<Node>>, x: i32, y: i32, score: usize, direction: Direction) {
    let node = board[x as usize][y as usize];
    if node.field == Field::BLOCKED {
        return;
    }
    if score < node.score {
        board[x as usize][y as usize].score = score;
        let (dx, dy) = direction.delta();
        dfs(board, x + dx, y + dy, score + 1, direction);
        let clockwise = direction.clockwise();
        let (dx, dy) = clockwise.delta();
        board[x as usize][y as usize].score = score + 1000;
        dfs(board, x + dx, y + dy, score + 1 + 1000, clockwise);
        let counter_clockwise = direction.counter_clockwise();
        let (dx, dy) = counter_clockwise.delta();
        dfs(board, x + dx, y + dy, score + 1 + 1000, counter_clockwise);
    }
}

fn backtrack(
    board: &mut Vec<Vec<Node>>,
    x: i32,
    y: i32,
    score: usize,
    direction: Direction,
) -> bool {
    let node = board[x as usize][y as usize];
    if node.field == Field::BLOCKED {
        return false;
    } else if node.field == Field::END {
        return true;
    }

    if score <= node.score {
        let mut found_goal = false;
        let (dx, dy) = direction.delta();
        found_goal |= backtrack(board, x + dx, y + dy, score + 1, direction);
        let clockwise = direction.clockwise();
        let (dx, dy) = clockwise.delta();
        found_goal |= backtrack(board, x + dx, y + dy, score + 1 + 1000, clockwise);
        let counter_clockwise = direction.counter_clockwise();
        let (dx, dy) = counter_clockwise.delta();
        found_goal |= backtrack(board, x + dx, y + dy, score + 1 + 1000, counter_clockwise);

        if found_goal {
            board[x as usize][y as usize].field = Field::BEST;
        }
        found_goal
    } else {
        false
    }
}

pub struct Puzzle16 {}

impl Aoc2024 for Puzzle16 {
    fn name(&self) -> String {
        "Day 16: Reindeer Maze".to_string()
    }

    fn solve_a(&self, input: &String) -> String {
        let mut board: Vec<Vec<Node>> = input
            .lines()
            .map(str::trim)
            .map(|line| line.bytes().map(Node::from).collect())
            .collect();

        let (x, y) = get_position(&board, Field::START).expect("START not found");

        dfs(&mut board, x, y, 0, Direction::RIGHT);

        let (x, y) = get_position(&board, Field::END).expect("END not found");

        let end_score = board[x as usize][y as usize].score;
        end_score.to_string()
    }

    fn solve_b(&self, input: &String) -> String {
        let mut board: Vec<Vec<Node>> = input
            .lines()
            .map(str::trim)
            .map(|line| line.bytes().map(Node::from).collect())
            .collect();

        let (x, y) = get_position(&board, Field::START).expect("START not found");

        dfs(&mut board, x, y, 0, Direction::RIGHT);
        backtrack(&mut board, x, y, 0, Direction::RIGHT);

        let best_places: usize = board
            .iter()
            .map(|row| row.iter().filter(|node| node.field == Field::BEST).count())
            .sum();
        (best_places + 1).to_string()
    }
}
