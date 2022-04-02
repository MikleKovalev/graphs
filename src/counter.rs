use crate::graph::Graph;

struct VertexSet {
    set: Vec<usize>,
    other: Vec<usize>,
}

pub struct Counter {
    sets: Vec<VertexSet>,
}

impl Counter {
    pub fn new(size: usize) -> Counter {
        let mut permutation = vec![0; size];
        let mut sets = Vec::<VertexSet>::with_capacity(1 << size);
        for _ in 0..((1 << size) - 1) {
            let mut set = Vec::<usize>::with_capacity(size);
            let mut other = Vec::<usize>::with_capacity(size);
            let mut carry = 1;
            for i in 0..size {
                permutation[i] += carry;
                carry = permutation[i] / 2;
                permutation[i] %= 2;
                if permutation[i] == 0 {
                    other.push(i);
                } else {
                    set.push(i);
                }
            }
            let vertex_set = VertexSet {
                set,
                other,
            };
            sets.push(vertex_set);
        }
        let sort_rule = |first: &VertexSet, second: &VertexSet| {
            first.set.len().partial_cmp(&second.set.len()).unwrap()
        };
        sets.sort_by(sort_rule);
        Counter {
            sets,
        }
    }

    pub fn count(&self, graph: &Graph) -> (usize, usize) {
        let mut domination = 100;
        let mut independence = 0;
        for set in self.sets.iter() {
            match Counter::domination(&graph, &set.set, &set.other) {
                Some(value) => {
                    domination = value;
                    break;
                }
                _ => continue,
            }
        }
        for set in self.sets.iter().rev() {
            match Counter::independence(&graph, &set.set) {
                Some(value) => {
                    independence = value;
                    break;
                }
                _ => continue,
            }
        }
        (domination, independence)
    }

    fn domination(graph: &Graph, set: &Vec<usize>, other: &Vec<usize>) -> Option<usize> {
        for from in other.iter() {
            let mut has_edge = false;
            for to in set.iter() {
                if graph.has_edge(*from, *to) {
                    has_edge = true;
                    break;
                }
            }
            if !has_edge {
                return None;
            }
        }
        Some(set.len())
    }

    fn independence(graph: &Graph, set: &Vec<usize>) -> Option<usize> {
        for i in 0..set.len() {
            for j in i + 1..set.len() {
                if graph.has_edge(i, j) {
                    return None;
                }
            }
        }
        Some(set.len())
    }
}
