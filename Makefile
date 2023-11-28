.PHONY: set-hook fmt fix check

set-hook: 
	git config core.hooksPath .githooks
	
fmt:
	cargo fmt

fix:
	cargo clippy --fix --allow-dirty --allow-staged

check:
	cargo fmt -- --check
	cargo clippy -- -D warnings