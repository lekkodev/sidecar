# TODO: we should generalize this for a rust focused makefile system.

# We could add things like licenses as well, will ignore for now.
# References:
# https://github.com/bufbuild/protobuf-es/blob/main/Makefile
# https://tech.davis-hansson.com/p/make/
SHELL := bash
.DELETE_ON_ERROR:
.SHELLFLAGS := -eu -o pipefail -c
.DEFAULT_GOAL := all
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules
MAKEFLAGS += --no-print-directory

## Some defaults from buf makefiles for ease of use.

.PHONY: help
help: ## Describe useful make targets
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "%-30s %s\n", $$1, $$2}'


.PHONY: clean
clean: ## Delete build artifacts and installed dependencies
	@# -X only removes untracked files, -d recurses into directories, -f actually removes files/dirs
	git clean -Xdf

## Rust specific

.PHONY: build
build:
	cargo build

.PHONY: test
test:
	cargo test

.PHONY: format
format: 
	cargo fmt

.PHONY: lint
lint:
	cargo clippy --all-targets --all-features -- -D warnings


.PHONY: all
all: build test format lint
