build:
	cargo build --release

run:
	./target/release/graph_lib_rust ${mode} ${target}
