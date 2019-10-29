build:
	cargo build --release

release:
	cargo build --release

run:
	./target/release/graph_lib_rust ${mode} ${target}
