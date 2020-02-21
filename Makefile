all:

doc:
	cargo doc --open

example:
	cargo run --example

build:
	cargo build --example 08-connect

run:
	cargo run --example 08-connect
