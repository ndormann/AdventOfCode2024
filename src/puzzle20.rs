use crate::prelude::Aoc2024;
use std::cmp::{max, min, Ordering};
use std::collections::BinaryHeap;
use std::fmt::{Debug, Formatter, Write};

#[derive(Hash, Eq, PartialEq, PartialOrd, Ord, Debug, Clone)]
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

#[derive(PartialEq, Eq)]
struct State {
    position: Position,
    cost: usize,
}

#[derive(PartialEq, Eq, Clone)]
struct Board {
    grid: Vec<Vec<Node>>,
    size: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Board {
    pub fn new(grid: Vec<Vec<Node>>) -> Self {
        let size = grid.len();
        Self { grid, size }
    }

    pub fn bfs(&mut self, x: usize, y: usize) {
        let directions = [
            Direction::UP,
            Direction::RIGHT,
            Direction::LEFT,
            Direction::DOWN,
        ];
        self.grid[x][y].score = 0;
        let mut heap = BinaryHeap::new();
        heap.push(State {
            position: Position { x, y },
            cost: 0,
        });

        while let Some(State { position, cost }) = heap.pop() {
            let next_cost = cost + 1;
            for direction in directions {
                let (dx, dy) = direction.delta();
                let x = (position.x as i32 + dx) as usize;
                let y = (position.y as i32 + dy) as usize;
                let next_node = self.grid[x][y];
                if next_node.field != Field::BLOCKED && next_node.score > next_cost {
                    heap.push(State {
                        position: Position { x, y },
                        cost: next_cost,
                    });
                    self.grid[x][y].score = next_cost;
                }
            }
        }
    }

    fn find_nodes_within(&self, x: i32, y: i32, cheat_length: usize) -> Vec<usize> {
        let start_x = max(x - cheat_length as i32, 1) as usize;
        let end_x = min(x + cheat_length as i32 + 1, self.size as i32 - 1) as usize;
        let start_y = max(y - cheat_length as i32, 1) as usize;
        let end_y = min(y + cheat_length as i32 + 1, self.size as i32 - 1) as usize;

        let mut options = Vec::new();
        for i in start_x..end_x {
            for j in start_y..end_y {
                let dx = (i as i32 - x).abs() as usize;
                let dy = (j as i32 - y).abs() as usize;
                let manhatten_dist = dx + dy;
                if manhatten_dist > cheat_length {
                    continue;
                }
                let node = self.grid[i][j];
                if node.field != Field::BLOCKED {
                    options.push(node.score + manhatten_dist);
                }
            }
        }
        options
    }

    pub fn find_shortcuts(&self, cheat_length: usize) -> Vec<usize> {
        let mut shortcuts = Vec::new();
        for i in 1..self.size - 1 {
            for j in 1..self.size - 1 {
                if self.grid[i][j].field != Field::BLOCKED {
                    let start_score = self.grid[i][j].score;
                    let options = self.find_nodes_within(i as i32, j as i32, cheat_length);
                    shortcuts.extend(
                        options
                            .iter()
                            .filter(|&&target_score| target_score < start_score)
                            .map(|target_score| start_score - target_score),
                    );
                }
            }
        }
        shortcuts
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

fn get_position(board: &Vec<Vec<Node>>, field: Field) -> Option<(usize, usize)> {
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j].field == field {
                return Some((i, j));
            }
        }
    }
    None
}

fn find_shortcuts_with_length(input: &String, length: usize) -> usize {
    let board: Vec<Vec<Node>> = input
        .lines()
        .map(str::trim)
        .map(|line| line.bytes().map(Node::from).collect())
        .collect();

    let mut board = Board::new(board);

    let (x, y) = get_position(&board.grid, Field::END).expect("START not found");

    board.bfs(x, y);

    let shortcuts = board.find_shortcuts(length);

    shortcuts.iter().filter(|&&x| x >= 100).count()
}

pub struct Puzzle20 {}

impl Aoc2024 for Puzzle20 {
    fn name(&self) -> String {
        "Day 20: Race Condition".to_string()
    }

    fn solve_a(&self, input: &String) -> String {
        find_shortcuts_with_length(input, 2).to_string()
    }

    fn solve_b(&self, input: &String) -> String {
        find_shortcuts_with_length(input, 20).to_string()
    }
}
