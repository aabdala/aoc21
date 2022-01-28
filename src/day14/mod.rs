mod part1;
mod part2;

use std::{collections::HashMap, ops::AddAssign};

use crate::utils;

pub fn solution(input: &str) -> String {
    utils::format_part_solutions(part1::solution(input), part2::solution(input))
}

fn replace_steps(input: &str, steps: u8) -> String {
    let mut lines = input.lines();
    let template = lines.next().expect("polymer template");
    let insert_rules: HashMap<(char, char), char> = lines
        .skip(1)
        .map(|line| {
            let parts = line.split_terminator(" -> ").collect::<Vec<_>>();
            let mut rule_chars = parts[0].chars();
            (
                (rule_chars.next().unwrap(), rule_chars.next().unwrap()),
                parts[1].chars().next().unwrap(),
            )
        })
        .collect();

    let mut pairs: HashMap<(char, char), usize> = template
        .chars()
        .into_iter()
        .fold(
            (HashMap::new(), '_'),
            |(mut acc, last): (HashMap<(char, char), usize>, char), each| {
                if last != '_' {
                    acc.entry((last, each)).or_insert(0).add_assign(1);
                }
                (acc, each)
            },
        )
        .0;
    let mut new_pairs: HashMap<(char, char), usize> = HashMap::new();

    for _ in 0..steps {
        for (pair @ (ch1, ch2), count) in pairs {
            if let Some(repl) = insert_rules.get(&pair) {
                new_pairs.entry((ch1, *repl)).or_insert(0).add_assign(count);
                new_pairs.entry((*repl, ch2)).or_insert(0).add_assign(count)
            } else {
                new_pairs.entry(pair).or_insert(count);
            }
        }
        pairs = new_pairs;
        new_pairs = HashMap::new();
    }

    let mut char_occurrences: HashMap<char, usize> =
        template.chars().take(1).map(|c| (c, 1)).collect();
    pairs
        .iter()
        .for_each(|((_, ch), count)| char_occurrences.entry(*ch).or_insert(0).add_assign(count));
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
        assert_eq!(result, "2432786807053");
    }
}
