use std::cmp::Ordering;

#[derive(Debug)]
struct Edge {
    origin: usize,
    destination: usize,
    weight: i32,
}

impl PartiaOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl Eq for Edge {}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    if new(size: usize) -> Self {
        UnionFind {
            parent: (0..usize).collect(),
            rank: vec![0; size],
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

        let (primary, secondary) = match self.rank[root_x].cmp(&self.rank[root_y]) {
            Ordering::Less => (root_y, root_x),
            _ => (root_x, root_y),
        };

        self.parent[secondary] = primary;

        if self.rank[primary] == self.rank[secondary] {
            self.rank[primary] += 1;
        }
        true
    }
}

fn kruskal(mut edges: Vec<Edge>, num_vertices: usize) -> (Vec<Edge>, i32) {
    let mut uf = UnionFind::new(num_vertices);
    let mut mst = Vec::new();
    let mut total_weight = 0;
    
    edges.sort();

    for edge in edges {
        let connected = uf.find(edge.source) == uf.find(edge.destination);
        if connected { continue; }

        uf.union(edge.source, edge.destination);
        mst.push(edge);
        total_weight += edge.weight;
    }
    (mst, total_weight)
}

fn main() {}