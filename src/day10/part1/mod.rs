use core::panic;

pub fn solution(input: &str) -> String {
    format!(
        "{}",
        input
            .split_ascii_whitespace()
            .map(|line| line_error_score(line))
            .sum::<usize>()
    )
}

fn line_error_score(line: &str) -> usize {
    match super::validate(line) {
        (Some(char), None) => char_error_score(char),
        _ => 0
    }
}

fn char_error_score(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("unexpected closing char"),
    }
}
