use crate::graph::vertex::VertexId;

pub type EdgeId = usize;

#[derive(Debug)]
pub struct Edge {
    id: EdgeId,
    start: VertexId,
    end: VertexId,
}

impl Edge {
    pub fn new(id: EdgeId, vertex_a: VertexId, vertex_b: VertexId) -> Self {
        Edge {
            id,
            start: vertex_a,
            end: vertex_b,
        }
    }

    pub fn id(&self) -> &EdgeId {
        &self.id
    }

    pub fn start(&self) -> &VertexId {
        &self.start
    }

    pub fn end(&self) -> &VertexId {
        &self.end
    }

    pub fn get_other_vertex(&self, vertex: VertexId) -> &VertexId {
        if self.start == vertex {
            &self.end
        } else {
            &self.start
        }
    }

    fn is_adjacent(&self, vertex: VertexId) -> bool {
        vertex == self.start || vertex == self.end
    }
}