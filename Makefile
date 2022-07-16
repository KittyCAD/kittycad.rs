SHELL := bash

SPEC = $(CURDIR)/spec.json

VERSION = $(shell cat VERSION.txt)

generate: kittycad
	cargo test --all -- --nocapture
	cargo clippy --all

target/debug/generator: generator/src/*.rs generator/Cargo.toml
	cargo build -p types
	cargo build --bin generator

update: update-specs

update-specs:
	$(RM) $(SPEC)
	make $(SPEC)

$(SPEC):
	curl -sSL $(SPEC_REMOTE) -o $@

.PHONY: kittycad
kittycad: target/debug/generator
	./target/debug/generator -i $(SPEC) -v $(VERSION) \
		-o kittycad \
		-n kittycad \
		-d "A fully generated & opinionated API client for the kittycad API." \
		--spec-link "https://github.com/$(SPEC_REPO)" $(EXTRA_ARGS)
	cargo fmt -p kittycad
	cargo clippy -p kittycad --fix --allow-dirty

.PHONY: tag
tag: ## Create a new git tag to prepare to build a release.
	git tag -sa v$(VERSION) -m "v$(VERSION)"
	@echo "Run git push origin v$(VERSION) to push your new tag to GitHub and trigger a release."

.PHONY: help
help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | sed 's/^[^:]*://g' | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
