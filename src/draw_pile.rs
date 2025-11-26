use crate::{game::Rand, rng::rand_slice};

use petgraph::visit::EdgeRef;

use std::fmt::Debug;

#[derive(Debug)]
struct Node<T: Debug> {
    value: T,
    can_draw: bool,
}

#[derive(Debug)]
enum Edge {
    Unlock,  // drawing the source card lets the target card be drawn if it couldn't be
    Ordered, // the source card must be drawn after the target card
}

pub struct DrawPile<T: Debug> {
    graph: petgraph::graph::DiGraph<Node<T>, Edge>,
}

impl<T: Debug> Default for DrawPile<T> {
    fn default() -> Self {
        Self {
            graph: Default::default(),
        }
    }
}

impl<T: Debug> DrawPile<T> {
    pub fn new(priority: Vec<T>, normal: Vec<T>) -> Self {
        let mut ret = Self::default();
        let mut normal_nodes = Vec::new();
        let mut priority_nodes = Vec::new();
        for t in normal {
            let n = ret.graph.add_node(Node {
                value: t,
                can_draw: true,
            });
            normal_nodes.push(n);
        }
        for t in priority {
            let n = ret.graph.add_node(Node {
                value: t,
                can_draw: true,
            });
            priority_nodes.push(n);
        }
        for target in priority_nodes {
            for &source in &normal_nodes {
                ret.graph.add_edge(source, target, Edge::Ordered);
            }
        }
        ret
    }
    pub fn push_top(&mut self, t: T) {
        let all = self.graph.node_indices();
        let n = self.graph.add_node(Node {
            value: t,
            can_draw: true,
        });
        for source in all {
            self.graph.add_edge(source, n, Edge::Ordered);
        }
    }
    pub fn push_bottom(&mut self, t: T) {
        let all = self.graph.node_indices();
        let n = self.graph.add_node(Node {
            value: t,
            can_draw: true,
        });
        for target in all {
            self.graph.add_edge(n, target, Edge::Ordered);
        }
    }
    pub fn shuffle_in_one(&mut self, t: T) {
        if self.is_empty() {
            self.graph.add_node(Node {
                value: t,
                can_draw: true,
            });
        } else {
            let all = self.graph.node_indices();
            let n = self.graph.add_node(Node {
                value: t,
                can_draw: false,
            });
            for source in all {
                self.graph.add_edge(source, n, Edge::Unlock);
            }
        }
    }
    pub fn shuffle_all(&mut self) {
        self.graph.clear_edges();
        for n in self.graph.node_indices() {
            self.graph[n].can_draw = true;
        }
    }
    pub fn take(&mut self, i: usize) -> T {
        for (ni, n) in self.graph.node_indices().enumerate() {
            if ni == i {
                if self.graph[n].can_draw {
                    let set_can_draw_nodes = self
                        .graph
                        .edges(n)
                        .filter(|e| matches!(e.weight(), Edge::Unlock))
                        .map(|e| e.target())
                        .collect::<Vec<_>>();

                    for n in set_can_draw_nodes {
                        self.graph[n].can_draw = true;
                    }
                }
                let node = self.graph.remove_node(n).unwrap();
                return node.value;
            }
        }
        panic!()
    }
    pub fn get(&self, i: usize) -> &T {
        self.get_all()[i]
    }
    #[cfg(test)]
    pub fn top(&self, rng: &mut Rand) -> &T {
        let possible = self.possible_indexes_to_draw();
        let c = rand_slice(rng, &possible);
        self.get(possible[c])
    }
    pub fn get_all(&self) -> Vec<&T> {
        self.graph
            .node_indices()
            .map(|i| &self.graph[i].value)
            .collect()
    }
    pub fn clear(&mut self) {
        self.graph.clear();
    }
    pub fn len(&self) -> usize {
        self.graph.node_count()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn pop(&mut self, rng: &mut Rand) -> T {
        let possible = self.possible_indexes_to_draw();
        let c = rand_slice(rng, &possible);
        for (ni, n) in self.graph.node_indices().enumerate() {
            if ni == c {
                assert!(self.graph[n].can_draw);
                break;
            }
        }
        self.take(c)
    }
    pub fn possible_indexes_to_draw(&self) -> Vec<usize> {
        assert!(!self.is_empty());
        let mut ret = Vec::new();
        for (i, n) in self.graph.node_indices().enumerate() {
            if self.graph[n].can_draw
                && !self
                    .graph
                    .edges(n)
                    .any(|e| matches!(e.weight(), Edge::Ordered))
            {
                ret.push(i);
            }
        }
        ret
    }
    #[cfg(test)]
    pub fn possible_values_to_draw(&self) -> Vec<&T> {
        self.possible_indexes_to_draw()
            .iter()
            .map(|i| self.get(*i))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Eq;
    use std::collections::HashSet;
    use std::fmt::Debug;
    use std::hash::Hash;

    use rand::{Rng, SeedableRng};

    use crate::test::assert_set_eq;

    use super::*;

    struct Seen<T: Hash + Eq + Copy + Debug> {
        seen: HashSet<T>,
        expected: HashSet<T>,
    }

    impl<T: Hash + Eq + Copy + Debug> Seen<T> {
        fn slice_to_set(ts: &[T]) -> HashSet<T> {
            ts.iter().copied().collect()
        }
        fn new(ts: &[T]) -> Self {
            Self {
                seen: Default::default(),
                expected: Self::slice_to_set(ts),
            }
        }
        fn add(&mut self, t: T) {
            if !self.expected.contains(&t) {
                panic!("unexpected {:?}", t);
            }
            self.seen.insert(t);
        }
        fn done(&self) -> bool {
            self.expected == self.seen
        }
        fn assert_done(&self) {
            assert_eq!(self.expected, self.seen);
        }
    }

    #[test]
    fn test_seen_success() {
        let mut seen = Seen::new(&[2, 3]);
        seen.add(2);
        seen.add(3);
        seen.add(2);
        seen.add(3);
        seen.assert_done();
    }

    #[test]
    #[should_panic]
    fn test_seen_missing() {
        let mut seen = Seen::new(&[2, 3]);
        seen.add(2);
        seen.add(2);
        seen.assert_done();
    }

    #[test]
    #[should_panic]
    fn test_seen_unexpected() {
        let mut seen = Seen::new(&[2, 3]);
        seen.add(2);
        seen.add(4);
    }

    #[test]
    fn test_shuffled_basic() {
        let mut rng = Rand::default();
        let mut seen = Seen::new(&[1, 2, 3]);
        for _ in 0..100 {
            let mut d = DrawPile::<i32>::new(vec![], vec![1, 2, 3]);
            assert_eq!(d.len(), 3);
            let v = d.pop(&mut rng);
            seen.add(v);
            assert_eq!(d.len(), 2);
            match v {
                1 => {
                    assert_set_eq(d.get_all(), &[2, 3]);
                }
                2 => {
                    assert_set_eq(d.get_all(), &[1, 3]);
                }
                3 => {
                    assert_set_eq(d.get_all(), &[2, 1]);
                }
                _ => panic!(),
            }
            if seen.done() {
                break;
            }
        }
        seen.assert_done();
    }

    #[test]
    #[should_panic]
    fn test_empty() {
        let mut rng = Rand::default();
        let mut d = DrawPile::<i32>::default();
        d.pop(&mut rng);
    }

    #[test]
    fn test_priority() {
        let mut rng = Rand::default();

        for _ in 0..10 {
            let mut d = DrawPile::<i32>::new(vec![4, 5], vec![1, 2, 3]);
            assert_set_eq(d.possible_values_to_draw(), &[4, 5]);
            d.pop(&mut rng);
            d.pop(&mut rng);
            assert_set_eq(d.possible_values_to_draw(), &[1, 2, 3]);
        }
    }

    #[test]
    fn test_top() {
        let mut rng = Rand::default();
        let mut d = DrawPile::<i32>::default();
        d.push_top(1);
        d.push_top(2);
        d.push_top(3);
        assert_eq!(d.pop(&mut rng), 3);
        assert_eq!(d.pop(&mut rng), 2);
        assert_eq!(d.pop(&mut rng), 1);
    }

    #[test]
    fn test_bottom() {
        let mut rng = Rand::default();
        let mut d = DrawPile::<i32>::default();
        d.push_bottom(1);
        d.push_bottom(2);
        d.push_bottom(3);
        assert_eq!(d.pop(&mut rng), 1);
        assert_eq!(d.pop(&mut rng), 2);
        assert_eq!(d.pop(&mut rng), 3);
    }

    #[test]
    fn test_top_bottom() {
        let mut rng = Rand::default();
        let mut d = DrawPile::<i32>::default();
        d.push_bottom(1);
        d.push_top(2);
        d.push_bottom(3);
        d.push_top(4);
        assert_eq!(d.pop(&mut rng), 4);
        assert_eq!(d.pop(&mut rng), 2);
        assert_eq!(d.pop(&mut rng), 1);
        assert_eq!(d.pop(&mut rng), 3);
    }

    #[test]
    fn test_top_shuffle() {
        let mut rng = Rand::default();
        let mut d = DrawPile::<i32>::new(vec![], vec![1, 2]);
        d.push_top(3);
        assert_eq!(d.pop(&mut rng), 3);
    }

    #[test]
    fn test_shuffled_in_1() {
        let mut rng = Rand::default();
        let mut d = DrawPile::<i32>::new(vec![], vec![1]);
        d.shuffle_in_one(2);
        assert_eq!(d.pop(&mut rng), 1);
        assert_eq!(d.pop(&mut rng), 2);
    }

    #[test]
    fn test_shuffled_in_2() {
        let mut rng = Rand::default();
        let mut d = DrawPile::<i32>::new(vec![], vec![1, 1]);
        d.shuffle_in_one(2);
        assert_eq!(d.pop(&mut rng), 1);
        assert_set_eq(d.possible_values_to_draw(), &[1, 2]);
    }

    #[test]
    fn test_shuffled_in_3() {
        let mut rng = Rand::default();
        let mut d = DrawPile::<i32>::default();
        d.shuffle_in_one(1);
        assert_eq!(d.pop(&mut rng), 1);
    }

    #[test]
    fn test_shuffled_in_4() {
        let mut rng = Rand::default();
        let mut d = DrawPile::<i32>::default();
        d.shuffle_in_one(1);
        d.shuffle_in_one(2);
        assert_eq!(d.pop(&mut rng), 1);
        assert_eq!(d.pop(&mut rng), 2);
    }

    #[test]
    fn test_shuffled_in_5() {
        let mut rng = Rand::default();
        let mut d = DrawPile::<i32>::default();
        d.shuffle_in_one(1);
        d.shuffle_in_one(2);
        assert_eq!(d.take(0), 1);
        assert_eq!(d.pop(&mut rng), 2);
    }

    #[test]
    fn test_shuffled_in_6() {
        let mut rng = Rand::default();
        let mut d = DrawPile::<i32>::default();
        d.shuffle_in_one(0);
        d.shuffle_in_one(1);
        d.shuffle_in_one(2);
        d.shuffle_in_one(3);
        assert_eq!(d.take(1), 1);
        assert_eq!(d.pop(&mut rng), 0);
        assert_set_eq(d.possible_values_to_draw(), &[2, 3]);
    }

    #[test]
    fn test_shuffled_in_bottom() {
        let mut rng = Rand::default();
        let mut d = DrawPile::<i32>::default();
        d.push_bottom(0);
        d.shuffle_in_one(1);
        assert_eq!(d.pop(&mut rng), 0);
    }

    #[test]
    fn test_get_all() {
        let mut d = DrawPile::<i32>::default();
        d.push_bottom(0);
        d.push_top(1);
        d.shuffle_in_one(2);
        d.push_bottom(3);
        assert_eq!(d.get_all(), vec![&0, &1, &2, &3]);
    }

    #[test]
    fn test_shuffle_all() {
        let mut d = DrawPile::<i32>::default();
        d.shuffle_in_one(0);
        d.shuffle_in_one(1);
        d.shuffle_in_one(2);
        d.shuffle_in_one(3);
        d.shuffle_all();
        assert_set_eq(d.possible_values_to_draw(), &[0, 1, 2, 3]);
    }

    #[test]
    fn fuzz() {
        let mut rng = rand::rngs::SmallRng::seed_from_u64(1);
        for _ in 0..100 {
            let mut dp = DrawPile::default();
            let mut test_pile = Vec::new();
            // random actions to the DrawPile as well as a mirror vector where we mirror the actions
            for x in 0..5 {
                match rng.random_range(0..3) {
                    0 => {
                        dp.push_top(x);
                        test_pile.push(x);
                    }
                    1 => {
                        dp.push_bottom(x);
                        test_pile.insert(0, x);
                    }
                    2 => {
                        dp.shuffle_in_one(x);
                        if test_pile.is_empty() {
                            test_pile.push(x);
                        } else {
                            test_pile.insert(rng.random_range(0..test_pile.len()), x);
                        }
                    }
                    _ => panic!(),
                }
            }
            // check if the mirror vector is in an order that the DrawPile allows
            while let Some(v) = test_pile.pop() {
                assert!(dp.possible_values_to_draw().contains(&&v));
                let idx = dp.get_all().into_iter().position(|e| *e == v).unwrap();
                dp.take(idx);
            }
            assert!(dp.is_empty());
        }
    }
}
