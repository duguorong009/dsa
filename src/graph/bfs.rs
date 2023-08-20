use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct Graph {
    count: usize,
    vertices: HashMap<i32, Vec<i32>>,
}

impl Graph {
    fn new(count: usize) -> Self {
        let mut vertices = HashMap::new();
        for i in 0..count {
            vertices.insert(i as i32, Vec::new());
        }

        Self { count, vertices }
    }

    fn add_edge(&mut self, v: i32, w: i32) {
        self.vertices
            .entry(v)
            .and_modify(|n| n.push(w))
            .or_insert(vec![w]);
    }

    fn bfs(&self, start: i32) -> Vec<i32> {
        // Prepare the "visited" & "queue"
        let mut visited = Vec::new();
        let mut queue = VecDeque::new();

        visited.push(start);

        let adjacents = match self.vertices.get(&start) {
            Some(v) => v.clone(),
            None => vec![],
        };
        for n in adjacents {
            queue.push_back(n);
        }

        // BFS
        while !queue.is_empty() {
            let head = queue.pop_front().unwrap();
            visited.push(head);

            let adjacents = match self.vertices.get(&head) {
                Some(v) => v.clone(),
                None => vec![],
            };
            for n in adjacents {
                if !visited.contains(&n) && !queue.contains(&n) {
                    queue.push_back(n);
                }
            }
        }

        visited
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_graph() {
        let vertices_cnt = 3;
        let graph = Graph::new(vertices_cnt);
        assert!(graph.count == vertices_cnt);
        assert!(graph.vertices.keys().len() == vertices_cnt);
    }

    #[test]
    fn test_add_edge() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);
        graph.add_edge(2, 3);
        graph.add_edge(3, 3);

        assert!(graph.count == 4);
        assert!(graph.vertices.get(&0).unwrap().len() == 2);
        assert!(graph.vertices.get(&1).unwrap().len() == 1);
        assert!(graph.vertices.get(&2).unwrap().len() == 2);
        assert!(graph.vertices.get(&3).unwrap().len() == 1);
    }

    #[test]
    fn test_bfs() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);
        graph.add_edge(2, 3);
        graph.add_edge(3, 3);

        let visited = graph.bfs(0);
        let expected = vec![0, 1, 2, 3];
        assert!(visited == expected);
    }
}
