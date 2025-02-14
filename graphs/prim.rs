use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub struct Graph {
    adj_list: Vec<Vec<(usize, i32)>>,
}

impl Graph {
    pub fn new(n: usize) -> Self {
        Graph {
            adj_list: vec![vec![]; n],
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize, weight: i32) {
        self.adj_list[u].push((v, weight));
        self.adj_list[v].push((u, weight));
    }
}

pub fn prim_mst(graph: &Graph) -> Option<(i32, Vec<(usize, usize, i32)>)> {
    let n = graph.adj_list.len();
    if n == 0 {
        return Some((0, vec![]));
    }

    let mut visited = vec![false; n];
    let mut heap = BinaryHeap::new();
    let mut mst_edges = Vec::new();
    let mut total_weight = 0;

    visited[0] = true;
    for &(v, w) in &graph.adj_list[0] {
        heap.push(Reverse((w, 0, v)));
    }

    while let Some(Reverse((w, u, v))) = heap.pop() {
        if visited[v] {
            continue;
        }

        visited[v] = true;
        mst_edges.push((u, v, w));
        total_weight += w;

        for &(next_v, next_w) in &(graph.adj_list[v]) {
            heap.push(Reverse((next_w, v, next_v)));
        }
    }

    // check
    if visited.iter().all(|&v| v) {
        Some((total_weight, mst_edges))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prim_mst() {
        // connected graph
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1, 1);
        graph.add_edge(0, 2, 2);
        graph.add_edge(1, 2, 3);

        let result = prim_mst(&graph).unwrap();
        assert_eq!(result.0, 3);
        assert_eq!(result.1, vec![(0, 1, 1), (0, 2, 2)]);

        // disconnected
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1, 1);
        graph.add_edge(2, 3, 2);
        assert!(prim_mst(&graph).is_none());

        // single node
        let graph = Graph::new(1);
        let result = prim_mst(&graph).unwrap();
        assert_eq!(result.0, 0);
        assert!(result.1.is_empty());

        // empty
        let graph = Graph::new(0);
        let result = prim_mst(&graph).unwrap();
        assert_eq!(result.1.is_empty());
    }
}

fn main() {
    let mut graph = Graph::new(5);

    graph.add_edge(0, 1, 2);
    graph.add_edge(0, 3, 6);
    graph.add_edge(1, 2, 3);
    graph.add_edge(1, 3, 8);
    graph.add_edge(1, 4, 5);
    graph.add_edge(2, 4, 7);
    graph.add_edge(3, 4, 9);

    match prim_mst(&graph) {
        Some((total_weight, mst_edges)) => {
            println!("weight: {}", total_weight);
            println!("edges in mst:");
            for (u, v, w) in mst_edges {
                println!("{} - {} (weight: {})", u, v, w);
            }
        }
        None => {
            println!("no mst exists");
        }
    }
}