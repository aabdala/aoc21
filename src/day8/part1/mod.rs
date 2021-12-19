pub fn solution(input: &str) -> String {
    input.split_terminator('\n')
        .map(|line| {
            let line_parts = line.split('|').collect::<Vec<_>>();
            (*line_parts.get(0).unwrap(), *line_parts.get(1).unwrap())
        });
    ":(".into()
}
