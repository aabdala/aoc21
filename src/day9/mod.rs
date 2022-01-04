mod part1;
mod part2;

use crate::utils;

pub fn solution(input: &str) -> String {
    utils::format_part_solutions(part1::solution(input), part2::solution(input))
}


fn get_low_points(floormap: &[Vec<u8>]) -> Vec<(usize, usize, u8)> {
    let mut low_points: Vec<(usize, usize, u8)> = vec![];
    for (i, line) in floormap.iter().enumerate() {
        for (j, depth) in line.iter().enumerate() {
            let mut low_point = true;
            if i != 0 {
                // look up
                let depth_up = floormap.get(i - 1).unwrap().get(j).unwrap();
                if depth >= depth_up {
                    low_point = false;
                }
            }
            if i < floormap.len() - 1 {
                // look down
                let depth_down = floormap.get(i + 1).unwrap().get(j).unwrap();
                if depth >= depth_down {
                    low_point = false;
                }
            }
            if j != 0 {
                // look left
                let depth_left = floormap.get(i).unwrap().get(j - 1).unwrap();
                if depth >= depth_left {
                    low_point = false;
                }
            }
            if j != line.len() - 1 {
                // look right
                let depth_right = floormap.get(i).unwrap().get(j + 1).unwrap();
                if depth >= depth_right {
                    low_point = false;
                }
            }
            if low_point {
                low_points.push((i, j, *depth));
            }
        }
    }
    low_points
}
