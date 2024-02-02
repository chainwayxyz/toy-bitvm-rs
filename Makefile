.PHONY: set-hook fmt fix check

set-hook: 
	chmod u+x .githooks/*
	git config core.hooksPath .githooks
	
fmt:
	cargo fmt

fix:
	cargo clippy --fix --allow-dirty --allow-staged

check:
	cargo fmt -- --check
	cargo clippy -- -D warnings

lint:
	cargo +nightly fmt --all --check
	cargo check --all-targets --all-features
	cargo clippy --all-targets --all-features

lint-fix: 
	cargo +nightly fmt --all
	cargo fix --allow-dirty
	cargo clippy --fix --allow-dirty