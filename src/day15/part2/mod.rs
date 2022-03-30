use super::{find_path_risk, read_cavemap, CaveMap};

pub fn solution(input: &str) -> String {
    let map = read_cavemap(input).extend();
    find_path_risk(map)
}

impl CaveMap {
    fn extend(&self) -> Self {
        let times = 5;
        let size_x = self.max_x + 1;
        let size_y = self.max_y + 1;
        let new_size_x = size_x * times;
        let new_size_y = size_y * times;
        let mut new_map = Vec::with_capacity(new_size_y);
        for y in 0..(new_size_y) {
            new_map.push(Vec::with_capacity(new_size_x));
            for x in 0..(new_size_x) {
                let mod_x = x.rem_euclid(size_x);
                let mod_y = y.rem_euclid(size_y);
                let div_x = x.div_euclid(size_x);
                let div_y = y.div_euclid(size_y);
                let mut new_risk = self.map[mod_y][mod_x] as usize + div_x + div_y;
                if new_risk > 9 {
                    new_risk = new_risk.rem_euclid(9);
                }
                new_map[y].push(new_risk as u8);
            }
        }
        CaveMap::from(new_map)
    }
}
