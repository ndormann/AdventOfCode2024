use crate::prelude::Aoc2024;
use std::collections::HashMap;
pub struct Puzzle4 {}

impl Aoc2024 for Puzzle4 {
    fn name(&self) -> String {
        "Day 4: Ceres Search".to_string()
    }

    fn solve_a(&self, input: &String) -> String {
        let grid: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
        let (count, _) = count_word_search(&grid, "XMAS");
        count.to_string()
    }

    fn solve_b(&self, input: &String) -> String {
        let grid: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
        let (_, cross_count) = count_word_search(&grid, "MAS");
        cross_count.to_string()
    }
}

fn count_word_search(grid: &Vec<Vec<u8>>, word: &str) -> (usize, usize) {
    let mut out_grid = grid.clone();

    let mut counts_0: HashMap<(i32, i32), usize> = HashMap::new();
    let mut counts_45: HashMap<(i32, i32), usize> = HashMap::new();

    let directions_0 = [
        (1, 0),  // Horizontal right
        (-1, 0), // Horizontal left
        (0, 1),  // Vertical down
        (0, -1), // Vertical up
    ];
    let directions_45 = [
        (1, 1),   // Diagonal down-right
        (-1, -1), // Diagonal up-left
        (1, -1),  // Diagonal down-left
        (-1, 1),  // Diagonal up-right
    ];
    let word_chars: Vec<u8> = word.as_bytes().to_vec();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut counts = 0;

    for x in 0..rows {
        for y in 0..cols {
            for &(dx, dy) in &directions_0 {
                if let Some(middle) = check_word(grid, x as i32, y as i32, dx, dy, &word_chars) {
                    *counts_0.entry(middle).or_insert(0) += 1;
                    counts += 1;
                }
            }
            for &(dx, dy) in &directions_45 {
                if let Some(middle) = check_word(grid, x as i32, y as i32, dx, dy, &word_chars) {
                    *counts_45.entry(middle).or_insert(0) += 1;
                    counts += 1;
                }
            }
        }
    }

    let valid_middles: Vec<(i32, i32)> = counts_45
        .iter()
        .filter(|(_, &middle_count)| middle_count == 2)
        .map(|(&middle, _)| middle)
        .collect();
    for (a, b) in valid_middles {
        out_grid[a as usize][b as usize] = b'0';
    }
    (
        counts,
        counts_45
            .iter()
            .filter(|(_, &middle_count)| middle_count > 1)
            .count(),
    )
}

fn check_word(
    grid: &Vec<Vec<u8>>,
    mut x: i32,
    mut y: i32,
    dx: i32,
    dy: i32,
    word_chars: &[u8],
) -> Option<(i32, i32)> {
    let res = Some((x + dx, y + dy));
    for &ch in word_chars {
        if x < 0 || y < 0 || x >= grid.len() as i32 || y >= grid[0].len() as i32 {
            return None;
        }
        if grid[x as usize][y as usize] != ch {
            return None;
        }
        x += dx;
        y += dy;
    }
    res
}
