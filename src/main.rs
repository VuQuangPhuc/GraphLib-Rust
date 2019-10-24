use std::{env, io};
use std::fs::File;

mod graph;

fn main() -> io::Result<()> {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let operation = args[0].clone();
    args.drain(0..1);

    println!("{}", operation);

    for f in args {
        println!("---------------------------------------");
        let file = File::open(f)?;
        let graph = graph::create_graph(file);
        graph.find_strong_connections();
    }

    Ok(())
}
