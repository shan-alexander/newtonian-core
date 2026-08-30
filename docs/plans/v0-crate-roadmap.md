---
tags: [roadmap, 0.1.0]
node_type: plan
status: in_progress
---
# v0 crate roadmap

## Status

in_progress

## Intent

Land `newtonian-core` as an honest `0.0.0` ports crate, then grow it into the **policy kernel** ([[docs/goals/configuration-indexed-policy-kernel]]) without a third crates.io name and without folding sleeves into `newton-machine`.

## Backlog

- [x] Choose crates.io name `newtonian-core`. See [[docs/adr/0001-crate-identity-and-name-collision]].
- [x] Name the policy-kernel product in rustbrain (ADRs 0016–0019). See [[docs/adr/0016-newtonian-core-is-the-policy-kernel]].
- [x] Kernel sits beside the chart (ADR 0020). See [[docs/adr/0020-kernel-sits-beside-the-chart]].
- [x] `Key` newtype + exact `Table<Key, T>` (ROM). Unauthored → none.
- [x] Hysteretic score eval with per-score clocks + enablement DAG (Rust stages).
- [x] Handwritten `Folded { specs, table }` (YAML Fold later; no YAML in `step`).
- [x] Fold YAML/TOML → `Policy` **once** (feature `fold`; no YAML in `step`).
- [x] `step_entity` Criterion bench (kernel budget, not desk SLOs).
- [x] Memory `BeliefStore` (`alloc`, `BTreeMap`).
- [x] Adapter `impl IntentionMachine for newton_machine::Runtime<M>` (feature `machine`).
- [ ] `exec` pulse helper, still no sockets. See [[docs/adr/0015-two-crates-exec-is-a-module]].
- [x] GitHub examples: `aapl_1m` + `mosquito` (host Executive; feature `machine` adapter).
- [ ] File-backed program snapshot *example* in a host, not in core.

## In Progress

- [x] Honest `0.1.0` crates.io crate: ports + kernel + Fold + `machine` adapter

## QA

- [x] `cargo test` (default, `--no-default-features` lib, `serde`)
- [x] `cargo clippy --all-targets -- -D warnings` (CI)
- [ ] rustbrain `doctor` clean after this slice

## Done

- [x] Family sentence and five-way split in README
- [x] Founding goals, ADRs, concepts, edge cases
- [x] Traits: IntentionMachine, BeliefStore, Mandate, Lift, Gateway, Skill, ProgramParts
- [x] Session-protocol host test (not a desk)
- [x] Stance host test (exact ROM + extra bits)
- [x] Cargo name `newtonian-core` (not the taken `newton-core`)
- [x] Two-crate family; `exec` is a module (ADR 0015)
- [x] Handwritten kernel: `step_entity`, scores, exact table

## Cancelled

- [~] Batteries-included Executive in this crate
- [~] BDI-CTL
- [~] YAML chart loader
- [~] Domain vocabulary in core

## Blocked

- (none)

## Priority / order

1. Keep `0.0.0` ports stable enough that the kernel can be written against them.
2. Implement the kernel in **this** crate (exact ROM, scores, Fold), not in `newton-machine`.
3. Do not add a *required* `run()`.
4. Do not add trading types. Clocks are tick / frame / sample.
5. Do not publish `newton-exec` as a third crate.
6. Do not default to longest-subset match.

## Out of scope

- Sockets, ROS, DOM, FIX.
- Visual chart editor.
- Claiming `1.0`.

## Related

- [[docs/goals/non-goals-of-newton-core]]
- [[changelog]]
- [[docs/adr/0001-crate-identity-and-name-collision]]
- [[docs/adr/0020-kernel-sits-beside-the-chart]]
