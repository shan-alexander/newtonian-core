---
tags: [crates-io, versioning]
node_type: adr
---
# 0001 Crate identity and name collision

## Status

Accepted

## Context

The family needs a crates.io name for the *ports* crate. Directory and conversation name: `newton-core`. As of 2026-08-28, crates.io already publishes [`newton-core` 0.5.x](https://crates.io/crates/newton-core) — “newton protocol core sdk”, newt-foundation / Newton Prover AVS, zkVM, unrelated to this family.

Sibling facts:

- `newton` is a 2017 physics simulator (already avoided by `newton-machine`).
- `newton-machine` is **not** yet on crates.io (checked 2026-08-28).
- Free names (404): `newton-exec`, `newtonian-core`, `newton-ports`, `newton-program`, `newton-belief`, `newton-policy`.

`newton-machine` ADR 0001 already refused to publish the *chart* crate as `newton-core` because it would hide the family name from search. This crate is a different crate. Reusing the taken name would also collide with an unrelated SDK.

## Decision

- Directory may stay **`newton-core`** (this repo). That is not the crates.io name.
- Cargo / crates.io name: **`newtonian-core`**. Import: `newtonian_core`.
- Do not publish as `newton-core`. Do not try to take over newt-foundation’s crate.
- Do not publish a third crate `newton-exec`. See [[docs/adr/0015-two-crates-exec-is-a-module]].
- Version **`0.0.0`** was the unpublished ontology freeze. First crates.io kernel ship is **`0.1.0`**. `0.x` is not SemVer-stable.

## Consequences

- Search on crates.io for “newtonian” / “UCA” / “executive” should hit this crate next to `newton-machine`.
- `publish = false` is lifted; accidental name collision is no longer the reason to block.
- Downstream: `newtonian-core = "0.1"` (pin `0.1` while we are in `0.x`; breaking changes are allowed).

## Related

- [[docs/goals/publish-an-honest-0-0-0-crate]]
- [[docs/adr/0015-two-crates-exec-is-a-module]]
