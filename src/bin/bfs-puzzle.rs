use std::collections::VecDeque;

#[derive(Clone, Debug)]
struct Node {
    children: Vec<Node>,
    initial_vector: Vec<usize>,
    parent: Option<Box<Node>>
}

impl Node {
    pub fn new(initial_vector: Vec<usize>, parent: Option<Box<Node>>) -> Self {
        Self {
            initial_vector,
            children: Vec::new(),
            parent
        }
    }

    fn find_zero(&self) -> Option<usize> {
        self.initial_vector.iter().position(|&x| x == 0)
    }

    pub fn print_puzzle(&self) {
        let mut count: u8 = 0;

        for i in self.initial_vector.iter() {
            if count % 3 == 0 { println!("") };
            print!("{i} ");

            count += 1;
        } 
    }

    fn move_up(&mut self) {
        if let Some(z_pos) = self.find_zero() {
            if z_pos >= 3 {
                let mut temp = self.initial_vector.clone();
                temp.swap(z_pos, z_pos - 3);

                self.children.push(Node::new(temp, Some(Box::new(self.clone()))));
            }
        }
    }

    fn move_down(&mut self) {
        if let Some(z_pos) = self.find_zero() {
            if z_pos < 6 {
                let mut temp = self.initial_vector.clone();
                temp.swap(z_pos, z_pos + 3);
                self.children.push(Node::new(temp, Some(Box::new(self.clone()))));
            }
        }
    }

    fn move_right(&mut self) {
        if let Some(z_pos) = self.find_zero() {
            if z_pos % 3 < 2 {
                let mut temp = self.initial_vector.clone();
                temp.swap(z_pos, z_pos + 1);
                self.children.push(Node::new(temp, Some(Box::new(self.clone()))));
            }
        }
    }

    fn move_left(&mut self) {
        if let Some(z_pos) = self.find_zero() {
            if z_pos % 3 > 0 {
                let mut temp = self.initial_vector.clone();
                temp.swap(z_pos, z_pos - 1);
                self.children.push(Node::new(temp, Some(Box::new(self.clone()))));
            }
        }
    }
}

fn is_visited(queue: &VecDeque<Node>, current_node: Node) -> bool {
    let mut tmp_queue = queue.clone();
    let mut exist = false;

    while let Some(node) = tmp_queue.pop_front() {
        if node.initial_vector == current_node.initial_vector {
            exist = true;
        }
    }

    exist
}

struct Puzzle {
    queue: VecDeque<Node>,
    visited: VecDeque<Node>,
    final_vector: Vec<usize>,
    nodes_count: usize
}

impl Puzzle {
    pub fn new(initial_vector: Vec<usize>, final_vector: Vec<usize>) -> Self {
        let initial_node = Node::new(initial_vector, None);
        let mut queue = VecDeque::new();

        queue.push_back(initial_node);

        Self {
            queue,
            visited: VecDeque::new(),
            final_vector,
            nodes_count: 0
        }
    }

    #[allow(non_snake_case)]
    pub fn BFS(&mut self) -> Result<(Node, usize), String> {
        println!("Searching for solution...");

        while let Some(current_node) = self.queue.pop_front().as_mut() {
            self.visited.push_back(current_node.clone());

            current_node.move_right();
            current_node.move_up();
            current_node.move_left();
            current_node.move_down();

            for current_child in current_node.children.iter() {
                if current_child.initial_vector == self.final_vector {
                    return Ok((current_child.clone(), self.nodes_count));
                }

                if !is_visited(&self.queue, current_child.clone()) && !is_visited(&self.visited, current_child.clone()) {
                    self.queue.push_back(current_child.clone());
                }
            }

            self.nodes_count += 1;
        }

        Err("Solution not found :(".to_string())
    }
}

fn trace_solution(mut node: Node) {
    let mut sol = Vec::new();

    sol.push(node.clone());

    while let Some(parent_node) = node.parent {
        node = *parent_node;
        sol.push(node.clone());
    }

    let mut depth: u32= 0;
    for i in sol.iter().rev() {
        depth += 1;
        i.print_puzzle();
        println!();
    }

    println!("Depth: {}", depth - 1);
}

fn main() {
    let initial_vector: Vec<usize> = vec![
        1, 2, 3,
        0, 4, 6,
        7, 5, 8
    ];

    let final_vector: Vec<usize> = vec![
        1, 2, 3,
        4, 5, 6,
        7, 8, 0
    ];

    let mut puzzle = Puzzle::new(initial_vector, final_vector);
    match puzzle.BFS() {
        Ok((node, nodes_count)) => {
            println!("Solution found :)");
            trace_solution(node);
            println!("Nodes count: {nodes_count}");
        }
        Err(err) => println!("{err}")
    }
}
