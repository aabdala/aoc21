pub fn solution(input: &str) -> String {
    let floormap: Vec<Vec<u8>> = input
        .split_ascii_whitespace()
        .map(|line| {
            line.chars()
                .map(|c| String::from(c).parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect();
    let low_points = super::get_low_points(&floormap);
    let mut basin_sizes: Vec<usize> = low_points
        .iter()
        .map(|low_point| get_basin_size(&floormap, *low_point, &mut vec![]))
        .collect();
    basin_sizes.sort_unstable();

    format!(
        "{}",
        basin_sizes
            .iter()
            .skip(basin_sizes.len() - 3)
            .map(|f| {
                dbg!(f);
                f
            })
            .product::<usize>()
    )
}

fn get_basin_size(
    floormap: &[Vec<u8>],
    (i, j, d): (usize, usize, u8),
    visited: &mut Vec<(usize, usize)>,
) -> usize {
    if visited.contains(&(i, j)) {
        return 0;
    }
    visited.push((i, j));
    let max_i = floormap.len() - 1;
    let max_j = floormap.get(0).unwrap().len() - 1;
    let mut basin_size = 1;
    let basin_limit: &u8 = &9;
    if i != 0 {
        // look up
        let depth = floormap.get(i - 1).unwrap().get(j).unwrap();
        if depth != basin_limit && depth >= &d {
            basin_size += get_basin_size(floormap, (i - 1, j, *depth), visited);
        }
    }
    if i < max_i {
        // look down
        let depth = floormap.get(i + 1).unwrap().get(j).unwrap();
        if depth != basin_limit && depth >= &d {
            basin_size += get_basin_size(floormap, (i + 1, j, *depth), visited);
        }
    }
    if j != 0 {
        // look left
        let depth = floormap.get(i).unwrap().get(j - 1).unwrap();
        if depth != basin_limit && depth >= &d {
            basin_size += get_basin_size(floormap, (i, j - 1, *depth), visited);
        }
    }
    if j != max_j {
        // look right
        let depth = floormap.get(i).unwrap().get(j + 1).unwrap();
        if depth != basin_limit && depth >= &d {
            basin_size += get_basin_size(floormap, (i, j + 1, *depth), visited);
        }
    }
    basin_size
}
