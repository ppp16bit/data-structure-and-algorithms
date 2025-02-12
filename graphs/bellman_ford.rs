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