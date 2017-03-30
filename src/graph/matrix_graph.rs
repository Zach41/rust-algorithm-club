use std::fmt;

use super::{Vertex, Edge, VertexId};

pub struct AdjacencyMatrixGraph<T> {
    vertices: Vec<Vertex<T>>,
    weights: Vec<Vec<Option<f32>>>,
}

impl<T: fmt::Debug> fmt::Debug for AdjacencyMatrixGraph<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:?}", self.vertices)?;
        for rows in &self.weights {
            f.pad(" ")?;
            for weight_opt in rows {
                if let Some(weight) = *weight_opt {
                    write!(f, "{:.*} ", 2, weight)?;
                } else {
                    f.pad("N ")?;
                }
            }
            f.pad("\n")?;
        }
        Ok(())
    }
}

impl<T> fmt::Display for AdjacencyMatrixGraph<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for rows in &self.weights {
            f.pad(" ")?;
            for weight_opt in rows {
                if let Some(weight) = *weight_opt {
                    write!(f, "{:.*} ", 2, weight)?;
                } else {
                    f.pad("N ")?;
                }
            }
            f.pad("\n")?;
        }
        Ok(())
    }
}

impl<T> AdjacencyMatrixGraph<T> {
    pub fn new() -> AdjacencyMatrixGraph<T> {
        AdjacencyMatrixGraph {
            vertices: Vec::new(),
            weights: Vec::new(),
        }
    }

    pub fn edges(&self) -> Vec<Edge> {
        let mut out_edges = Vec::new();
        
        for (i, rows) in self.weights.iter().enumerate() {
            for (j, weight_opt) in rows.iter().enumerate() {
                if let Some(weight) = *weight_opt {
                    out_edges.push(
                        Edge {
                            from: i,
                            to: j,
                            weight: Some(weight)
                        }
                    )
                }
            }
        }
        out_edges
    }
    
    pub fn create_vertex(&mut self, data: T) -> VertexId {
        use std::iter;

        let vertex = Vertex {
            data: data,
            index: self.weights.len(),
        };
        self.vertices.push(vertex);
        
        for rows in &mut self.weights {
            rows.push(None);
        }
        let new_row = iter::repeat(None).take(self.vertices.len()).collect();
        self.weights.push(new_row);

        self.weights.len() - 1
    }

    pub fn add_directed_edge(&mut self, src: VertexId, des: VertexId, weight: Option<f32>) {
        let len = self.vertices.len();
        assert!(src < len && des < len);

        self.weights[src][des] = weight;
    }

    pub fn add_undirected_edge(&mut self, src: VertexId, des: VertexId, weight: Option<f32>) {
        self.add_directed_edge(src, des, weight);
        self.add_directed_edge(des, src, weight);
    }

    pub fn get_weight(&self, src: VertexId, des: VertexId) -> Option<f32> {
        assert!(src < self.vertices.len() && des < self.vertices.len());
        self.weights[src][des]
    }

    pub fn edges_from_src(&self, src: VertexId) -> Vec<Edge> {
        let mut out_edges = Vec::new();
        let row = &self.weights[src];

        for (idx, weight_opt) in row.iter().enumerate() {
            if let Some(weight) = *weight_opt {
                out_edges.push(
                    Edge {
                        from: src,
                        to: idx,
                        weight: Some(weight)
                    }
                );
            }
        }
        out_edges
    }

    pub fn bfs_traverse(&self, src: VertexId) -> Vec<VertexId> {
        use std::iter;
        use queue::Queue;
        let mut visited: Vec<bool> = iter::repeat(false)
            .take(self.vertices.len())
            .collect();

        let mut queue = Queue::new();
        let mut out_vertices = Vec::new();
        queue.enqueue(src);

        while !queue.is_empty() {
            let top = queue.dequeue().unwrap();
            if visited[top] {
                continue;
            }
            visited[top] = true;
            out_vertices.push(top);
            let row = &self.weights[top];
            for (idx, weight_opt) in row.iter().enumerate() {
                if weight_opt.is_some() {
                    queue.enqueue(idx);
                }
            }
        }
        out_vertices
    }

    pub fn dfs_traverse(&self, src: VertexId) -> Vec<VertexId> {
        use std::iter;
        let mut visited_flags: Vec<bool> = iter::repeat(false)
            .take(self.vertices.len())
            .collect();
        let mut visited_nodes = Vec::new();
        self._dfs(src, &mut visited_flags, &mut visited_nodes);
        visited_nodes
    }

    fn _dfs(&self, src: VertexId, visited_flags: &mut Vec<bool>, visited_nodes: &mut Vec<VertexId>) {
        if visited_flags[src] {
            return;
        }
        let row = &self.weights[src];
        visited_nodes.push(src);
        visited_flags[src] = true;
        for (idx, weight_opt) in row.iter().enumerate() {
            if weight_opt.is_some() {
                self._dfs(idx, visited_flags, visited_nodes);
            }
        }
    }

    pub fn dijkstra(&self, src: VertexId) -> Vec<f32> {
        assert!(src < self.vertices.len());
        
        use std::iter;
        use std::f32::MAX as F32MAX;
        use std::usize::MAX as USIZEMAX;
        
        let mut visited: Vec<_> = iter::repeat(false).take(self.vertices.len()).collect();
        let mut dist: Vec<_> = iter::repeat(F32MAX).take(self.vertices.len()).collect();

        for (idx, weight) in self.weights[src].iter().enumerate() {
            if let Some(weight) = *weight {
                dist[idx] = weight;
            } else {
                dist[idx] = F32MAX;
            }
        }

        dist[src] = 0f32;
        visited[src] = true;

        for _ in 0..self.vertices.len() - 1 {
            let mut min_dist = F32MAX;
            let mut min_idx = 0;

            for (idx, dis) in dist.iter().enumerate() {
                if min_dist > *dis && !visited[idx] {
                    min_idx = idx;
                    min_dist = *dis;
                }
            }
            if min_dist == F32MAX {
                return dist;
            }
            visited[min_idx] = true;
            for idx in 0..dist.len() {
                if let Some(weight) = self.weights[min_idx][idx] {
                    if dist[idx] > dist[min_idx] + weight {
                        dist[idx] = dist[min_idx] + weight;
                    }
                }
            }
        }
        dist
    }
}
