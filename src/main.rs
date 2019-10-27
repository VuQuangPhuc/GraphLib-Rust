use std::{env, io};

mod graph;

fn main() -> io::Result<()> {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let operation = args[0].clone();
    args.drain(0..1);

    println!("{}", operation);
    stack::huge_g(operation, args);

    Ok(())
}

mod stack {
    use std::thread;
    use std::fs::File;

    use crate::graph;
    use crate::graph::Graph;
    use crate::graph::ugraph::UndirectedGraph;

    pub fn huge_g(mode: String, args: Vec<String>) -> () {
        let builder = thread::Builder::new()
            .name("STACKSSSS".into())
            .stack_size(2048 * 1024 * 1024);

        let handler = builder.spawn(move || {
            if &mode == "undirected" {
                for f in args {
                    println!("---------------------------------------");
                    let file = File::open(f).unwrap();
                    let graph: UndirectedGraph = graph::create_undirected_graph(file);
                    graph.find_strong_connected_components();
                }
            }
        }).unwrap();

        handler.join().unwrap();
    }
}
