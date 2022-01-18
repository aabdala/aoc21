use std::{collections::HashMap, fmt::Display};

use super::TransparentPaper;

pub fn solution(input: &str) -> String {
    let mut tp = super::TransparentPaper::from(input);
    tp.solve();
    format!("\n{}", tp)
}

impl Display for TransparentPaper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut max_x = 0;
        let mut max_y = 0;
        let mut points_map = HashMap::new();
        for (x, y) in &self.points {
            max_x = *x.max(&max_x);
            max_y = *y.max(&max_y);
            points_map.entry(y).or_insert_with(Vec::new).push(x);
        }
        for y in 0..=max_y {
            let xs: HashMap<usize, bool> = points_map
                .get(&y)
                .unwrap_or_else(|| panic!("missing position y={}", y))
                .iter()
                .map(|x| (**x, true))
                .collect();
            for x in 0..=max_x {
                if xs.contains_key(&x) {
                    write!(f, "#")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        Result::Ok(())
    }
}
