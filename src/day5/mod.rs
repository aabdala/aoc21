mod part1;
mod part2;

use crate::utils;

use lazy_static::lazy_static;
use regex::{Captures, Regex};

use std::collections::HashMap;

pub fn solution(input: &str) -> String {
    utils::format_part_solutions(part1::solution(input), part2::solution(input))
}

struct Oceanfloor {
    chart: HashMap<i32, HashMap<i32, i32>>, // map[x-positions]<map[y-positions]>
}

impl Oceanfloor {
    fn new() -> Self {
        Oceanfloor {
            chart: HashMap::new(),
        }
    }

    fn draw(&mut self, line: Line) {
        let mut x_range = line.x_range();
        let mut y_range = line.y_range();
        Self::grow_to_match(&mut x_range, y_range.len());
        Self::grow_to_match(&mut y_range, x_range.len());
        x_range
            .iter()
            .zip(y_range.iter())
            .for_each(|(x, y)| self.upsert_position(x, y))
    }

    fn upsert_position(&mut self, x: &i32, y: &i32) {
        let y_positions = self.chart.entry(*x).or_insert_with(HashMap::new);
        let counter = y_positions.entry(*y).or_insert(0);
        *counter += 1;
    }

    fn grow_to_match(vec: &mut Vec<i32>, len: usize) {
        if vec.len() < len {
            let first_elem = vec.get(0).unwrap().to_owned();
            vec.resize_with(len, || first_elem);
        }
    }

    fn intersect_count(&self) -> usize {
        self.chart
            .iter()
            .flat_map(|(_, v)| v.values().collect::<Vec<&i32>>())
            .filter(|x| **x > 1)
            .count()
    }
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn collect(a: i32, b: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut i = a;
        if a < b {
            while i <= b {
                result.push(i);
                i += 1;
            }
        } else {
            while i >= b {
                result.push(i);
                i -= 1;
            }
        }
        result
    }

    fn x_range(&self, other: &Position) -> Vec<i32> {
        Self::collect(self.x, other.x)
    }

    fn y_range(&self, other: &Position) -> Vec<i32> {
        Self::collect(self.y, other.y)
    }
}

#[derive(Debug)]
struct Line {
    start: Position,
    end: Position,
}

impl Line {
    fn number_at(line_captures: &Captures, i: usize) -> i32 {
        line_captures.get(i).unwrap().as_str().parse().unwrap()
    }

    fn from(input: &str) -> Self {
        lazy_static! {
            static ref LINE_REGEX: Regex = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
        }
        let line_captures = LINE_REGEX.captures(input).expect("invalid line format");
        let x1: i32 = Self::number_at(&line_captures, 1);
        let y1: i32 = Self::number_at(&line_captures, 2);
        let x2: i32 = Self::number_at(&line_captures, 3);
        let y2: i32 = Self::number_at(&line_captures, 4);

        Self {
            start: Position { x: x1, y: y1 },
            end: Position { x: x2, y: y2 },
        }
    }

    fn x_range(&self) -> Vec<i32> {
        self.start.x_range(&self.end)
    }

    fn y_range(&self) -> Vec<i32> {
        self.start.y_range(&self.end)
    }

    fn is_straight(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }

    fn is_diagonal(&self) -> bool {
        self.x_range().len() == self.y_range().len()
    }
}
