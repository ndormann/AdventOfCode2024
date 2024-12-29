use crate::prelude::Aoc2024;
use std::fmt::{Debug, Formatter, Write};

#[derive(PartialEq, Eq, Clone, Copy)]
struct Field {
    pub todo: bool,
    pub sides: u8,
    pub color: u8,
}

impl Field {
    /// Bitmask constants for directions
    const UP: u8 = 1 << 0; // Bit 0
    const RIGHT: u8 = 1 << 1; // Bit 1
    const DOWN: u8 = 1 << 2; // Bit 2
    const LEFT: u8 = 1 << 3; // Bit 3

    const DIRECTIONS: [u8; 4] = [Self::UP, Self::LEFT, Self::RIGHT, Self::DOWN];

    /// Checks if a direction is already visited
    fn has_side(&self, direction: u8) -> bool {
        (self.sides & direction) != 0
    }

    /// Marks a direction as visited
    fn mark_side(&mut self, direction: u8) {
        self.sides |= direction;
    }

    fn new(color: &u8) -> Self {
        Self {
            todo: true,
            sides: 0,
            color: *color,
        }
    }

    fn get_direction(dir: u8) -> (i32, i32) {
        match dir {
            Self::UP => (0, -1),
            Self::DOWN => (0, 1),
            Self::LEFT => (-1, 0),
            Self::RIGHT => (1, 0),
            _ => unreachable!(),
        }
    }

    fn get_perpendicular(dir: u8) -> ((i32, i32), (i32, i32)) {
        match dir {
            Self::UP => ((-1, 0), (1, 0)),
            Self::DOWN => ((-1, 0), (1, 0)),
            Self::LEFT => ((0, -1), (0, 1)),
            Self::RIGHT => ((0, -1), (0, 1)),
            _ => unreachable!(),
        }
    }
}

impl From<&u8> for Field {
    fn from(value: &u8) -> Self {
        match value {
            c => Field::new(c),
        }
    }
}

impl Debug for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.todo {
            false => f.write_char('.'),
            true => f.write_char(self.color as char),
        }
    }
}

fn get_in_bounds(board: &mut Vec<Vec<Field>>, x: i32, y: i32) -> Option<&mut Field> {
    let size = board.len() as i32;
    if x < 0 || y < 0 || x >= size || y >= size {
        None
    } else {
        Some(&mut board[x as usize][y as usize])
    }
}

fn flood_fill(board: &mut Vec<Vec<Field>>, x: i32, y: i32) -> (usize, usize) {
    let mut todo: Vec<(i32, i32)> = Vec::new();
    let size = board.len() as i32;
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut area = 0;
    let ch;
    let start = board[x as usize][y as usize];
    if start.todo {
        ch = start.color;
    } else {
        return (0, 0);
    }
    let mut perimeter = 0;
    todo.push((x, y));
    while let Some((x, y)) = todo.pop() {
        let curr_field = board[x as usize][y as usize];
        if !curr_field.todo {
            continue;
        } else {
            board[x as usize][y as usize].todo = false;
        };
        area += 1;
        for (dx, dy) in directions {
            let next_x = x + dx;
            let next_y = y + dy;
            if next_x < 0 || next_y < 0 || next_x >= size || next_y >= size {
                perimeter += 1;
            } else {
                let next_field = board[next_x as usize][next_y as usize];
                if next_field.todo && next_field.color == ch {
                    todo.push((next_x, next_y));
                } else if next_field.color != ch {
                    perimeter += 1;
                }
            }
        }
    }
    (area, perimeter)
}

