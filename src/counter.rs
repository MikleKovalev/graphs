use crate::graph::Graph;

pub struct Counter {
    in_set: Vec<Vec<usize>>,
    not_in_set: Vec<Vec<usize>>,
}

impl Counter {
    pub fn new(size: usize) -> Counter {
        let mut permutation = vec![0; size];
        let mut in_set = Vec::<Vec<usize>>::with_capacity(1 << size);
        let mut not_in_set = Vec::<Vec<usize>>::with_capacity(1 << size);
        for _ in 0..1 << size {
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
            in_set.push(set);
            not_in_set.push(other);
        }
        Counter {
            in_set,
            not_in_set,
        }
    }

    pub fn count(&self, graph: &Graph) -> (usize, usize) {
        let mut domination = 100;
        let mut independence = 0;
        for (set, other) in self.in_set.iter().zip(self.not_in_set.iter()) {
            if let Some(d) = Counter::domination(&graph, &set, &other) {
                domination = domination.min(d);
            }
            if let Some(i) = Counter::independence(&graph, &set) {
                independence = independence.max(i);
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
