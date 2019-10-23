use crate::graph::vertex::VertexId;

pub type EdgeId = usize;

#[derive(Debug)]
pub struct Edge {
    id: EdgeId,
    vertex_a: VertexId,
    vertex_b: VertexId,
}

impl Edge {
    pub fn new(id: EdgeId, vertex_a: VertexId, vertex_b: VertexId) -> Self {
        Edge {
            id,
            vertex_a,
            vertex_b,
        }
    }

    pub fn id(&self) -> &EdgeId {
        &self.id
    }

    pub fn vertex_a(&self) -> &VertexId {
        &self.vertex_a
    }

    pub fn vertex_b(&self) -> &VertexId {
        &self.vertex_b
    }

    fn get_other_vertex(&self, vertex: VertexId) -> &VertexId {
        if self.vertex_a == vertex {
            &self.vertex_b
        } else {
            &self.vertex_a
        }
    }

    fn is_adjacent(&self, vertex: VertexId) -> bool {
        vertex == self.vertex_a || vertex == self.vertex_b
    }
}