fn flood_fill_sides(board: &mut Vec<Vec<Field>>, x: i32, y: i32) -> (usize, usize) {
    let mut todo: Vec<(i32, i32)> = Vec::new();
    let ch;
    let start = board[x as usize][y as usize];
    if start.todo {
        ch = start.color;
    } else {
        return (0, 0);
    }
    let mut perimeter = 0;
    let mut area = 0;
    todo.push((x, y));
    while let Some((x, y)) = todo.pop() {
        let mut curr_field = board[x as usize][y as usize];
        if !curr_field.todo {
            continue;
        } else {
            curr_field.todo = false;
        };
        area += 1;
        for dir in Field::DIRECTIONS {
            let (dx, dy) = Field::get_direction(dir);
            let next_x = x + dx;
            let next_y = y + dy;
            if let Some(next) = get_in_bounds(board, next_x, next_y) {
                if next.todo && next.color == ch {
                    todo.push((next_x, next_y));
                } else if next.color != ch {
                    curr_field.mark_side(dir);
                    let ((dx_1, dy_1), (dx_2, dy_2)) = Field::get_perpendicular(dir);
                    let p_1_x = x + dx_1;
                    let p_1_y = y + dy_1;
                    let p_2_x = x + dx_2;
                    let p_2_y = y + dy_2;
                    let mut side_count: i32 = 0;
                    if let Some(p_1) = get_in_bounds(board, p_1_x, p_1_y) {
                        if p_1.color == ch && p_1.has_side(dir) {
                            side_count += 1;
                        }
                    }
                    if let Some(p_2) = get_in_bounds(board, p_2_x, p_2_y) {
                        if p_2.color == ch && p_2.has_side(dir) {
                            side_count += 1;
                        }
                    }
                    match side_count {
                        0 => perimeter += 1,
                        2 => perimeter -= 1,
                        _ => {}
                    }
                }
            } else {
                curr_field.mark_side(dir);
                let ((dx_1, dy_1), (dx_2, dy_2)) = Field::get_perpendicular(dir);
                let p_1_x = x + dx_1;
                let p_1_y = y + dy_1;
                let p_2_x = x + dx_2;
                let p_2_y = y + dy_2;
                let mut side_count: i32 = 0;
                if let Some(p_1) = get_in_bounds(board, p_1_x, p_1_y) {
                    if p_1.color == ch && p_1.has_side(dir) {
                        side_count += 1;
                    }
                }
                if let Some(p_2) = get_in_bounds(board, p_2_x, p_2_y) {
                    if p_2.color == ch && p_2.has_side(dir) {
                        side_count += 1;
                    }
                }
                match side_count {
                    0 => perimeter += 1,
                    2 => perimeter -= 1,
                    _ => {}
                }
            }
        }
        board[x as usize][y as usize] = curr_field;
    }
    (area, perimeter)
}

pub struct Puzzle12 {}

fn get_board(input: &String) -> Vec<Vec<Field>> {
    input
        .lines()
        .map(str::trim)
        .map(|line| line.as_bytes().iter().map(Field::from).collect())
        .collect()
}

impl Aoc2024 for Puzzle12 {
    fn name(&self) -> String {
        "Day 12: Garden Groups".to_string()
    }

    fn solve_a(&self, input: &String) -> String {
        let mut board = get_board(input);

        let mut cost = 0;
        while let Some((x, y)) = board.iter().enumerate().find_map(|(row_idx, row)| {
            row.iter()
                .position(|el| el.todo)
                .map(|col_idx| (row_idx as i32, col_idx as i32))
        }) {
            let (_area, _perimeter) = flood_fill(&mut board, x, y);
            cost += _area * _perimeter;
        }
        cost.to_string()
    }

    fn solve_b(&self, input: &String) -> String {
        let mut board = get_board(input);

        let mut cost = 0;
        while let Some((x, y)) = board.iter().enumerate().find_map(|(row_idx, row)| {
            row.iter()
                .position(|el| el.todo)
                .map(|col_idx| (row_idx as i32, col_idx as i32))
        }) {
            let (_area, _sides) = flood_fill_sides(&mut board, x, y);
            cost += _area * _sides;
        }
        cost.to_string()
    }
}
