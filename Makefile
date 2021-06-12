.PHONY: install
install:
	cargo install --force --path . --features kusama

.PHONY: clear
clear:
	rm -rf target/