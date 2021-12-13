pub fn solution(input: &str) -> String {
    let mut incs = 0;

    let mut last3 = [-1, -1, -1];
    input.split_ascii_whitespace()
        .map(|elem| elem.parse::<i32>().unwrap())
        .map(| each| {
            #[allow(clippy::needless_range_loop)]
            for n in 0..1 {
                if last3[n] == -1 {
                    last3[n] = each;
                    return -1;
                }
            }
            last3[2] = each;
            let result = last3.iter().sum();
            last3[0] = last3[1];
            last3[1] = last3[2];
            result
        }).fold(-1, |last, curr| {
            if last != -1 && curr > last {
                incs += 1;
            }
            curr
        });

    format!("{}", incs)
}
