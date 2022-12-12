.DEFAULT_GOAL := run

.PHONY: run
run:
ifdef day
	cargo run $(day)
else
	@echo -e "You have to provide a day argument.\nExample:\nmake day=1"
endif


.PHONY: test
test:
ifdef day
	cargo test day_$(day):
else
	cargo test
endif
