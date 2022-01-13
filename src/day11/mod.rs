mod part1;
mod part2;

use crate::utils;

pub fn solution(input: &str) -> String {
    utils::format_part_solutions(part1::solution(input), part2::solution(input))
}

fn parse_readings(input: &str) -> Vec<Vec<u8>> {
    input
        .split_ascii_whitespace()
        .map(|line| {
            line.chars()
                .into_iter()
                .map(|c| String::from(c).parse::<u8>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

const MAX_BRIGHT: u8 = 10;

fn step(readings: &mut Vec<Vec<u8>>) -> usize {
    let mut total = 0;
    for i in 0..readings.len() {
        for j in 0..readings[i].len() {
            let step_flashes = flash(readings, i as isize, j as isize);
            total += step_flashes;
        }
    }
    for line in readings.iter_mut() {
        for reading in line.iter_mut() {
            if *reading == MAX_BRIGHT {
                *reading = 0;
            }
        }
    }
    total
}

fn flash(readings: &mut Vec<Vec<u8>>, i: isize, j: isize) -> usize {
    if i < 0 || j < 0 {
        return 0;
    }
    let iu = i as usize;
    let ju = j as usize;
    let max_i = (readings.len() - 1) as isize;
    let max_j = (readings[0].len() - 1) as isize;
    if i > max_i || j > max_j {
        return 0;
    }
    if readings[iu][ju] == MAX_BRIGHT {
        return 0;
    }
    let mut total = 0;
    readings[iu][ju] += 1;
    if readings[iu][ju] == MAX_BRIGHT {
        total += 1;
        total += flash(readings, i, j - 1);
        total += flash(readings, i, j + 1);
        total += flash(readings, i - 1, j - 1);
        total += flash(readings, i - 1, j);
        total += flash(readings, i - 1, j + 1);
        total += flash(readings, i + 1, j - 1);
        total += flash(readings, i + 1, j);
        total += flash(readings, i + 1, j + 1);
    }
    total
}
