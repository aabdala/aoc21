pub fn solution(input: &str) -> String {
    let (numbers, mut boards) = super::parse_bingo(input);

    let mut winner_move: Option<(i32, i32)> = None;
    for number in numbers {
        if let Some((winner, _)) = boards
            .iter_mut()
            .map(|b| {
                let won = b.play(number);
                (b, won)
            })
            .filter(|(_, won)| *won)
            .last()
        {
            winner_move = Some((number, winner.result()));
        }
    }

    if let Some((number, winner_score)) = winner_move {
        format!("{}", number * winner_score)
    } else {
        ":(".into()
    }
}
