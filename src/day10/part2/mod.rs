use core::panic;
use std::collections::VecDeque;

pub fn solution(input: &str) -> String {
    let mut scores = input
        .split_ascii_whitespace()
        .map(|line| line_completion_score(line))
        .filter(|score| *score > 0)
        .collect::<Vec<_>>();

    scores.sort_unstable();
    format!("{}", scores[scores.len() / 2])
}

fn line_completion_score(line: &str) -> usize {
    match super::validate(line) {
        (None, Some(chars)) => completion_score(chars),
        _ => 0,
    }
}

fn completion_score(opening_chars: VecDeque<char>) -> usize {
    opening_chars
        .iter()
        .rev()
        .fold(0, |acc, c| (acc * 5) + char_syntax_score(*c))
}

fn char_syntax_score(c: char) -> usize {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!("unexpected char"),
    }
}
