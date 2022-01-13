pub fn solution(input: &str) -> String {
    let mut readings = super::parse_readings(input);

    let mut flashes: usize = 0;
    for _ in 1..=100 {
        flashes += super::step(&mut readings);
    }
    format!("{}", flashes)
}
