use crate::prelude::Aoc2024;
use std::cmp::PartialEq;
use std::fmt::{Debug, Formatter, Write};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Field {
    FREE,
    PACKAGE, // Each bit represents a direction
    PackageLeft,
    PackageRight,
    BLOCKED,
    ROBOT,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    UP,
    RIGHT, // Each bit represents a direction
    DOWN,
    LEFT,
}

impl Direction {
    fn parse(value: u8) -> Option<Self> {
        match value {
            b'>' => Some(Direction::RIGHT),
            b'^' => Some(Direction::UP),
            b'<' => Some(Direction::LEFT),
            b'v' => Some(Direction::DOWN),
            _ => None,
        }
    }

    fn delta(&self) -> (i32, i32) {
        match self {
            Direction::UP => (-1, 0),   // UP
            Direction::RIGHT => (0, 1), // RIGHT
            Direction::DOWN => (1, 0),  // DOWN
            Direction::LEFT => (0, -1), // LEFT
        }
    }
}

impl From<u8> for Field {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Field::FREE,
            b'#' => Field::BLOCKED,
            b'O' => Field::PACKAGE, // Initially no directions visited
            b'@' => Field::ROBOT,
            _ => Field::FREE,
        }
    }
}

impl Debug for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Field::FREE => f.write_char('.'),
            Field::BLOCKED => f.write_char('#'),
            Field::PACKAGE => f.write_char('O'),
            Field::PackageLeft => f.write_char('['),
            Field::PackageRight => f.write_char(']'),
            Field::ROBOT => f.write_char('@'),
        }
    }
}

fn get_robot(board: &Vec<Vec<Field>>) -> Option<(i32, i32)> {
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] == Field::ROBOT {
                return Some((i as i32, j as i32));
            }
        }
    }
    None
}

fn walk_board(board: &Vec<Vec<Field>>, instructions: &Vec<Direction>) -> Vec<Vec<Field>> {
    let mut board = board.clone();
    let (mut x, mut y) = get_robot(&board).expect("GUARD not found");

    for instruction in instructions {
        let (dx, dy) = instruction.delta();
        let next_x = x + dx;
        let next_y = y + dy;
        if next_x < 0 || next_x >= board.len() as i32 || next_y < 0 || next_y >= board.len() as i32
        {
            continue;
        }
        let next_field = board[next_x as usize][next_y as usize];
        match next_field {
            Field::BLOCKED => continue,
            Field::PACKAGE => {
                let mut blocked = true;
                let mut next_x = next_x;
                let mut next_y = next_y;
                loop {
                    next_x += dx;
                    next_y += dy;
                    if next_x < 0
                        || next_x >= board.len() as i32
                        || next_y < 0
                        || next_y >= board.len() as i32
                    {
                        break;
                    }
                    let next_field = board[next_x as usize][next_y as usize];
                    match next_field {
                        Field::BLOCKED => break,
                        Field::PACKAGE => continue,
                        _ => {
                            blocked = false;
                            board[next_x as usize][next_y as usize] = Field::PACKAGE;
                            break;
                        }
                    }
                }
                if blocked {
                    continue;
                }
            }
            _ => {}
        };
        board[x as usize][y as usize] = Field::FREE;
        x = next_x;
        y = next_y;
        board[next_x as usize][next_y as usize] = Field::ROBOT;
    }
    board
}

fn convert_board(board: Vec<Vec<Field>>) -> Vec<Vec<Field>> {
    board
        .iter()
        .map(|row| {
            row.iter().fold(Vec::new(), |mut acc, field| {
                match field {
                    Field::BLOCKED => {
                        acc.push(Field::BLOCKED);
                        acc.push(Field::BLOCKED);
                    }
                    Field::ROBOT => {
                        acc.push(Field::ROBOT);
                        acc.push(Field::FREE);
                    }
                    Field::PACKAGE => {
                        acc.push(Field::PackageLeft);
                        acc.push(Field::PackageRight);
                    }
                    _ => {
                        acc.push(Field::FREE);
                        acc.push(Field::FREE);
                    }
                }
                acc
            })
        })
        .collect()
}

fn is_blocked(board: &Vec<Vec<Field>>, x: i32, y: i32, direction: &Direction) -> bool {
    let (dx, dy) = direction.delta();
    let next_x = x + dx;
    let next_y = y + dy;
    if next_x < 0 || next_x >= board.len() as i32 || next_y < 0 || next_y >= board[0].len() as i32 {
        return true;
    }
    let next_field = board[next_x as usize][next_y as usize];
    match direction {
        Direction::LEFT | Direction::RIGHT => match next_field {
            Field::BLOCKED => true,
            Field::PACKAGE => is_blocked(board, next_x, next_y, direction),
            Field::PackageLeft | Field::PackageRight => {
                is_blocked(board, next_x, next_y, direction)
            }
            _ => false,
        },
        Direction::DOWN | Direction::UP => match next_field {
            Field::BLOCKED => true,
            Field::PACKAGE => is_blocked(board, next_x, next_y, direction),
            Field::PackageLeft => {
                is_blocked(board, next_x, next_y, direction)
                    || is_blocked(board, next_x, next_y + 1, direction)
            }
            Field::PackageRight => {
                is_blocked(board, next_x, next_y, direction)
                    || is_blocked(board, next_x, next_y - 1, direction)
            }
            _ => false,
        },
    }
}

