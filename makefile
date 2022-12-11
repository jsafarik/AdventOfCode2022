.DEFAULT_GOAL := run

.PHONY: run
run:
	cargo run

.PHONY: test
test:
	cargo test $(ONLY)
