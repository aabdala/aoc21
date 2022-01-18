mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod utils;

fn main() {
    println!(
        "day 1:\n{}",
        day1::solution(&utils::read_input_file("day1"))
    );
    println!(
        "day 2:\n{}",
        day2::solution(&utils::read_input_file("day2"))
    );
    println!(
        "day 3:\n{}",
        day3::solution(&utils::read_input_file("day3"))
    );
    println!(
        "day 4:\n{}",
        day4::solution(&utils::read_input_file("day4"))
    );
    println!(
        "day 5:\n{}",
        day5::solution(&utils::read_input_file("day5"))
    );
    println!(
        "day 6:\n{}",
        day6::solution(&utils::read_input_file("day6"))
    );
    println!(
        "day 7:\n{}",
        day7::solution(&utils::read_input_file("day7"))
    );
    println!(
        "day 8:\n{}",
        day8::solution(&utils::read_input_file("day8"))
    );
    println!(
        "day 9:\n{}",
        day9::solution(&utils::read_input_file("day9"))
    );
    println!(
        "day 10:\n{}",
        day10::solution(&utils::read_input_file("day10"))
    );
    println!(
        "day 11:\n{}",
        day11::solution(&utils::read_input_file("day11"))
    );
    // println!(
    //     "day 12:\n{}",
    //     day12::solution(&utils::read_input_file("day12"))
    // );
    println!(
        "day 13:\n{}",
        day13::solution(&utils::read_input_file("day13"))
    );
}
