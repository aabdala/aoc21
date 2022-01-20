mod part1;
mod part2;

use std::{collections::HashMap, ops::AddAssign};

use crate::utils;

pub fn solution(input: &str) -> String {
    utils::format_part_solutions(part1::solution(input), part2::solution(input))
}

fn replace_steps(input: &str, steps: u8) -> String {
    let mut lines = input.lines();
    let template = lines.next().expect("poliymer template");
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

    let mut char_occurrences: HashMap<char, usize> = HashMap::new();
    let mut queue: Vec<(char, char, u8)> = Vec::new();
    let poly_chars = template.chars().collect::<Vec<_>>();
    poly_chars
        .iter()
        .for_each(|c| count_char(&mut char_occurrences, *c));
    for i in 0..(poly_chars.len() - 1) {
        queue.push((poly_chars[i], poly_chars[i + 1], 1));
        while !queue.is_empty() {
            let (ch1, ch2, step) = queue.pop().unwrap();
            let replacement = insert_rules.get(&(ch1, ch2));
            if let Some(repl_char) = replacement {
                count_char(&mut char_occurrences, *repl_char);
                if step < steps {
                    queue.push((ch1, *repl_char, step + 1));
                    queue.push((*repl_char, ch2, step + 1));
                }
            }
        }
    }

    let mut max = 0;
    let mut min = usize::MAX;

    for (_, count) in char_occurrences {
        max = count.max(max);
        min = count.min(min);
    }

    format!("{}", max - min)
}

fn count_char(occ_map: &mut HashMap<char, usize>, c: char) {
    occ_map.entry(c).or_insert(0).add_assign(1);
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
