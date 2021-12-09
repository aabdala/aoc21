use core::panic;

use crate::day2::Instruction;

pub fn solution(input: String) -> String {
    let position = input
        .split_terminator('\n')
        .map(|each| {
            let elems: Vec<&str> = each.split_ascii_whitespace().collect();
            let value = elems.get(1).unwrap().parse::<i32>().unwrap();
            match *elems.get(0).unwrap() {
                "up" => Instruction::Up(value),
                "down" => Instruction::Down(value),
                "forward" => Instruction::Forward(value),
                _ => panic!("unexpected command"),
            }
        })
        .fold((0, 0, 0), |position, inst| match inst {
            Instruction::Down(units) => (position.0 + units, position.1, position.2),
            Instruction::Up(units) => (position.0 - units, position.1, position.2),
            Instruction::Forward(units) => (
                position.0,
                position.1 + units,
                position.2 + position.0 * units,
            ),
        });
    format!("{}", position.1 * position.2)
}
