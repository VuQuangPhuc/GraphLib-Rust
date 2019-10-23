use std::{fs};
use std::io::{BufReader};

type VertexId = i32;
type EdgeId = i32;

struct Vertex<'a> {
    id: VertexId,
    graph: &'a Graph,
    edges: Vec<EdgeId>,
}

impl Vertex<'_> {
    fn num_edges(&self) -> usize {
        self.edges.len()
    }

    fn add_edge(&mut self, id: EdgeId) {
        assert!(self.graph.edges[id].vertex_a == self.id || self.graph.edges[id].vertex_b == self.id);
        self.edges.push(id)
    }
}

struct Edge<'a> {
    id: EdgeId,
    vertex_a: VertexId,
    vertex_b: VertexId,
    graph: &'a Graph,
}

impl Edge<'_> {
    fn other_vertex(&self, id: VertexId) -> VertexId {
        if self.vertex_a == id {
            self.vertex_b
        } else {
            self.vertex_a
        }
    }

    fn is_adjacent(&self, id: VertexId) -> bool {
        self.vertex_a == id || self.vertex_b == id
    }
}

struct Graph {
    vertices: Vec<Vertex<'static>>,
    edges: Vec<Edge<'static>>,
}

impl <'a>Graph {
    fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    fn num_edges(&self) -> usize {
        self.edges.len()
    }

    fn add_vertex(& mut self) -> VertexId {
        let id = self.get_free_vertex_id();
        let vertex: Vertex = Vertex {
            id,
            graph: self,
            edges: Vec::new(),
        };

        self.vertices.push(vertex);
        id
    }

    fn add_edge(&mut self, vertex_a: VertexId, vertex_b: VertexId) -> EdgeId {
        let id: EdgeId = self.get_free_edge_id();
        let edge = Edge {
            id,
            vertex_a,
            vertex_b,
            graph: self,
        };

        self.edges.push(edge);
        id
    }

    fn get_free_vertex_id(&self) -> VertexId {
        self.num_vertices() as VertexId
    }

    fn get_free_edge_id(&self) -> EdgeId {
        self.edges.len() as EdgeId
    }
}

fn main() {
    let filename = "./test_data";
    let file = fs::File::open(filename)?;
    let reader = BufReader::new(file);
}
