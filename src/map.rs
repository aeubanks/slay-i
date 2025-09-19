use rand::Rng;

use crate::{game::Rand, rng::rand_slice};

pub struct Map {
    nodes: Vec<Vec<Node>>,
}

#[derive(Default, Clone)]
struct Node {
    // x of connected nodes above
    edges: Vec<usize>,
}

const WIDTH: usize = 7;
const HEIGHT: usize = 15;
const PRINT_WIDTH: usize = WIDTH * 2 - 1;
const PRINT_HEIGHT: usize = HEIGHT * 2 - 1;
const PRINT_TOTAL: usize = PRINT_WIDTH * PRINT_HEIGHT;

struct Print {
    strs: [char; PRINT_TOTAL],
}

impl Print {
    fn new() -> Self {
        Self { strs: [' '; _] }
    }
    fn str(&self) -> String {
        let mut ret = String::new();
        for y in (0..PRINT_HEIGHT).rev() {
            for x in 0..PRINT_WIDTH {
                ret.push(self.get(x, y));
            }
            ret.push('\n');
        }

        ret
    }
    fn idx(x: usize, y: usize) -> usize {
        y * PRINT_WIDTH + x
    }
    fn set(&mut self, x: usize, y: usize, c: char) {
        self.strs[Print::idx(x, y)] = c
    }
    fn get(&self, x: usize, y: usize) -> char {
        self.strs[Print::idx(x, y)]
    }
}

impl Map {
    fn new() -> Self {
        Self {
            nodes: vec![vec![Node::default(); HEIGHT]; WIDTH],
        }
    }
    fn ancestor_depth(
        &self,
        mut y: usize,
        x1: usize,
        x2: usize,
        max_depth: usize,
    ) -> Option<usize> {
        assert_ne!(x1, x2);
        let mut depth = 0;
        let mut min_x = x1.min(x2);
        let mut max_x = x1.max(x2);
        while depth < max_depth && y != 0 {
            depth += 1;
            min_x = *self.parents(min_x, y).last().unwrap();
            max_x = *self.parents(max_x, y).first().unwrap();
            if min_x == max_x {
                return Some(depth);
            }
            y -= 1;
        }
        None
    }
    fn x_neighbors(x: usize) -> Vec<usize> {
        let mut ret = Vec::new();
        if x != 0 {
            ret.push(x - 1);
        }
        ret.push(x);
        if x != WIDTH - 1 {
            ret.push(x + 1);
        }
        ret
    }
    fn parents(&self, x: usize, y: usize) -> Vec<usize> {
        if y == 0 {
            return Vec::new();
        }
        let mut ret = Vec::new();
        for neighbor_x in Map::x_neighbors(x) {
            if self.nodes[neighbor_x][y - 1].edges.contains(&x) {
                ret.push(neighbor_x);
            }
        }
        ret
    }

    fn print(&self) {
        print!("{}", self.str());
    }
    fn str(&self) -> String {
        let mut print = Print::new();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if !self.nodes[x][y].edges.is_empty() || !self.parents(x, y).is_empty() {
                    print.set(x * 2, y * 2, '.');
                    for &e in &self.nodes[x][y].edges {
                        if e + 1 == x {
                            print.set(x * 2 - 1, y * 2 + 1, '\\');
                        } else if e == x {
                            print.set(x * 2, y * 2 + 1, '|');
                        } else if e == x + 1 {
                            print.set(x * 2 + 1, y * 2 + 1, '/');
                        } else {
                            panic!();
                        }
                    }
                }
            }
        }
        print.str()
    }
}

impl Map {
    pub fn generate(rng: &mut Rand) -> Self {
        let mut map = Map::new();
        let mut first_x = None;
        for _ in 0..6 {
            let mut cur_x = rng.random_range(0..WIDTH);
            // second starting x must be different from first starting x
            match first_x {
                Some(x) => {
                    while cur_x == x {
                        cur_x = rng.random_range(0..WIDTH);
                    }
                }
                None => first_x = Some(cur_x),
            }
            for y in 0..(HEIGHT - 1) {
                // choose one of three closest x values
                let mut next_x = rand_slice(rng, &Map::x_neighbors(cur_x));
                // reroll if ancestors are too close
                for p in map.parents(next_x, y + 1) {
                    if p == cur_x {
                        continue;
                    }
                    if let Some(a) = map.ancestor_depth(y, p, cur_x, 2)
                        && a <= 2
                    {
                        if next_x > cur_x {
                            if cur_x == 0 {
                                next_x = 0;
                            } else {
                                next_x = cur_x - rng.random_range(0..=1);
                            }
                        } else if next_x == cur_x {
                            next_x = rand_slice(rng, &Map::x_neighbors(cur_x));
                        } else {
                            if cur_x == WIDTH - 1 {
                                next_x = WIDTH - 1;
                            } else {
                                next_x = cur_x + rng.random_range(0..=1);
                            }
                        }
                    }
                }
                // adjust overlapping edges to not overlap
                if next_x > cur_x && map.nodes[cur_x + 1][y].edges.contains(&cur_x) {
                    next_x = cur_x;
                }
                if next_x < cur_x && map.nodes[cur_x - 1][y].edges.contains(&cur_x) {
                    next_x = cur_x;
                }
                map.nodes[cur_x][y].edges.push(next_x);
                cur_x = next_x;
            }
        }
        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ancestor_depth() {
        let mut map = Map::new();
        // (0, 0) -> (0, 1)
        // (0, 0) -> (1, 1)
        // (0, 1) -> (0, 2)
        // (1, 1) -> (1, 2)
        map.nodes[0][0].edges = vec![0, 1];
        map.nodes[0][1].edges = vec![0];
        map.nodes[1][1].edges = vec![1];

        // (6, 0) -> (6, 1)
        // (6, 1) -> (6, 2)
        map.nodes[6][0].edges = vec![6];
        map.nodes[6][1].edges = vec![6];

        // (2, 1) -> (1, 2)
        // (2, 1) -> (2, 2)
        // (2, 1) -> (3, 2)
        // (3, 0) -> (2, 1)
        // (3, 0) -> (4, 1)
        // (4, 1) -> (3, 2)
        // (4, 1) -> (4, 2)
        // (4, 1) -> (5, 2)
        map.nodes[2][1].edges = vec![1, 2, 3];
        map.nodes[3][0].edges = vec![2, 4];
        map.nodes[4][1].edges = vec![3, 4, 5];

        assert_eq!(map.ancestor_depth(2, 1, 6, 99), None);
        assert_eq!(map.ancestor_depth(2, 3, 5, 99), Some(1));
        assert_eq!(map.ancestor_depth(2, 3, 4, 99), Some(1));
        assert_eq!(map.ancestor_depth(2, 2, 4, 99), Some(2));
        assert_eq!(map.ancestor_depth(2, 2, 4, 2), Some(2));
        assert_eq!(map.ancestor_depth(2, 2, 4, 1), None);
    }

    #[test]
    fn test_map() {
        let mut rng = Rand::default();
        for _ in 0..20 {
            let map = Map::generate(&mut rng);
            let num_start_points = (0..WIDTH)
                .into_iter()
                .filter(|&x| !map.nodes[x][0].edges.is_empty())
                .count();
            assert!(num_start_points >= 2);
            for y in 0..HEIGHT {
                for x in 0..(WIDTH - 1) {
                    // check no overlapping edges
                    assert!(
                        !(map.nodes[x][y].edges.contains(&(x + 1))
                            && map.nodes[x + 1][y].edges.contains(&x))
                    );
                }
            }
        }
    }
}
