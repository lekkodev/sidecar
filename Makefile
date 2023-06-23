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
	rm -r src/gen/proto/cli
	buf generate buf.build/lekkodev/cli --template templates/buf.gen.cli.yaml --path lekko/backend --path lekko/feature --path lekko/rules
	rm -r src/gen/proto/sdk
	buf generate buf.build/lekkodev/sdk --template templates/buf.gen.sdk.yaml

.PHONY: all
all: build test format lint




## Docker

# Settable
DOCKER_BINS := sidecar
DOCKER_ORG := lekko
DOCKER_REMOTE := 525250420071.dkr.ecr.us-east-1.amazonaws.com

# TODO: check our local machine, right now this just runs on M1 Mac w/ Docker Desktop.

define dockerbinfunc
.PHONY: dockerbuilddeps$(1)
dockerbuilddeps$(1)::

.PHONY: dockerbuildlocal$(1)
dockerbuildlocal$(1): dockerbuilddeps$(1)
	docker build -t $(DOCKER_ORG)/$(1):latest -f Dockerfile.$(1) .

.PHONY: dockerbuild$(1)
dockerbuild$(1): dockerbuilddeps$(1)
	docker build -t $(DOCKER_REMOTE)/$(DOCKER_ORG)/$(1):amd64 -f Dockerfile.$(1) --platform=linux/amd64 .

.PHONY: dockerpush$(1)
dockerpush$(1): dockerbuilddeps$(1)
# TODO: some main branch protection, check if there are any local changes, etc.
	$(eval GIT_HASH := $(shell git rev-parse main))
	$(eval DATE := $(shell date +'%Y-%m-%d'))
	$(eval TAG := $(DATE)_$(GIT_HASH))
	@read -p "Do you want to create and push a git tag in this format: $(TAG) [Press any key to continue]: "
	git tag -f $(TAG)
	docker build -t $(DOCKER_REMOTE)/$(DOCKER_ORG)/$(1):$(TAG) -f Dockerfile.$(1) --platform=linux/amd64 .

	@read -p "Do you want to push this image: $(DOCKER_REMOTE)/$(DOCKER_ORG)/$(1):$(TAG) [Press any key to continue]: "
	aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin $(DOCKER_REMOTE)
	docker push $(DOCKER_REMOTE)/$(DOCKER_ORG)/$(1):$(TAG)

	@read -p "Do you want to push this image as latest in $(DOCKER_REMOTE)/$(DOCKER_ORG)/$(1)? If this is a test build, feel free to Ctrl+C now. [Press any key to continue]: "

	@$(MAKE) dockertaglatest$(1)

	@read -p "Do you want to push this tag to main: $(TAG) [Press any key to continue]: "
	git push origin --tags

dockertaglatest$(1):
	$(eval GIT_HASH := $(shell git rev-parse HEAD))
	$(eval DATE := $(shell date +'%Y-%m-%d'))
	$(eval TAG := $(DATE)_$(GIT_HASH))
	$(eval MANIFEST := $(shell aws ecr batch-get-image --region us-east-1 --repository-name $(DOCKER_ORG)/$(1) --image-ids imageTag=$(TAG) --query 'images[].imageManifest' --output text))
	aws ecr put-image --region us-east-1 --repository-name $(DOCKER_ORG)/$(1) --image-tag latest --image-manifest '${MANIFEST}'

dockerbuildlocal:: dockerbuildlocal$(1)
dockerbuild:: dockerbuild$(1)
# Intentionally don't create a grouped dockerpush.
endef

$(foreach dockerbin,$(sort $(DOCKER_BINS)),$(eval $(call dockerbinfunc,$(dockerbin))))

release:
	./release.sh
