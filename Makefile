.PHONY: dev watch clean

dev:
	cargo build

watch:
	cargo watch -x "build -p lark"

clean:
	cargo clean
