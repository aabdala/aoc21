mod part1;
mod part2;

use crate::utils;

pub fn solution(input: &str) -> String {
    utils::format_part_solutions(part1::solution(input), part2::solution(input))
}

enum Instruction {
  Up(i32),
  Down(i32),
  Forward(i32),
}

