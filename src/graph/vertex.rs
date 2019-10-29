use crate::graph::edge::{EdgeId};

pub type VertexId = usize;

#[derive(Debug)]
pub struct Vertex {
    id: VertexId,
    edges: Vec<EdgeId>,
}

impl Vertex {
    pub fn new(id: VertexId, edges: Vec<EdgeId>) -> Self {
        Vertex {
            id,
            edges,
        }
    }

    pub fn id(&self) -> &VertexId {
        &self.id
    }

    pub fn edges(&self) -> &Vec<EdgeId> {
        &self.edges
    }

    pub fn add_edge(&mut self, edge: EdgeId) -> () {
        self.edges.push(edge);
    }
}