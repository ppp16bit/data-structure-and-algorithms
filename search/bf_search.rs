use std::collections::VecDeque;

fn bfs(graph: &Vec<Vec<usize>>, start: usize) -> Option<Vec<usize>> {
    if start >= graph.len() {
        return None;
    }

    let mut visited = vec![false; graph.len()];
    let mut queue = VecDeque::new();
    let mut visit = Vec::new();

    visited[start] = true;
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        visit.push(node);
        for &neighbor in &graph[node] {
            if neighbor >= graph.len() {
                return None;
            }
            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back(neighbor);
            }
        }
    }
    Some(visit)
}

fn main() {
    let graph = vec![
        vec![1,2],
        vec![0,3],
        vec![0,3],
        vec![1,2,4],
        vec![3],
    ];

    // usei match para evitar if else
    match bfs(&graph, 0) {
        Some(order) => println!("{:?}", order),
        None => println!(r" /\_/\ 
                          =(+ - +)=    cat"), // gato
    }
}