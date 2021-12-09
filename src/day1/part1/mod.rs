pub fn solution(input: String) -> String {
    let mut incs = 0;

    input
        .split_ascii_whitespace()
        .map(|elem| elem.parse::<i32>().unwrap())
        .fold(0, |last, curr| {
            if curr > last {
                incs += 1;
            }
            curr
        });
    format!("{}", incs)
}
