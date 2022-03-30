use super::{find_path_risk, read_cavemap};

pub fn solution(input: &str) -> String {
    let map = read_cavemap(input);
    find_path_risk(map)
}
