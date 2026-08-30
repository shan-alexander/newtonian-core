---
tags: [crates-io, versioning, goals]
node_type: goal
---
# Publish an honest 0.0.0 crate

`0.0.0` meant: the ontology is named, the traits compile, the loop is not claimed, SemVer is not claimed.

`0.1.0` (2026-08-30) is the first crates.io kernel: ports + `step_entity` + Fold + optional `machine` adapter. `0.x` is still not SemVer-stable. The loop (`exec`) is still not claimed as required.

crates.io already has a crate named `newton-core` (newt-foundation zkVM SDK, unrelated). This crate publishes as **`newtonian-core`**. See [[docs/adr/0001-crate-identity-and-name-collision]]. There is no `newton-exec` crate ([[docs/adr/0015-two-crates-exec-is-a-module]]).

## Related

- [[docs/adr/0001-crate-identity-and-name-collision]]
- [[docs/plans/v0-crate-roadmap]]
