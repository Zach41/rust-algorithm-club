use std::clone::Clone;
use std::fmt;

use super::{Vertex, Edge, VertexId};

pub struct AdjacencyListGraph<T> {
    vertices: Vec<Vertex<T>>,
    edge_list: Vec<Vec<Edge>>,
}

impl<T> fmt::Display for AdjacencyListGraph<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (idx, edges) in self.edge_list.iter().enumerate() {
            if edges.is_empty() {
                continue;
            }
            try!(write!(f, "{}:", idx));
            for edge in edges {
                try!(write!(f, " ->{}({:.*})", edge.to, 2, edge.weight.unwrap()));
            }
            try!(writeln!(f));
        }
        Ok(())
    }
}

impl<T> AdjacencyListGraph<T> {
    pub fn new() -> AdjacencyListGraph<T> {
        AdjacencyListGraph {
            vertices: Vec::new(),
            edge_list: Vec::new(),
        }
    }

    pub fn edges(&self) -> Vec<Edge> {
        let mut out_edges = Vec::new();
        
        for edges in &self.edge_list {
            out_edges.extend_from_slice(edges);
        }
        out_edges.dedup();
        out_edges
    }

    pub fn create_vertex(&mut self, data: T) -> VertexId {
        let idx = self.vertices.len();
        let vertex = Vertex {
            data: data,
            index: idx
        };
        self.vertices.push(vertex);
        self.edge_list.push(Vec::new());

        idx
    }

    pub fn add_directed_edge(&mut self, src: VertexId, des: VertexId, weight: Option<f32>) {
        assert!(src < self.vertices.len() && des < self.vertices.len());

        let edge = Edge {
            from: src,
            to: des,
            weight: weight
        };
        self.edge_list[src].push(edge);
    }

    pub fn add_undirected_edge(&mut self, src: VertexId, des: VertexId, weight: Option<f32>) {
        self.add_directed_edge(src, des, weight);
        self.add_directed_edge(des, src, weight);
    }

    pub fn get_weight(&self, src: VertexId, des: VertexId) -> Option<f32> {
        assert!(self.vertices.len() > src && des < self.vertices.len());

        let edges = &self.edge_list[src];

        let mut matched: Vec<_> = edges.into_iter().filter(|edge| edge.to == des).collect();

        matched.pop().and_then(|&e| e.weight)
    }

    pub fn edges_from_src(&self, src: VertexId) -> Vec<Edge> {
        assert!(src < self.vertices.len());

        self.edge_list[src].clone()
    }
    
    pub fn bfs_traverse(&self, src: VertexId) -> Vec<VertexId> {
        use queue::Queue;
        use std::iter;

        let mut visited: Vec<_> = iter::repeat(false).
            take(self.vertices.len()).collect();
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
            for edge in &self.edge_list[top] {
                queue.enqueue(edge.to);
            }
        }
        out_vertices
    }

    pub fn dfs_traverse(&self, src: VertexId) -> Vec<VertexId> {
        use std::iter;
        
        let mut visited: Vec<_> = iter::repeat(false).take(self.vertices.len()).collect();
        let mut out_vertices = Vec::new();
        self._dfs_traverse(src, &mut visited, &mut out_vertices);
        out_vertices
    }

    pub fn _dfs_traverse(&self, src: VertexId, visited: &mut Vec<bool>, out_vertices: &mut Vec<VertexId>) {
        if visited[src] {
            return;
        }
        visited[src] = true;
        out_vertices.push(src);
        for edge in &self.edge_list[src] {
            self._dfs_traverse(edge.to, visited, out_vertices);
        }
    }

    pub fn dijkstra(&self, src: VertexId) -> Vec<f32> {
        use std::iter;
        use std::f32::MAX as F32MAX;
        
        let mut visited: Vec<_> = iter::repeat(false).take(self.vertices.len()).collect();
        let mut dist: Vec<_> = iter::repeat(F32MAX).take(self.vertices.len()).collect();

        for edge in &self.edge_list[src] {
            if let Some(weight) = edge.weight {
                dist[edge.to] = weight;
            }
        }
        dist[src] = 0f32;
        visited[src] = true;

        for _ in 0..self.vertices.len() - 1 {
            let mut min_dist = F32MAX;
            let mut min_idx = 0;

            for (idx, dis) in dist.iter().enumerate() {
                if min_dist > *dis && !visited[idx] {
                    min_dist = *dis;
                    min_idx = idx;
                }
            }
            if min_dist == F32MAX {
                return dist;
            }
            visited[min_idx] = true;
            for edge in &self.edge_list[min_idx] {
                if let Some(weight) = edge.weight {
                    if dist[edge.to] > dist[min_idx] + weight {
                        dist[edge.to] = dist[min_idx] + weight;
                    }
                }
            }
        }
        dist
    }
}
