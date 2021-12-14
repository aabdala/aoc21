use super::{Line, Oceanfloor};

pub fn solution(input: &str) -> String {
    let oceanfloor = &mut Oceanfloor::new();
    input
        .split_terminator('\n')
        // .filter(|s| !s.is_empty())
        .map(Line::from)
        .filter(|line| line.is_straight())
        .for_each(|l| oceanfloor.draw(l));

    format!("{}", oceanfloor.intersect_count())
}
