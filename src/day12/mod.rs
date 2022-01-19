mod part1;
mod part2;

use crate::utils;

pub fn solution(input: &str) -> String {
    utils::format_part_solutions(part1::solution(input), part2::solution(input))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let result = part1::solution(&utils::read_input_file("day12"));
        assert_eq!(result, "4011");
    }

    #[test]
    fn part2() {
        let result = part2::solution(&utils::read_input_file("day12"));
        assert_eq!(result, "108035");
    }
}