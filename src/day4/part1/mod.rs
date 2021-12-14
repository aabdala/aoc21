pub fn solution(input: &str) -> String {
    let (numbers, mut boards) = super::parse_bingo(input);

    for number in numbers {
        if let Some((winner, _)) = boards
            .iter_mut()
            .map(|b| {
                let won = b.play(number);
                (b, won)
            })
            .find(|(_, won)| *won)
        {
            return format!("{}", winner.result() * number);
        }
    }
    String::from(":(")
}
