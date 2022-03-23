use std::collections::{BTreeMap, HashMap};

pub fn solution(input: &str) -> String {
    let map = CaveMap::from(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| String::from(c).parse::<u8>().expect("invalid risk level"))
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<Vec<u8>>>(),
    );

    let target_pos = map.target_pos();
    let mut open_set = OpenSet::from(((0, 0), 0, target_pos.0 + target_pos.1));

    let mut steps = 0;
    loop {
        let node @ (cur_pos, risk_acc, _) = open_set.next();

        if cur_pos == target_pos {
            dbg!(steps);
            return format!("{}", risk_acc);
        }

        let moves = vec![
            map.move_left(node),
            map.move_right(node),
            map.move_up(node),
            map.move_down(node),
        ];

        moves
            .into_iter()
            .flatten()
            .for_each(|new_node| open_set.offer(new_node));
        steps += 1;
    }
}
type Node = ((usize, usize), usize, usize);

struct OpenSet {
    est_risk_to_node: BTreeMap<usize, Vec<Node>>,
    pos_to_lowest_risk: HashMap<(usize, usize), usize>,
}

impl OpenSet {
    fn from(initial: Node) -> Self {
        let mut est_risk_to_node = BTreeMap::new();
        est_risk_to_node.insert(initial.2, vec![initial]);
        let pos_to_lowest_risk = HashMap::new();
        OpenSet {
            est_risk_to_node,
            pos_to_lowest_risk,
        }
    }

    fn next(&mut self) -> Node {
        let first_key = self.est_risk_to_node.keys().next().unwrap();
        let nodes = self.est_risk_to_node.get(first_key).unwrap();
        let node = nodes[0].to_owned();
        let key = first_key.to_owned();
        if nodes.len() == 1 {
            self.est_risk_to_node.remove(&key);
        } else {
            let mut new_nodes = nodes.to_vec();
            new_nodes.remove(0);
            self.est_risk_to_node.insert(key, new_nodes);
        }
        node
    }

    fn insert_node(&mut self, new_node: Node) {
        self.est_risk_to_node
            .entry(new_node.2)
            .or_default()
            .push(new_node);
        self.pos_to_lowest_risk.insert(new_node.0, new_node.1);
    }

    fn offer(&mut self, new_node: Node) {
        let lowest_risk = self.pos_to_lowest_risk.get(&new_node.0);
        if let Some(risk) = lowest_risk {
            if new_node.1 < *risk {
                self.insert_node(new_node);
            }
        } else if lowest_risk.is_none() {
            self.insert_node(new_node);
        }
    }
}

struct CaveMap {
    map: Vec<Vec<u8>>,
    max_x: usize,
    max_y: usize,
}

impl CaveMap {
    fn from(map: Vec<Vec<u8>>) -> Self {
        CaveMap {
            map: map.clone(),
            max_y: map.len() - 1,
            max_x: map[0].len() - 1,
        }
    }

    fn target_pos(&self) -> (usize, usize) {
        (self.max_x, self.max_y)
    }

    fn move_right(&self, ((x, y), risk_acc, _): Node) -> Option<Node> {
        if x >= self.max_x {
            return None;
        }
        self.build_node(risk_acc, y, x + 1)
    }

    fn move_left(&self, ((x, y), risk_acc, _): Node) -> Option<Node> {
        if x == 0 {
            return None;
        }
        self.build_node(risk_acc, y, x - 1)
    }

    fn move_up(&self, ((x, y), risk_acc, _): Node) -> Option<Node> {
        if y == 0 {
            return None;
        }
        self.build_node(risk_acc, y - 1, x)
    }

    fn move_down(&self, ((x, y), risk_acc, _): Node) -> Option<Node> {
        if y >= self.max_y {
            return None;
        }
        self.build_node(risk_acc, y + 1, x)
    }

    fn build_node(&self, risk_acc: usize, new_y: usize, new_x: usize) -> Option<Node> {
        let new_risk = risk_acc + self.map[new_y][new_x] as usize;
        Some((
            (new_x, new_y),
            new_risk,
            new_risk as usize + self.max_x - new_x + self.max_y - new_y,
        ))
    }
}
