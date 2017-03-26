use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex<T> {
    pub data: T,
    index: usize,
}

pub type VertexId = usize;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Edge {
    pub from: VertexId,
    pub to: VertexId,
    pub weight: Option<f32>,
}

mod list_graph;
mod matrix_graph;

pub use self::matrix_graph::AdjacencyMatrixGraph;

#[cfg(test)]
mod test;
