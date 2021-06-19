.PHONY: build
build:
	cargo build --release

.PHONY: fuzz
fuzz:
	cargo fuzz run decode --jobs 4 -- -runs=1000
