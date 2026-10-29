# kittycad.rs 

The Rust API client for KittyCAD.

- [Rust docs](https://docs.rs/kittycad)
- [KittyCAD API Docs](https://zoo.dev/docs/api?lang=rust)

## Generating

You can trigger a build with the GitHub action to generate the client. This will
automatically update the client to the latest version based on the spec
at [spec.json](spec.json).

Alternatively, if you wish to generate the client locally, run:

```bash
$ make generate
```

## Contributing

Please do not change the code directly since it is generated. PRs that change
the code directly will be automatically closed by a bot.


## Windows TLS smoke test

To confirm the client honors certificates from the Windows root store (for
example when routing requests through a corporate MITM proxy), opt into the
smoke test:

```bash
WIN_CA_SMOKE=1 cargo test --test win_ca_smoke
```

CI runs provision certificates and the HTTPS test harness via the shared
`kittycad/gh-action-win-ca` workflow. If you need to reproduce the handshake
locally on Windows, follow the helper instructions in that repository.

The test defaults to hitting `https://localhost:4443/` and expects an `ok`
response body. Override `SMOKE_URL`, `SMOKE_ATTEMPTS`, or `SMOKE_DELAY_MS` if
your proxy uses a different endpoint or boot time.


### Releasing a new kittycad version

1. Make sure the `VERSION.txt` has the new version you want to release.
2. Make sure you have run `make generate` and pushed any changes. The release
   will fail if running `make generate` causes any changes to the generated
   code.
3. Run `make tag` this is just an easy command for making a tag formatted
   correctly with the version.
4. Push the tag (the result of `make tag` gives instructions for this)
5. Everything else is triggered from the tag push. Just make sure all the tests
   pass on the `main` branch before making and pushing a new tag.

### Releasing openapitor-runtime

`openapitor-runtime` is versioned and published independently of `kittycad` and
`openapitor`. Its version comes from `openapitor-runtime/Cargo.toml`, rather than
`VERSION.txt`.

1. Set the version in `openapitor-runtime/Cargo.toml` and run
   `cargo test -p openapitor-runtime` to test it and update `Cargo.lock`.
2. Commit and push the changes, and make sure the tests pass on `main`.
3. Run `just tag-runtime` to create a signed tag such as
   `openapitor-runtime-v0.1.0`.
4. Push the tag using the command printed by `just tag-runtime`.
5. The release workflow checks that the tag matches the manifest version, runs
   clippy and tests for the runtime, publishes only `openapitor-runtime` to
   crates.io, and creates a GitHub release.

The workflow uses the existing `CARGO_TOKEN` repository secret, which must have
permission to publish `openapitor-runtime`.

Publish the runtime version before releasing either dependent crate. When
updating the runtime dependency, update `openapitor/Cargo.toml` and the generated
manifest template in `openapitor/src/lib.rs`, then run `make generate` to update
`kittycad/Cargo.toml`. The workspace's `[patch.crates-io]` entry uses the local
runtime for development; published crates use the dependency from crates.io.
