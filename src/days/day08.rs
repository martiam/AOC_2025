use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////
fn distance((x1, y1, z1): (i64, i64, i64), (x2, y2, z2): (i64, i64, i64)) -> i64 {
    let dx = x1 - x2;
    let dy = y1 - y2;
    let dz = z1 - z2;
    dx * dx + dy * dy + dz * dz
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        if self.size[root_x] < self.size[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }

        true
    }
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/8.txt").unwrap();
    let junctions: Vec<(i64, i64, i64)> = input
        .lines()
        .map(|line| {
            let parts: Vec<i64> = line.split(",").map(|s| s.parse::<i64>().unwrap()).collect();
            (parts[0], parts[1], parts[2])
        })
        .collect();
    let mut edges = Vec::new();

    for i in 0..junctions.len() {
        for j in (i + 1)..junctions.len() {
            let distance = distance(junctions[i], junctions[j]);
            edges.push((distance, i, j));
        }
    }

    edges.sort_unstable();

    let mut uf = UnionFind::new(junctions.len());

    for (_, i, j) in edges.iter().take(1000) {
        uf.union(*i, *j);
    }

    let mut circuit_sizes: HashMap<usize, usize> = HashMap::new();
    for i in 0..junctions.len() {
        let root = uf.find(i);
        *circuit_sizes.entry(root).or_insert(0) += 1;
    }
    let mut sizes: Vec<usize> = circuit_sizes.values().copied().collect();
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    let sol1 = sizes[0] * sizes[1] * sizes[2];
    
    // Part 2
    let mut sol2 = 0;
    let mut uf = UnionFind::new(junctions.len());
    for (_, i, j) in edges {
        uf.union(i, j);
        
        let root = uf.find(0);
        if uf.size[root] == junctions.len() {
            let x1 = junctions[i].0;
            let x2 = junctions[j].0;
            sol2 = x1 * x2;
            break;
        }
    }

    (Solution::from(sol1), Solution::from(sol2))
}
