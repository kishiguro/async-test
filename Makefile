all:

doc:
	cargo doc --open

example:
	cargo run --example

build:
	cargo build --example 11-select-connect

run:
	cargo run --example 11-select-connect
