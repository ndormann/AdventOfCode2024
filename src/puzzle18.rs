use crate::prelude::Aoc2024;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter, Write};

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Position {
    x: usize,
    y: usize,
}

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
}

impl Debug for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Field::FREE => f.write_char('.'),
            Field::BLOCKED => f.write_char('#'),
            Field::END => f.write_char('E'),
            Field::START => f.write_char('S'),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Node {
    pub field: Field,
    pub score: usize,
}

#[derive(PartialEq, Eq, Clone)]
struct Board {
    grid: Vec<Vec<Node>>,
    size: usize,
}

impl Board {
    pub fn new(positions: &Vec<Position>, step: usize, size: usize) -> Self {
        let mut grid: Vec<Vec<Node>> = Vec::new();
        let mut row: Vec<Node> = Vec::new();
        row.resize(size, Node::new(Field::FREE));
        grid.resize(size, row);
        for position in positions.iter().take(step) {
            grid[position.x][position.y] = Node::new(Field::BLOCKED);
        }
        grid[0][0] = Node::new(Field::START);
        grid[size - 1][size - 1] = Node::new(Field::END);
        Self { grid, size }
    }

    fn dfs(&mut self, x: i32, y: i32, score: usize) {
        let directions = [
            Direction::UP,
            Direction::RIGHT,
            Direction::LEFT,
            Direction::DOWN,
        ];
        if x < 0 || x >= self.size as i32 || y < 0 || y >= self.size as i32 {
            return;
        }
        let node = self.grid[x as usize][y as usize];

        if score < node.score {
            self.grid[x as usize][y as usize].score = score;
            if node.field == Field::FREE || node.field == Field::START {
                for direction in directions {
                    let (dx, dy) = direction.delta();
                    self.dfs(x + dx, y + dy, score + 1);
                }
            }
        }
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for el in row {
                f.write_fmt(format_args!("{:?}", el))?
            }
            f.write_char('\n')?
        }
        Ok(())
    }
}

impl Node {
    pub fn new(field: Field) -> Self {
        Self {
            field,
            score: usize::MAX,
        }
    }
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
}

impl From<&str> for Position {
    fn from(value: &str) -> Self {
        let mut parts = value.split(",").filter_map(|num| num.parse::<usize>().ok());
        Self {
            x: parts.next().unwrap(),
            y: parts.next().unwrap(),
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

fn get_positions(input: &String) -> Vec<Position> {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|&arg| !str::is_empty(arg))
        .map(Position::from)
        .collect()
}

pub struct Puzzle18 {}

impl Aoc2024 for Puzzle18 {
    fn name(&self) -> String {
        "Day 18: RAM Run".to_string()
    }

    fn solve_a(&self, input: &String) -> String {
        let positions = get_positions(input);

        let mut board = Board::new(&positions, 1024, 71);

        let (x, y) = get_position(&board.grid, Field::START).expect("START not found");

        board.dfs(x, y, 0);

        let (end_x, end_y) = get_position(&board.grid, Field::END).expect("END not found");

        let end_score = board.grid[end_x as usize][end_y as usize].score;
        end_score.to_string()
    }

    fn solve_b(&self, input: &String) -> String {
        let positions = get_positions(input);
        let pos_index: Vec<usize> = (0..positions.len()).collect();

        let board = Board::new(&positions, 1024, 71);
        let (end_x, end_y) = get_position(&board.grid, Field::END).expect("END not found");

        let target_index = pos_index.binary_search_by(|&index| {
            let mut board = Board::new(&positions, index, 71);
            board.dfs(0, 0, 0);
            let new_end_score = board.grid[end_x as usize][end_y as usize].score;
            if new_end_score < usize::MAX {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        if let Err(target_index) = target_index {
            let position = &positions[target_index - 1];
            format!("{},{}", position.x, position.y)
        } else {
            "None found".to_string()
        }
    }
}