fn move_packages(board: &mut Vec<Vec<Field>>, x: i32, y: i32, direction: &Direction) {
    let (dx, dy) = direction.delta();
    let next_x = x + dx;
    let next_y = y + dy;
    let field = board[x as usize][y as usize];
    match field {
        Field::FREE => return,
        _ => (),
    };
    match direction {
        Direction::LEFT => {
            move_packages(board, next_x + dx, next_y + dy, direction);
            board[(next_x + dx) as usize][(next_y + dy) as usize] = Field::PackageLeft;
            board[next_x as usize][next_y as usize] = Field::PackageRight;
        }
        Direction::RIGHT => {
            move_packages(board, next_x + dx, next_y + dy, direction);
            board[(next_x + dx) as usize][(next_y + dy) as usize] = Field::PackageRight;
            board[next_x as usize][next_y as usize] = Field::PackageLeft;
        }
        Direction::DOWN | Direction::UP => {
            if field == Field::PackageLeft {
                move_packages(board, next_x, next_y, direction);
                move_packages(board, next_x, next_y + 1, direction);
                board[x as usize][y as usize] = Field::FREE;
                board[x as usize][(y + 1) as usize] = Field::FREE;
                board[next_x as usize][next_y as usize] = Field::PackageLeft;
                board[next_x as usize][(next_y + 1) as usize] = Field::PackageRight;
            } else {
                move_packages(board, next_x, next_y, direction);
                move_packages(board, next_x, next_y - 1, direction);
                board[x as usize][y as usize] = Field::FREE;
                board[x as usize][(y - 1) as usize] = Field::FREE;
                board[next_x as usize][(next_y - 1) as usize] = Field::PackageLeft;
                board[next_x as usize][next_y as usize] = Field::PackageRight;
            }
        }
    }
}

fn walk_double_board(board: &Vec<Vec<Field>>, instructions: &Vec<Direction>) -> Vec<Vec<Field>> {
    let mut board = board.clone();
    let (mut x, mut y) = get_robot(&board).expect("GUARD not found");

    for instruction in instructions {
        let (dx, dy) = instruction.delta();
        let next_x = x + dx;
        let next_y = y + dy;
        if next_x < 0
            || next_x >= board.len() as i32
            || next_y < 0
            || next_y >= board[0].len() as i32
        {
            continue;
        }
        let next_field = board[next_x as usize][next_y as usize];
        match next_field {
            Field::BLOCKED => continue,
            Field::PACKAGE => {
                if is_blocked(&board, next_x, next_y, instruction) {
                    continue;
                }
            }
            Field::PackageLeft => {
                if is_blocked(&board, next_x, next_y, instruction)
                    || is_blocked(&board, next_x, next_y + 1, instruction)
                {
                    continue;
                }
            }
            Field::PackageRight => {
                if is_blocked(&board, next_x, next_y, instruction)
                    || is_blocked(&board, next_x, next_y - 1, instruction)
                {
                    continue;
                }
            }
            _ => {}
        };
        move_packages(&mut board, next_x, next_y, instruction);
        board[x as usize][y as usize] = Field::FREE;
        board[next_x as usize][next_y as usize] = Field::ROBOT;
        x = next_x;
        y = next_y;
    }
    board
}

fn score(board: &Vec<Vec<Field>>) -> usize {
    let mut gps_sum = 0;
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] == Field::PACKAGE {
                gps_sum += i * 100 + j
            } else if board[i][j] == Field::PackageLeft {
                gps_sum += i * 100 + j
            }
        }
    }
    gps_sum
}

pub struct Puzzle15 {}

fn parse_input(input: &String) -> (Vec<Vec<Field>>, Vec<Direction>) {
    let mut parts = input.split("\n\n");
    let board = parts.next().unwrap();

    let board: Vec<Vec<Field>> = board
        .lines()
        .map(str::trim)
        .map(|line| line.bytes().map(Field::from).collect())
        .collect();

    let instructions: Vec<Direction> = parts
        .next()
        .unwrap()
        .bytes()
        .filter_map(Direction::parse)
        .collect();
    (board, instructions)
}

impl Aoc2024 for Puzzle15 {
    fn name(&self) -> String {
        "Day 15: Warehouse Woes".to_string()
    }

    fn solve_a(&self, input: &String) -> String {
        let (board, instructions) = parse_input(input);
        let board = walk_board(&board, &instructions);
        score(&board).to_string()
    }

    fn solve_b(&self, input: &String) -> String {
        let (board, instructions) = parse_input(input);
        let board = convert_board(board);
        let board = walk_double_board(&board, &instructions);
        score(&board).to_string()
    }
}
