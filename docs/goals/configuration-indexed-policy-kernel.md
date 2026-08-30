---
tags: [goal, kernel, product]
node_type: goal
aliases: [policy kernel goal]
---
# Configuration-indexed policy kernel

End-goal of `newtonian-core`: publish a **synchronous, gated, configuration-indexed policy kernel** that systems engineers (Quant desks, games, 10 ms robots, session hosts) can share **without** sharing a broker, a renderer, or a lake — and without pretending Newton’s chart is the sleeve product.

0.0.0 named the ports. This goal is what those ports are *for*. See [[docs/adr/0016-newtonian-core-is-the-policy-kernel]].

## Goals

- One `step_entity` (or equivalent): beliefs → gated scores → exact bitset key → ROM lookup → `Cmd`. Same function live / replay / test.
- YAML/TOML after Fold may change knobs and which chords exist. IR, XOR children, behavior *kinds*, and the step law stay Rust ([[docs/adr/0018-fold-yaml-once-never-interpret-topology]]).
- Exact key. Unauthored → NONE. No longest-subset default ([[docs/adr/0017-exact-superstate-key-not-longest-subset]]).
- Hysteretic scores with their own clocks; not Harel history ([[docs/adr/0019-gated-hysteretic-scores-are-not-harel-history]]).
- Enablement DAG; skip disabled gates ([[docs/concepts/enablement-dag]]).
- `no_std` + `alloc` hot path; no string ids; no I/O in `step`; Criterion on `step` belongs in this crate.
- Newton optional: `IntentionMachine` for session/order/clip XOR; category-change `apply` only.
- Domain-agnostic clocks: tick / frame / sample. Trading bars, game frames, vision 10 ms are **host names** for the same Mealy pulse ([[docs/concepts/synchronous-pulse]]).
- Executive stays the host (`exec` optional). Adapters stay out.

## Test

An agent can implement the kernel from [[docs/concepts/policy-kernel]] without opening `newton-machine`’s `update` to YAML and without importing a broker.

A game designer can change `DiveAttack.speed` in TOML without rustc, and cannot add `ParryWindow` without a new score kind.

Five desks can depend on this crate and keep their own mandate YAML, IR catalog, and broker adapter.

## Related

- [[docs/concepts/policy-kernel]]
- [[docs/goals/versatile-not-a-desk]]
- [[docs/goals/ports-not-batteries]]
- [[docs/plans/v0-crate-roadmap]]
