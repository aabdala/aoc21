pub fn solution(input: &str) -> String {
    let mut bowl = super::parse_fishbowl(input);
    (0..80).for_each(|_| bowl.tick());
    format!("{}", bowl.fish_count())
}
