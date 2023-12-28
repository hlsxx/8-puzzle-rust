use std::collections::HashSet;

pub struct Graph {
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
    fn DFS(&mut self, current_node: usize) {
        let mut visited = HashSet::new();
        let mut stack = Vec::new();

        visited.insert(current_node);
        stack.push(current_node);

        while let Some(current_node) = stack.pop() {
            println!("{current_node}");

            for &neighbor in self.adj_list[current_node].iter() {
                if visited.insert(neighbor) {
                    stack.push(neighbor);
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
    graph.DFS(0);
}
