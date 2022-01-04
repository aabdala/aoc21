pub fn solution(input: &str) -> String {
    let floormap: Vec<Vec<u8>> = input
        .split_ascii_whitespace()
        .map(|line| {
            line.chars()
                .map(|c| String::from(c).parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect();

    let low_points = super::get_low_points(&floormap);
    format!(
        "{}",
        low_points
            .iter()
            .map(|(_, _, d)| usize::from(d + 1))
            .sum::<usize>()
    )
}
