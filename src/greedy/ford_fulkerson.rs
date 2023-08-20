use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct Graph {
    count: usize,
    vertices: HashMap<i32, Vec<i32>>,
    capacities: HashMap<(i32, i32), usize>,
}

impl Graph {
    pub fn new(count: usize) -> Self {
        let mut vertices = HashMap::new();
        for i in 0..count {
            vertices.insert(i as i32, Vec::new());
        }
        let capacities = HashMap::new();

        Self {
            count,
            vertices,
            capacities,
        }
    }

    fn add_edge(&mut self, start: i32, end: i32, weight: usize) {
        self.vertices
            .entry(start)
            .and_modify(|connected| {
                if !connected.contains(&end) {
                    connected.push(end);
                }
            })
            .or_insert(vec![end]);

        self.vertices
            .entry(end)
            .and_modify(|connected| {
                if !connected.contains(&start) {
                    connected.push(start);
                }
            })
            .or_insert(vec![start]);

        self.capacities
            .entry((start, end))
            .and_modify(|w| {
                *w = weight;
            })
            .or_insert(weight);
    }

    fn get_edge_capacity(&self, v1: i32, v2: i32) -> usize {
        match self.capacities.get(&(v1, v2)) {
            Some(w) => *w,
            None => 0,
        }
    }

    fn searching_algo_bfs(
        &self,
        graph: &Vec<Vec<usize>>,
        s: i32,
        t: i32,
        parent: &mut Vec<i32>,
    ) -> bool {
        let mut visited = vec![false; self.count];
        let mut queue = VecDeque::new();

        queue.push_back(s);
        visited[s as usize] = true;

        while !queue.is_empty() {
            let u = queue.pop_front().unwrap();

            let neighbors = match self.vertices.get(&u) {
                Some(n) => n.clone(),
                None => vec![],
            };
            for v in neighbors {
                let capacity = graph[u as usize][v as usize];
                if !visited[v as usize] && capacity > 0 {
                    queue.push_back(v);
                    visited[v as usize] = true;
                    parent[v as usize] = u;
                }
            }
        }

        if visited[t as usize] {
            true
        } else {
            false
        }
    }

    // Applying the fordfulkerson algorithm
    fn ford_fulkerson(&self, source: i32, sink: i32) -> usize {
        let mut graph: Vec<Vec<usize>> = vec![vec![0; self.count]; self.count];
        for u in 0..self.count {
            for v in 0..self.count {
                graph[u][v] = self.get_edge_capacity(u as i32, v as i32);
            }
        }

        let mut parent = vec![-1; self.count];
        let mut max_flow = 0;

        while self.searching_algo_bfs(&graph, source, sink, &mut parent) {
            let mut path_flow = usize::MAX;
            let mut s = sink;
            while s != source {
                let edge_capacity = graph[parent[s as usize] as usize][s as usize];
                path_flow = path_flow.min(edge_capacity);
                s = parent[s as usize];
            }

            // Adding the path flows
            max_flow += path_flow;

            // Updating the residual values of edges
            let mut v = sink;
            while v != source {
                let u = parent[v as usize];
                graph[u as usize][v as usize] -= path_flow;
                graph[v as usize][u as usize] += path_flow;
                v = parent[v as usize];
            }
        }

        max_flow
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_edge() {
        let mut graph = Graph::new(6);

        graph.add_edge(0, 1, 8);
        graph.add_edge(0, 4, 3);
        graph.add_edge(1, 2, 9);
        graph.add_edge(2, 4, 7);
        graph.add_edge(2, 5, 2);
        graph.add_edge(3, 5, 5);
        graph.add_edge(4, 2, 7);
        graph.add_edge(4, 3, 4);

        println!("graph: {graph:?}");
    }

    #[test]
    fn test_fordfulkerson() {
        let mut graph = Graph::new(6);

        graph.add_edge(0, 1, 8);
        graph.add_edge(0, 4, 3);
        graph.add_edge(1, 2, 9);
        graph.add_edge(2, 4, 7);
        graph.add_edge(2, 5, 2);
        graph.add_edge(3, 5, 5);
        graph.add_edge(4, 2, 7);
        graph.add_edge(4, 3, 4);

        let max_flow = graph.ford_fulkerson(0, 5);
        assert!(max_flow == 6);
    }
}
