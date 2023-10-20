SHELL := bash

SPEC = $(CURDIR)/spec.json

VERSION = $(shell cat VERSION.txt)

generate: kittycad
	cargo clippy --all
	cargo nextest run --no-capture --all-features --no-fail-fast

target/debug/openapitor: openapitor/src/*.rs openapitor/src/*/*.rs openapitor/Cargo.toml spec.json
	cargo build --bin openapitor

update: update-specs

update-specs:
	$(RM) $(SPEC)
	make $(SPEC)

$(SPEC):
	curl -sSL $(SPEC_REMOTE) -o $@

.PHONY: kittycad
kittycad: target/debug/openapitor
	./target/debug/openapitor -i $(SPEC) --target-version $(VERSION) \
		-o kittycad \
		-n kittycad \
		-d "A fully generated & opinionated API client for the KittyCAD API." \
		--spec-url "https://api.kittycad.io" \
		--base-url "https://api.kittycad.io" \
		--request-timeout-seconds 600 \
		--repo-name "KittyCAD/kittycad.rs" $(EXTRA_ARGS)
	mv -f $(CURDIR)/kittycad/kittycad.rs.patch.json $(CURDIR)

.PHONY: tag
tag: ## Create a new git tag to prepare to build a release.
	git tag -sa v$(VERSION) -m "v$(VERSION)"
	@echo "Run git push origin v$(VERSION) to push your new tag to GitHub and trigger a release."

.PHONY: help
help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | sed 's/^[^:]*://g' | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
