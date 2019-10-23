use std::{env, io};
use std::fs::File;

fn main() -> io::Result<()> {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let operation = args[0].clone();
    args.drain(0..1);

    println!("{}", operation);

    for f in args {
        println!("---------------------------------------");
        println!("File: {:?}\n", f);
        let file = File::open(f)?;
        graph::create_graph(file);
    }

    Ok(())
}

mod graph {
    use std::fs::File;
    use std::io;
    use std::io::{BufRead, BufReader};
    use std::time::Instant;

    pub fn create_graph(file: File) -> Graph {
        let now = Instant::now();
        let mut graph: Graph = Graph {
            vertices: Vec::new(),
            edges: Vec::new(),
        };
        graph.read_from_file(&file);
        println!("{}", now.elapsed().as_micros());

        graph
    }

    type VertexId = usize;
    type EdgeId = usize;

    #[derive(Debug)]
    struct Vertex {
        id: VertexId,
        edges: Vec<EdgeId>,
    }

    impl Vertex {
        fn num_edges(&self) -> usize {
            self.edges.len()
        }

        fn get_edge(&self, edge_id: EdgeId) -> EdgeId {
            assert!(self.edges.len() > edge_id);
            self.edges[edge_id]
        }
    }

    #[derive(Debug)]
    struct Edge {
        id: EdgeId,
        vertex_a: VertexId,
        vertex_b: VertexId,
    }

    impl Edge {
        fn get_other_vertex(&self, vertex: VertexId) -> VertexId {
            if self.vertex_a == vertex {
                self.vertex_b
            } else {
                self.vertex_a
            }
        }

        fn is_adjacent(&self, vertex: VertexId) -> bool {
            vertex == self.vertex_a || vertex == self.vertex_b
        }
    }

    #[derive(Debug)]
    pub struct Graph {
        vertices: Vec<Vertex>,
        edges: Vec<Edge>,
    }

    impl Graph {
        fn read_from_file(&mut self, file: &File) -> () {
            let reader = BufReader::new(file);

            let mut first = true;
            for line in reader.lines().filter_map(io::Result::ok) {
                if first {
                    let count: i32 = line.parse().unwrap();
                    for _ in 0..count {
                        self.add_vertex();
                    }
                    first = false;
                } else {
                    let vertices = line.trim().split(' ').flat_map(str::parse::<VertexId>).collect::<Vec<_>>();
                    self.add_edge(vertices[0], vertices[1]);
                }
            }
        }

        fn add_vertex(& mut self) -> VertexId {
            let id = self.get_free_vertex_id();
            let vertex = Vertex {
                id,
                edges: Vec::new(),
            };
            self.vertices.push(vertex);

            id
        }

        fn add_edge(&mut self, vertex_a: VertexId, vertex_b: VertexId) -> EdgeId {
            let id = self.get_free_edge_id();
            let edge = Edge {
                id,
                vertex_a,
                vertex_b,
            };
            self.vertices[vertex_a].edges.push(id);
            self.vertices[vertex_b].edges.push(id);
            self.edges.push(edge);

            id
        }

        fn get_free_vertex_id(&self) -> VertexId {
            self.vertices.len() as VertexId
        }

        fn get_free_edge_id(&self) -> EdgeId {
            self.edges.len() as EdgeId
        }
    }
}
