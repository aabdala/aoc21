mod part1;
mod part2;

use std::collections::VecDeque;

use crate::utils;

pub fn solution(input: &str) -> String {
    utils::format_part_solutions(part1::solution(input), part2::solution(input))
}

fn is_opening_char(c: char) -> bool {
    matches!(c, '<' | '[' | '(' | '{')
}

fn to_closing(opening: char) -> char {
    match opening {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("unexpected opening char"),
    }
}

fn validate(line: &str) -> (Option<char>, Option<VecDeque<char>>) {
    let mut opening_chars: VecDeque<char> = VecDeque::new();
    for c in line.trim().chars() {
        if is_opening_char(c) {
            opening_chars.push_back(c);
        } else {
            match opening_chars.pop_back() {
                Some(last_opening) => {
                    if to_closing(last_opening) != c {
                        return (Some(c), None);
                    }
                }
                None => return (Some(c), None),
            }
        }
    }

    if !opening_chars.is_empty() {
        return (None, Some(opening_chars))
    }

    (None, None)
}
