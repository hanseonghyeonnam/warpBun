.PHONY: help release clean

clean:
	rm -rf target/ target

warpBun: ./src/main.rs
	cargo build && mv ./target/debug/warpBun ./

release:
	cargo build -r && mv ./target/release/warpBun ./
help:
	@chmod +x ./src/help.sh
	@bash ./src/help.sh

