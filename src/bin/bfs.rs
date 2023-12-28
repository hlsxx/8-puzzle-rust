use std::collections::{VecDeque, HashSet};

struct Graph {
    adj_list: Vec<Vec<usize>>
}

impl Graph {
    fn new(nodes_count: usize) -> Self {
        Self {
            adj_list: vec![Vec::new(); nodes_count]
        }
    }

    fn add_edge(&mut self, parent: usize, current: usize) {
        self.adj_list[parent].push(current);
    }

    #[allow(non_snake_case)]
    fn BFS(&mut self, current_node: usize) {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        visited.insert(current_node);
        queue.push_back(current_node);

        while let Some(current_node) = queue.pop_front() {
            println!("{current_node}");
            
            for &neighbor in self.adj_list[current_node].iter() {
                if visited.insert(neighbor) {
                    queue.push_back(neighbor);
                }
            }
        }
    }
}

fn main() {
    let mut graph = Graph::new(7);

    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(0, 3);
    graph.add_edge(1, 4);
    graph.add_edge(2, 5);
    graph.add_edge(3, 6);
    graph.add_edge(4, 5);
    graph.add_edge(6, 5);
    graph.BFS(0);
}
