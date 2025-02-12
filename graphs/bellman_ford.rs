#[derive(Debug)]
struct Edge {
    from: usize,
    to: usize,
    weight: i32,
}

#[derive(Debug)]
struct Graph {
    edges: Vec<Edge>,
}

impl Graph {
    fn new(edges: Vec<Edge>) -> Self {
        Self { edges }
    }
}

fn bellman_ford(graph: &Graph, num_vertice: usize, source: usize) -> Result<Vec<i32>, &'static str> {
    let mut distance = vec![i32::MAX; num_vertice];
    distance[source] = 0;

    for _ in 0..num_vertice - 1 {
        for edge in &graph.edges {
            if distance[edge.from] != i32::MAX {
                let new_distace = distance[edge.from] + edge.weight;
                if new_distace < distance[edge.to] {
                    distance[edge.to] = new_distace;
                }
            }
        }
    }

    for edge in &graph.edges {
        if distance[edge.from] != i32::MAX {
            let new_distace = distance[edge.from] + edge.weight;
            if new_distace < distance[edge.to] {
                return Err("negative cycle");
            }
        }
    }
    Ok(distance)
}

fn main() {
    let edges = vec![
        Edge { from: 0, to: 1, weight: 4 },
        Edge { from: 0, to: 2, weight: 2 },
        Edge { from: 1, to: 2, weight: -3 },
        Edge { from: 1, to: 3, weight: 2 },
        Edge { from: 2, to: 3, weight: 3},
    ];

    let graph = Graph::new(edges);
    let result = bellman_ford(&graph, 4, 0);

    match result {
        Ok(distance) => println!("{:?}", distance),
        Err(error) => println!("{}", error),
    }
}