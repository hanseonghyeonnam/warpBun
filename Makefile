.PHONY: help release


warpBun: ./src/main.rs
	cargo build && mv ./target/debug/warpBun ./

release:
	cargo build -r

help:
	@bash ./src/help.sh

