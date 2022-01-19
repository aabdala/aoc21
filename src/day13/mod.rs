mod part1;
mod part2;

use std::collections::HashSet;

use crate::utils;

pub fn solution(input: &str) -> String {
    utils::format_part_solutions(part1::solution(input), part2::solution(input))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let result = part1::solution(&utils::read_input_file("day13"));
        assert_eq!(result, "607");
    }

    #[test]
    fn part2() {
        let result = part2::solution(&utils::read_input_file("day13"));
        let expected_result = "
 ##  ###  #### #    ###  #### #### #   
#  # #  #    # #    #  # #       # #   
#    #  #   #  #    #  # ###    #  #   
#    ###   #   #    ###  #     #   #   
#  # #    #    #    #    #    #    #   
 ##  #    #### #### #    #    #### ####
";
        assert_eq!(result, expected_result);
    }
}

type Point = (usize, usize);

trait Fold {
    fn fold(&self, p: Point) -> Point;
}

impl Fold for (Option<usize>, Option<usize>) {
    fn fold(&self, (x, y): Point) -> Point {
        match self {
            (Some(x_fold), None) => {
                if x > *x_fold {
                    (2 * x_fold - x, y)
                } else {
                    (x, y)
                }
            }
            (None, Some(y_fold)) => {
                if y > *y_fold {
                    (x, 2 * y_fold - y)
                } else {
                    (x, y)
                }
            }
            _ => panic!(),
        }
    }
}
struct TransparentPaper {
    points: HashSet<Point>,
    folds: Vec<(Option<usize>, Option<usize>)>,
}

impl TransparentPaper {
    fn from(input: &str) -> Self {
        let mut points: HashSet<Point> = HashSet::new();
        let mut folds: Vec<(Option<usize>, Option<usize>)> = vec![];
        for line in input.lines() {
            if line.trim().is_empty() {
                continue;
            }
            if line.starts_with("fold along x=") {
                let x = line
                    .trim_start_matches("fold along x=")
                    .parse::<usize>()
                    .unwrap();
                folds.push((Some(x), None));
            } else if line.starts_with("fold along y=") {
                let y = line
                    .trim_start_matches("fold along y=")
                    .parse::<usize>()
                    .unwrap();
                folds.push((None, Some(y)));
            } else {
                let mut parts_iter = line.split_terminator(',');
                let x = parts_iter.next().unwrap().parse::<usize>().unwrap();
                let y = parts_iter.next().unwrap().parse::<usize>().unwrap();
                points.insert((x, y));
            }
        }
        Self { folds, points }
    }

    fn solve(&mut self) -> HashSet<Point> {
        for fold in &self.folds {
            self.points = self.fold(fold);
        }
        self.points.clone()
    }

    fn fold(&self, fold: &(Option<usize>, Option<usize>)) -> HashSet<Point> {
        let mut new_points = HashSet::new();
        for point in &self.points {
            new_points.insert(fold.fold(*point));
        }
        new_points.clone()
    }
}
