use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Edge {
    source: usize,
    destination: usize,
    weight: i32,
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            parent: (0..size).collect(),
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

    // sinto fortes dores de cabeça 
    for edge in edges {
        let connected = uf.find(edge.source) == uf.find(edge.destination);
        if connected { continue; }

        uf.union(edge.source, edge.destination);
        total_weight += edge.weight;
        mst.push(edge);

    (mst, total_weight)
}

fn main() {
    let edges = vec![
        Edge { source: 0, destination: 1, weight: 10 },
        Edge { source: 0, destination: 2, weight: 6 },
        Edge { source: 0, destination: 3, weight: 5 },
        Edge { source: 1, destination: 3, weight: 15 },
        Edge { source: 2, destination: 3, weight: 4 },
    ];

    let (mst, total_weight) = kruskal(edges, 4);

    println!("mst:");
    mst.iter().for_each(|e| println!("{} - {}: {}", e.source, e.destination, e.weight));
    println!("total: {}", total_weight);
}