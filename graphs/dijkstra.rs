use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub fn dijkstra(graph: &Vec<Vec<(usize, usize)>>, start: usize) -> Vec<usize> {
    let n = graph.len();
    let mut distances = vec![usize::MAX; n];
    distances[start] = 0;

    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, start)));

    while let Some(Reverse((current_dist, current_node))) = heap.pop() {
        if current_dist > distances[current_node] {
            continue;
        }

        for &(neighbor, weight) in &graph[current_node] {
            let tentative_dist = current_dist + weight;
            if tentative_dist < distances[neighbor] {
                distances[neighbor] = tentative_dist;
                heap.push(Reverse((tentative_dist, neighbor)));
            }
        }
    }
    distances
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_graph() {               
                                // estrutura do grafo
                                // 0 - (4) -> 1
                                // 0 - (1) -> 2
                                // 1 - (1) -> 3
                                // 2 - (2) -> 1, 2 - (5) -> 3
        let graph = vec![
            vec![(1, 4), (2, 1)],  // 0
            vec![(3, 1)],          // 1
            vec![(1, 2), (3, 5)],  // 2
            vec![]                 // 3
        ];

        let distances = dijkstra(&graph, 0);
        assert_eq!(distances, vec![0, 3, 1, 4]);
    }

    #[test]
    fn unreachable_node() {
        let graph = vec![
            vec![(1, 2)], // 0
            vec![],       // 1
            vec![],       // 2 unreachable
        ];

        let distances = dijkstra(&graph, 0);
        assert_eq!(distances, vec![0, 2, usize::MAX]);
    }

    #[test]
    fn single_node() {
        let graph = vec![vec![]];
        let distances = dijkstra(&graph, 0);
        assert_eq!(distances, vec![0]);
    }
}

fn main() {
    let graph = vec![
        vec![(1, 4), (2, 1)],
        vec![(3, 1)],
        vec![(1, 2), (3, 5)],
        vec![]
    ];

    let start = 0;
    let distances = dijkstra(&graph, start);
    println!("node: {} - {:?}", start, distances)
}