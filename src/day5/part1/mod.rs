use lazy_static::lazy_static;
use regex::{Captures, Regex};

use std::{collections::{HashMap}, ops::RangeInclusive};

pub fn solution(input: &str) -> String {
    let oceanfloor = &mut Oceanfloor::new();
    input
        .split_terminator('\n')
        .filter(|s| !s.is_empty())
        .map(|line_str| Line::from(line_str))
        .filter(|line| line.is_straight())
        .for_each(|l| oceanfloor.draw(l));

    format!("{}", oceanfloor.intersect_count())
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
        line.x_range().for_each(|x_position| {
            let y_positions = self.chart.entry(x_position).or_insert_with(HashMap::new);
            line.y_range().for_each(|y_position| {
                let counter = y_positions.entry(y_position).or_insert(0);
                *counter += 1;
            })
        })
    }

    fn intersect_count(&self) -> usize {
        self.chart.iter().flat_map(|(_, v)| v.values().collect::<Vec<&i32>>()).filter(|x| **x > 1).count()
    }
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32
}

impl Position {
    fn x_range(&self, other: &Position) -> RangeInclusive<i32> {
        if self.x < other.x {
            self.x..=other.x
        } else {
            other.x..=self.x
        }
    }

    fn y_range(&self, other: &Position) -> RangeInclusive<i32> {
        if self.y < other.y {
            self.y..=other.y
        } else {
            other.y..=self.y
        }
    }
}

#[derive(Debug)]
struct Line {
    start: Position,
    end: Position,
}

impl Line {
    fn number_at(line_captures: &Captures, i : usize ) -> i32 {
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
            end: Position{x: x2, y: y2},
        }
    }

    fn x_range(&self) -> RangeInclusive<i32> {
        self.start.x_range(&self.end)
    }

    fn y_range(&self) -> RangeInclusive<i32> {
        self.start.y_range(&self.end)
    }

    fn is_straight(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }
}
