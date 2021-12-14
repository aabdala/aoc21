mod part1;
mod part2;

use crate::utils;
use std::{convert::TryInto, fmt::Debug};

pub fn solution(input: &str) -> String {
    utils::format_part_solutions(part1::solution(input), part2::solution(input))
}

fn parse_bingo(input: &str) -> (Vec<i32>, Vec<Board>) {
    let mut lines = input.split_terminator('\n');
    let numbers: Vec<i32> = lines
        .find(|_| true)
        .unwrap()
        .split_terminator(',')
        .map(|each| each.parse::<i32>().unwrap())
        .collect();

    let boards: Vec<Board> = lines
        .skip(1)
        .fold(vec![String::new()], |mut board_strs, each| {
            if each.is_empty() {
                // build board
                board_strs.push(String::new());
            } else {
                let len = board_strs.len();
                let last_board = &board_strs[len - 1];
                board_strs[len - 1] = last_board.to_owned() + "\n" + each;
            }
            board_strs
        })
        .iter()
        .map(|each| Board::from(each))
        .collect();
    (numbers, boards)
}
#[derive(Debug)]
struct Board {
    won: bool,
    contents: [[(i32, bool); 5]; 5],
}

impl Board {
    fn from(contents: &str) -> Self {
        let contents: Vec<[(i32, bool); 5]> = contents
            .split_terminator('\n')
            .filter(|each| !each.is_empty())
            .map(|line| {
                to_5_array(
                    line.split_ascii_whitespace()
                        .map(|num_str| (num_str.parse::<i32>().unwrap(), false))
                        .collect(),
                )
            })
            .collect();
        Self {
            won: false,
            contents: to_5_array(contents),
        }
    }

    fn result(&self) -> i32 {
        let mut result = 0;
        for i in 0..5 {
            for j in 0..5 {
                if !self.contents[i][j].1 {
                    result += self.contents[i][j].0;
                }
            }
        }
        result
    }

    fn play(&mut self, number: i32) -> bool {
        if self.won {
            return false;
        }
        for i in 0..5 {
            for j in 0..5 {
                if self.contents[i][j].0 == number {
                    self.contents[i][j].1 = true;
                    if self.won_at(i, j) {
                        self.won = true;
                        return true;
                    }
                }
            }
        }
        false
    }

    fn won_at(&self, i: usize, j: usize) -> bool {
        let marked: fn(&(i32, bool)) -> bool = |(_, marked)| *marked;
        self.contents[i].iter().all(marked)
            || (0..5).map(|_i| &self.contents[_i][j]).all(marked)
    }
}

fn to_5_array<T>(v: Vec<T>) -> [T; 5]
where
    T: Debug,
{
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", 5, v.len()))
}
