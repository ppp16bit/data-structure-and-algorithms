use std::collections::HashSet;

struct Graph {
    adjacencies: Vec<Vec<usize>>,
}

impl Graph {
    fn new(size: usize) -> Self {
        Self {
            adjacencies: vec![Vec::new(); size],
        }
    }
    
    fn add_edge(&mut self, origin: usize, destination: usize) {
        self.adjacencies[origin].push(destination);
    }

    fn dfs(&self, start: usize) {
        let mut visited = HashSet::new();
        let mut stack = vec![start];

        // add neighbor in reverse order keep
        while let Some(no) = stack.pop() {
            if visited.insert(no) {
                println!("{}", no);
                for &neighbor in self.adjacencies[no].iter().rev() {
                    if !visited.contains(&neighbor) {
                        stack.push(neighbor);
                    }
                }
            }
        }
    }
}

fn main() {
    let mut graph = Graph::new(5);

    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(1, 3);
    graph.add_edge(2, 3);
    graph.add_edge(3, 4);

    graph.dfs(0);
}