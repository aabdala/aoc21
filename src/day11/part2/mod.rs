pub fn solution(input: &str) -> String {
    let mut readings = super::parse_readings(input);

    let readings_size = readings.len() * readings[0].len();
    for i in 1.. {
        if super::step(&mut readings) == readings_size {
            return format!("{}", i);
        }
    }
    ":(".into()
}
