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

.PHONY: generate
generate:
	buf generate --template buf.gen.yaml buf.build/lekkodev/cli


.PHONY: all
all: build test format lint




## Docker

# Settable
DOCKER_BINS := sidecar
DOCKER_ORG := lekko

ifneq (,$(findstring amd64,$(MAKECMDGOALS)))
    DOCKER_BUILD_EXTRA_FLAGS := --platform=linux/amd64
    DOCKER_EXTRA_TAG := amd64
endif

# TODO: check our local machine, right now this just runs on M1 Mac w/ Docker Desktop.
DOCKER_BUILD_EXTRA_FLAGS ?= --platform=linux/arm64
DOCKER_EXTRA_TAG ?= arm64


define dockerbinfunc
.PHONY: dockerbuilddeps$(1)
dockerbuilddeps$(1)::

.PHONY: dockerbuild$(1)
dockerbuild$(1): dockerbuilddeps$(1)
	docker build $(DOCKER_BUILD_EXTRA_FLAGS) -t $(DOCKER_ORG)/$(1):latest -f Dockerfile.$(1) .
ifdef EXTRA_DOCKER_ORG
	docker tag $(DOCKER_ORG)/$(1):latest $(EXTRA_DOCKER_ORG)/$(1):latest
endif
ifdef DOCKER_EXTRA_TAG
	docker tag $(DOCKER_ORG)/$(1):latest $(DOCKER_ORG)/$(1):$(DOCKER_EXTRA_TAG)
endif

dockerbuild:: dockerbuild$(1)
endef

$(foreach dockerbin,$(sort $(DOCKER_BINS)),$(eval $(call dockerbinfunc,$(dockerbin))))
