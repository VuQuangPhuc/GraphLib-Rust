use crate::graph::vertex::VertexId;

pub type EdgeId = usize;

#[derive(Debug)]
pub struct Edge {
    id: EdgeId,
    start: VertexId,
    end: VertexId,
}

impl Edge {
    pub fn new(id: EdgeId, start: VertexId, end: VertexId) -> Self {
        Edge {
            id,
            start,
            end,
        }
    }

    pub fn start(&self) -> &VertexId {
        &self.start
    }

    pub fn end(&self) -> &VertexId {
        &self.end
    }

    pub fn get_other_vertex(&self, vertex: &VertexId) -> &VertexId {
        if self.start == *vertex {
            &self.end
        } else {
            &self.start
        }
    }
}