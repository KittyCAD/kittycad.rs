SHELL := bash

SPEC = $(CURDIR)/spec.json

VERSION = $(shell cat VERSION.txt)

generate: kittycad
	cargo clippy --all
	cargo test --all -- --nocapture

target/debug/generator: generator/src/*.rs generator/src/*/*.rs generator/Cargo.toml spec.json
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
		-d "A fully generated & opinionated API client for the KittyCAD API." \
		--spec-url "https://api.kittycad.io" \
		--base-url "https://api.kittycad.io" \
		--repo-name "KittyCAD/kittycad.rs" $(EXTRA_ARGS)
	cargo fmt -p kittycad

.PHONY: tag
tag: ## Create a new git tag to prepare to build a release.
	git tag -sa v$(VERSION) -m "v$(VERSION)"
	@echo "Run git push origin v$(VERSION) to push your new tag to GitHub and trigger a release."

.PHONY: help
help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | sed 's/^[^:]*://g' | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
