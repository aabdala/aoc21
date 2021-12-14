use super::Board;

pub fn solution(input: &str) -> String {
    let (numbers, mut boards) = super::parse_bingo(input);

    let mut last_winner: Option<&Board> = Option::None;
    for number in numbers {
        if let Some((winner, _)) = boards
            .iter_mut()
            .map(|b| {
                let won = b.play(number);
                (b, won)
            })
            .rev()
            .find(|(_, won)| *won)
        {
            last_winner = Option::Some(winner);
        }
    }

    if let Some(winner) = last_winner {
        format!("{}", winner.result())
    } else {
        ":(".into()
    }
}
