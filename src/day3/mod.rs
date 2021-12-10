mod part1;
mod part2;

use crate::utils;

pub fn solution(input: String) -> String {
    utils::format_part_solutions(part1::solution(input.clone()), part2::solution(input))
}

fn count_bits_by_position_str(input: String) -> Vec<i32> {
    let bits = to_counting_bits(to_bits(input));
    count_bits_by_position(bits)
}

fn to_counting_bits(bits: Vec<Vec<i8>>) -> Vec<Vec<i8>> {
    bits.into_iter()
        .map(|bs| {
            bs.into_iter()
                .map(|b| if b == 0 { -1 } else { 1 })
                .collect()
        })
        .collect()
}

fn to_bits(input: String) -> Vec<Vec<i8>> {
    let bits: Vec<Vec<i8>> = input
        .split_ascii_whitespace()
        .map(|each| {
            each.chars()
                .map(|c| if c == '0' { 0 } else { 1 })
                .collect::<Vec<i8>>()
        })
        .collect();
    bits
}

fn count_bits_by_position(bits: Vec<Vec<i8>>) -> Vec<i32> {
    bits.into_iter().fold(
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
fn calc_decimal(binary: Vec<i8>) -> i32 {
    binary
        .into_iter()
        .rev()
        .fold((0, 0), |(i, decimal), bit| {
            (i + 1, decimal + (bit as i32) * BASE2.pow(i))
        })
        .1
}
