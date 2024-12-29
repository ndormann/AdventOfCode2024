use crate::prelude::Aoc2024;

pub struct Puzzle9 {}

fn get_free_spaces(input: &String) -> (Vec<(usize, usize)>, usize) {
    let mut pos = 0;
    let free_spaces = input
        .bytes()
        .enumerate()
        .filter_map(|(i, el)| {
            let length = (el - b'0') as usize;
            if i % 2 == 0 {
                pos += length;
                None
            } else if length > 0 {
                let res = Some((pos, length));
                pos += length;
                res
            } else {
                None
            }
        })
        .collect();
    (free_spaces, pos)
}
impl Aoc2024 for Puzzle9 {
    fn name(&self) -> String {
        "Day 9: Disk Fragmenter".to_string()
    }

    fn solve_a(&self, input: &String) -> String {
        let (mut free_spaces, mut pos) = get_free_spaces(input);
        free_spaces.reverse();

        let mut result = 0;
        for (i, el) in input.bytes().enumerate().rev() {
            let id = i / 2;
            let length = (el - b'0') as usize;
            if i % 2 != 0 {
                pos -= length;
                continue;
            }
            for _ in 0..length {
                pos -= 1;
                if let Some((start, len)) = free_spaces.last_mut() {
                    if *start > pos {
                        free_spaces.clear();
                        result += pos * id;
                    } else {
                        result += *start * id;
                        *start += 1;
                        *len -= 1;
                        if *len == 0 {
                            free_spaces.pop();
                        }
                    }
                } else {
                    result += pos * id;
                }
            }
        }
        result.to_string()
    }

    fn solve_b(&self, input: &String) -> String {
        let (mut free_spaces, mut pos) = get_free_spaces(input);
        free_spaces.reverse();

        let cost = |id: usize, start: usize, len: usize| id * (start * len + (len * (len - 1)) / 2);

        let mut result = 0;
        for (i, el) in input.bytes().enumerate().rev() {
            let id = i / 2;
            let length = (el - b'0') as usize;
            pos -= length;
            if i % 2 != 0 {
                continue; // skip free spaces
            }
            if let Some((start, len)) = free_spaces
                .iter_mut()
                .filter(|(_, len)| *len >= length)
                .next()
            {
                if *start < pos {
                    result += cost(id, *start, length);
                    *start += length;
                    *len -= length;
                } else {
                    result += cost(id, pos, length);
                }
            } else {
                result += cost(id, pos, length);
            }
        }
        result.to_string()
    }
}
