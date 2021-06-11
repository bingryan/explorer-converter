.PHONY: install
install:
	cargo install --force --path .

.PHONY: clear
clear:
	rm -rf target/