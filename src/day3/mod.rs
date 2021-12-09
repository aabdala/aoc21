mod part1;
mod part2;

use crate::utils;

pub fn solution(input: String) -> String {
    utils::format_part_solutions(part1::solution(input.clone()), part2::solution(input))
}

fn count_bits_by_position(input: String) -> Vec<i32> {
    input
        .split_ascii_whitespace()
        .map(|each| {
            each.chars()
                .map(|c| if c == '0' { -1 } else { 1 })
                .collect::<Vec<i8>>()
        })
        .fold(
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            |bc, linebits| {
                linebits
                    .into_iter()
                    .zip(bc)
                    .map(|(a, b)| b + a as i32)
                    .collect()
            },
        )
}

const BASE2: i32 = 2;
fn calc_decimal(binary: Vec<u8>) -> i32 {
    binary
        .into_iter()
        .rev()
        .fold((0, 0), |(i, decimal), bit| {
            (i + 1, decimal + (bit as i32) * BASE2.pow(i))
        })
        .1
}
