use crate::prelude::Aoc2024;
use std::cmp::PartialEq;
use std::fmt::{Debug, Formatter, Write};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Field {
    FREE,
    VISITED(u8), // Each bit represents a direction
    BLOCKED,
    GUARD,
}

impl Field {
    /// Bitmask constants for directions
    const UP: u8 = 1 << 0; // Bit 0
    const RIGHT: u8 = 1 << 1; // Bit 1
    const DOWN: u8 = 1 << 2; // Bit 2
    const LEFT: u8 = 1 << 3; // Bit 3

    /// Checks if a direction is already visited
    fn has_visited(&self, direction: u8) -> bool {
        if let Field::VISITED(bits) = self {
            (bits & direction) != 0
        } else {
            false
        }
    }

    /// Marks a direction as visited
    fn mark_visited(&mut self, direction: u8) {
        if let Field::VISITED(bits) = self {
            *bits |= direction;
        } else {
            *self = Field::VISITED(direction);
        }
    }
}

impl From<&u8> for Field {
    fn from(value: &u8) -> Self {
        match value {
            b'.' => Field::FREE,
            b'#' => Field::BLOCKED,
            b'X' => Field::VISITED(0), // Initially no directions visited
            b'^' => Field::GUARD,
            _ => Field::FREE,
        }
    }
}

impl Debug for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Field::FREE => f.write_char('.'),
            Field::BLOCKED => f.write_char('#'),
            Field::VISITED(_) => f.write_char('X'),
            Field::GUARD => f.write_char('^'),
        }
    }
}

enum WalkingResult {
    EXITED,
    LOOP,
}

fn walk_board(board: &mut Vec<Vec<Field>>, mut x: i32, mut y: i32) -> WalkingResult {
    let mut direction = 0;
    let directions = [Field::UP, Field::RIGHT, Field::DOWN, Field::LEFT];
    loop {
        let current_cell = &mut board[x as usize][y as usize];
        if current_cell.has_visited(directions[direction]) {
            return WalkingResult::LOOP;
        }
        current_cell.mark_visited(directions[direction]);

        let (dx, dy) = match direction {
            0 => (-1, 0), // UP
            1 => (0, 1),  // RIGHT
            2 => (1, 0),  // DOWN
            3 => (0, -1), // LEFT
            _ => unreachable!(),
        };
        let next_x = x + dx;
        let next_y = y + dy;
        if next_x < 0 || next_x >= board.len() as i32 || next_y < 0 || next_y >= board.len() as i32
        {
            return WalkingResult::EXITED;
        }
        if board[next_x as usize][next_y as usize] == Field::BLOCKED {
            direction = (direction + 1) % 4;
        } else {
            x = next_x;
            y = next_y;
        }
    }
}

pub struct Puzzle6 {}

impl Puzzle6 {
    fn get_board(input: &String) -> Vec<Vec<Field>> {
        input
            .lines()
            .map(str::trim)
            .map(|line| line.as_bytes().iter().map(Field::from).collect())
            .collect()
    }

    fn get_guard(board: &Vec<Vec<Field>>) -> (i32, i32) {
        board
            .iter()
            .enumerate()
            .find_map(|(row_idx, row)| {
                row.iter()
                    .position(|el| *el == Field::GUARD)
                    .map(|col_idx| (row_idx as i32, col_idx as i32))
            })
            .expect("GUARD not found")
    }
}

impl Aoc2024 for Puzzle6 {
    fn name(&self) -> String {
        "Day 6: Guard Gallivant".to_string()
    }

    fn solve_a(&self, input: &String) -> String {
        let board = Self::get_board(input);
        let (x, y) = Self::get_guard(&board);

        let mut a_board = board.clone();
        walk_board(&mut a_board, x, y);
        let visited: usize = a_board
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|el| matches!(el, Field::VISITED(_)))
                    .count()
            })
            .sum();
        visited.to_string()
    }

    fn solve_b(&self, input: &String) -> String {
        let board = Self::get_board(input);
        let (x, y) = Self::get_guard(&board);

        let mut loops = 0;
        for i in 0..board.len() {
            for j in 0..board.len() {
                if board[i][j] == Field::FREE {
                    // Copy the contents of `board` into `test_board`
                    let mut test_board = board.clone();
                    test_board[i][j] = Field::BLOCKED;
                    if let WalkingResult::LOOP = walk_board(&mut test_board, x, y) {
                        loops += 1;
                    }
                }
            }
        }
        loops.to_string()
    }
}
