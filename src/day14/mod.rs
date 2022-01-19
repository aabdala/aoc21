mod part1;
mod part2;

use std::{collections::HashMap, ops::AddAssign};

use crate::utils;

pub fn solution(input: &str) -> String {
    utils::format_part_solutions(part1::solution(input), part2::solution(input))
}

fn replace_steps(input: &str, steps: usize) -> String {
    let mut lines = input.lines();
    let template = lines.next().expect("poliymer template");
    let insert_rules: HashMap<&str, &str> = lines
        .skip(1)
        .map(|line| {
            let parts = line.split_terminator(" -> ").collect::<Vec<_>>();
            (parts[0], parts[1])
        })
        .collect();
    let mut polymer: String = template.into();
    for _ in 0..steps {
        let poly_chars = polymer.chars().collect::<Vec<_>>();
        let mut expanded: String = poly_chars[0].into();
        for i in 0..(poly_chars.len() - 1) {
            let pair = format!("{}{}", poly_chars[i], poly_chars[i + 1]);
            let replacement = insert_rules
                .get(pair.as_str())
                .map_or(poly_chars[i + 1].into(), |insert| {
                    format!("{}{}", insert, poly_chars[i + 1])
                });
            replacement.chars().for_each(|ch| expanded.push(ch));
        }
        polymer = expanded;
    }
    let char_occurrences: HashMap<char, usize> =
        polymer.chars().fold(HashMap::new(), |mut map, each| {
            map.entry(each).or_default().add_assign(1);
            map
        });
    let mut max = 0;
    let mut min = usize::MAX;

    for (_, count) in char_occurrences {
        max = count.max(max);
        min = count.min(min);
    }

    format!("{}", max - min)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let result = part1::solution(&utils::read_example_file("day14"));
        assert_eq!(result, "1588");
    }

    #[test]
    fn part1() {
        let result = part1::solution(&utils::read_input_file("day14"));
        assert_eq!(result, "2345");
    }

    #[test]
    fn part2_example() {
        let result = part2::solution(&utils::read_example_file("day14"));
        assert_eq!(result, "2188189693529");
    }

    #[test]
    fn part2() {
        let result = part2::solution(&utils::read_input_file("day14"));
        assert_eq!(result, "");
    }
}
