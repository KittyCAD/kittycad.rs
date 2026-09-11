# CPU modeling sessions

The public API selects CPU engines with `pool: Some("cpu".into())` and
`webrtc: Some(false)`. The account must have the `cpu_engine_pool` feature enabled.
Without effective CPU access, the API falls back to the default GPU pool.
A successful WebSocket connection or pong does not establish CPU placement;
verify the actual selected pool in server routing logs.

```rust
let params = kittycad::modeling::CommandsWsParams {
    pool: Some("cpu".into()),
    webrtc: Some(false),
    ..Default::default()
};
```

The API derives geometry-only mode from the selected pool, including GPU fallback.
The existing `geometry_only` field is retained for source compatibility, but is
not a public CPU routing switch. Omit it when selecting CPU through the API.
Explicit `pool: Some("default".into())`, or no pool, retains the default GPU route.

Existing exhaustive struct initializers that predate the `geometry_only` field
must include it or use `..Default::default()`; this guidance does not remove any
fields or introduce a new signature change.

CPU sessions currently have reduced rendering capabilities. Omit rendering
options and validate the geometry commands and exports required by your workload.
A paired protocol ping trial confirms connectivity, not geometry/rendering parity
or a performance distribution. API #4622 defines this pool-based routing contract;
API #4601 is not a prerequisite.
