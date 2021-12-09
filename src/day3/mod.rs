mod part1;
mod part2;

use crate::utils;

pub fn solution() -> String {
    utils::format_part_solutions(part1::solution(), part2::solution())
}
