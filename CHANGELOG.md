# Changelog

All notable changes to this project are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] — 2026-08-30

Breaking: `Lifted` is no longer `Copy`; one pulse can emit several `Msg`s.

### Added

- `Lifted::Batch` / `into_msgs` / `from_msgs` (`alloc`). One pulse may lift Setup+Arm+Fire without stuffing a `Vec` into `Msg`.
- `ScoreSpec::max_age` (own-ticks, `0` = no cap) and `disarm_mask` (any already in-play score bit forces off). Fold: `max_age`, `disarm_when`.
- `PortExecutive::pulse`: lift → apply each msg → admit each cmd. No I/O, no kernel, no `revise`.
- ADR 0021.

### Changed

- `Lifted` dropped `Copy` so `Batch(Vec<M>)` can exist.

## [0.1.0] — 2026-08-30

First crates.io release of the **policy kernel**. `0.x` is not SemVer-stable.

### Added

- Policy kernel: `Key`, `ScoreSpec`, `Table`, `Folded`, `step_entity`. Exact lookup; unauthored → none. Hysteresis and enablement DAG. Criterion bench `step_entity`.
- Feature `fold`: YAML/TOML compiled **once** into `Policy` (`Catalog` of Rust kinds → exact table + knobs). No document in `step`.
- Feature `machine`: blanket `IntentionMachine` for `newton_machine::Runtime<M>` (crates.io 0.2.1).
- ADR 0020: kernel sits beside the chart (`step_entity` does not `apply`; Newton `project()` may join the key as extra bits).
- `Revision` lift signature (now + changed fact; no cloned previous store).
- `IntentionMachine::restore`, `Mandate::freshness_bound`, generic `OpenGateway<Cmd>`, `ProgramParts` associated-type bounds, `alloc` `MemoryStore`.
- GitHub-only examples: `aapl_1m` (April 2026 lake) and `mosquito` (500 ms camera; intern is `mosquito_policy.toml`).
- CI: fmt, clippy `-D warnings`, tests, docs.

### Changed

- Policy kernel is the crate **end-goal** (ADRs 0016–0019). `0.0.0` named the ports; `0.1.0` ships the kernel and Fold.

## [0.0.0] — 2026-08-28

Unpublished seed (ontology freeze).

### Added

- Seed crate **`newtonian-core`**: ports, belief/mandate/lift/gateway/skill traits.
- Optional `exec` module (named pulse; not a third crate).
- rustbrain goals, ADRs, concepts, edge cases, and v0 roadmap.
- Honest non-goals: not a desk, not a required interpreter, not BDI-CTL.

### Changed

- Cargo name `newton-core` → `newtonian-core` (crates.io `newton-core` is an unrelated zkVM SDK).
- Family is two crates (`newton-machine` + `newtonian-core`), not three. See ADR 0015.

[Unreleased]: https://github.com/shan-alexander/newtonian-core/compare/0.2.0...HEAD
[0.2.0]: https://github.com/shan-alexander/newtonian-core/releases/tag/0.2.0
[0.1.0]: https://github.com/shan-alexander/newtonian-core/releases/tag/0.1.0
[0.0.0]: https://github.com/shan-alexander/newtonian-core
