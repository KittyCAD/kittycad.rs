# Geometry-only modeling sessions (proposed)

`CommandsWsParams` gains `geometry_only: Option<bool>`. Pass `Some(true)`
along with `webrtc: Some(false)` only for workloads that never need images
or video. Omission and `Some(false)` preserve rendered-session intent.

Existing exhaustive struct initializers must add `geometry_only: None` or
use `..Default::default()`. Review release compatibility before publication.
The binding does not synthesize SSAO or video dimensions.

Deploy the explicit API contract before enabling callers. CPU routing also
depends on the server rollout flag and pool availability; the option does
not guarantee CPU placement. Require matched geometry/export parity and
verified CPU routing before migrating CI beyond a bounded canary.
