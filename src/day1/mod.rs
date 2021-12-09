use std::fs;
use crate::utils;

pub fn solution() {
    let contents = utils::read_input_file("day1");
    let mut incs = 0;
    // part 1:
    // contents.split_ascii_whitespace()
    //     .map(|elem| elem.parse::<i32>().unwrap())
    //     .fold(0, |last, curr| {
    //         if curr > last {
    //             incs = incs + 1;
    //         }
    //         return curr
    //     });

    // part 2
    let mut last3 = [-1, -1, -1];
    contents.split_ascii_whitespace()
        .map(|elem| elem.parse::<i32>().unwrap())
        .map(| each| {
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
            return result;
        }).fold(-1, |last, curr| {
            if last != -1 && curr > last {
                incs = incs + 1;
            }
            return curr
        });

    print!("{}", incs)
}

// (a, {b, [c), d}, e], f, g
// prev
// cur =   a   | a + b | a + b + c
// next =      | b     | b + c

// prev = a + b + c    | a + b + c | b + c + d
// cur  = b + c        | b + c + d | c + d + e
// next = c            | c + d     | d + e
