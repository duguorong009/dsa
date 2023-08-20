use std::collections::HashMap;

#[derive(Debug)]
struct Graph {
    count: usize,
    vertices: HashMap<i32, Vec<i32>>,
    weights: HashMap<(i32, i32), usize>,
}

impl Graph {
    fn new(count: usize) -> Self {
        let mut vertices = HashMap::new();
        for i in 0..count {
            vertices.insert(i as i32, Vec::new());
        }
        let weights = HashMap::new();

        Self {
            count,
            vertices,
            weights,
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

        let (v1, v2) = if start < end {
            (start, end)
        } else {
            (end, start)
        };

        self.weights
            .entry((v1, v2))
            .and_modify(|w| {
                *w = weight;
            })
            .or_insert(weight);
    }

    fn get_edge_weight(&self, v1: i32, v2: i32) -> usize {
        let (v1, v2) = if v1 < v2 { (v1, v2) } else { (v2, v1) };

        match self.weights.get(&(v1, v2)) {
            Some(w) => *w,
            None => usize::MIN,
        }
    }

    fn dijkstra(&self, start: i32) {
        let count = self.count;
        let mut visited_vertex = vec![false; count];
        let mut distance = vec![usize::MAX; count];

        // distance of self loop is zero
        distance[start as usize] = 0;
        for _ in 0..count {
            // Update the distance between neighbouring vertex and source vertex
            let u = find_min_distance(&distance, &visited_vertex) as usize;

            visited_vertex[u] = true;

            // Update all the neighbouring vertex distances
            for v in 0..count {
                let edge_u_v = self.get_edge_weight(u as i32, v as i32);
                if !visited_vertex[v] && edge_u_v != 0 && (distance[u] + edge_u_v < distance[v]) {
                    distance[v] = distance[u] + edge_u_v;
                }
            }
        }

        for i in 0..distance.len() {
            println!("Distance from {start} to {i} is {}", distance[i]);
        }
    }
}

fn find_min_distance(distance: &Vec<usize>, visited_vertex: &Vec<bool>) -> i32 {
    let mut min_dist = usize::MAX;
    let mut min_dist_vertex = -1;
    for i in 0..distance.len() {
        if !visited_vertex[i] && distance[i] < min_dist {
            min_dist = distance[i];
            min_dist_vertex = i as i32;
        }
    }

    min_dist_vertex
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_graph() {
        let v_count = 3;
        let graph = Graph::new(v_count);
        assert!(graph.count == v_count);
        assert!(graph.vertices.keys().len() == v_count);
    }

    #[test]
    fn test_add_edge() {
        let v_count = 6;
        let mut graph = Graph::new(v_count);
        graph.add_edge(0, 1, 4);
        graph.add_edge(0, 2, 4);
        graph.add_edge(1, 2, 2);
        graph.add_edge(2, 3, 3);
        graph.add_edge(2, 4, 6);
        graph.add_edge(2, 5, 1);
        graph.add_edge(3, 4, 2);
        graph.add_edge(4, 5, 3);

        assert!(graph.count == v_count);
        assert!(graph.vertices.get(&0).unwrap().len() == 2);
        assert!(graph.vertices.get(&1).unwrap().len() == 2);
        assert!(graph.vertices.get(&2).unwrap().len() == 5);
        assert!(graph.vertices.get(&3).unwrap().len() == 2);
        assert!(graph.vertices.get(&4).unwrap().len() == 3);
        assert!(graph.vertices.get(&5).unwrap().len() == 2);
    }

    #[test]
    fn test_dijkstra() {
        let v_count = 6;
        let mut graph = Graph::new(v_count);
        graph.add_edge(0, 1, 4);
        graph.add_edge(0, 2, 4);
        graph.add_edge(1, 2, 2);
        graph.add_edge(2, 3, 3);
        graph.add_edge(2, 4, 6);
        graph.add_edge(2, 5, 1);
        graph.add_edge(3, 4, 2);
        graph.add_edge(4, 5, 3);

        graph.dijkstra(0);
    }
}
