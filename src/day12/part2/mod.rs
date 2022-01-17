use std::{cell::RefCell, collections::HashMap, rc::Rc};

const CAVE_START: &str = "start";
const CAVE_END: &str = "end";

pub fn solution(input: &str) -> String {
    let cave_map: HashMap<&str, CaveRef> = get_cave_names(input)
        .into_iter()
        .map(|name| {
            (
                name,
                Rc::new(RefCell::new(Cave::with_name(String::from(name)))),
            )
        })
        .collect();

    input.split_ascii_whitespace().for_each(|line| {
        let parts = line.trim().split_terminator('-').collect::<Vec<_>>();
        let src_cave = cave_map.get(parts[0]).unwrap();
        let dst_cave = cave_map.get(parts[1]).unwrap();
        src_cave.borrow_mut().add_path(Rc::clone(dst_cave));
        dst_cave.borrow_mut().add_path(Rc::clone(src_cave));
    });
    let mut all_paths = vec![];
    cave_map
        .get(CAVE_START)
        .expect("starting cave missing")
        .borrow()
        .find_paths("".into(), &mut all_paths, &mut HashMap::new());
    let valid_paths_count = all_paths
        .iter()
        .filter(|path| path.ends_with(CAVE_END))
        .count();
    format!("{}", valid_paths_count)
}

fn get_cave_names(input: &str) -> Vec<&str> {
    input
        .split_ascii_whitespace()
        .flat_map(|line| line.trim().split_terminator('-'))
        .collect::<Vec<_>>()
}

type CaveRef = Rc<RefCell<Cave>>;

#[derive(Debug, Clone)]
struct Cave {
    name: String,
    paths: Vec<CaveRef>,
}

impl Cave {
    fn with_name(cave_name: String) -> Cave {
        Cave {
            name: cave_name,
            paths: vec![],
        }
    }

    fn add_path(&mut self, dst_cave: CaveRef) {
        self.paths.push(dst_cave);
    }

    fn find_paths(&self, path: String, acc: &mut Vec<String>, visits: &mut HashMap<String, usize>) {
        let new_path = format!("{}/{}", path, self.name);
        acc.push(new_path.clone());
        if self.name == CAVE_END {
            return;
        }

        *visits.entry(self.name.clone()).or_insert(0) += 1;
        self.paths
            .iter()
            .filter(|cave| cave.borrow().can_visit(visits))
            .for_each(|cave| {
                cave.borrow()
                    .find_paths(new_path.clone(), acc, &mut visits.clone())
            });
    }

    fn can_visit(&self, visits: &HashMap<String, usize>) -> bool {
        if !is_small_cavename(&self.name) {
            return true;
        }
        let has_visited_twice = visits
            .iter()
            .any(|(key, value)| is_small_cavename(key) && *value == 2);

        let mut visits_limit = 2;
        if matches!(self.name.as_str(), CAVE_START | CAVE_END) || has_visited_twice {
            visits_limit = 1;
        }

        *visits.get(&self.name).unwrap_or(&0) < visits_limit
    }
}

fn is_small_cavename(name: &str) -> bool {
    name.chars().take(1).all(|c| c.is_ascii_lowercase())
}
