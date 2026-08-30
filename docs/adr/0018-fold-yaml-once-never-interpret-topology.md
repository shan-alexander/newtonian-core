---
tags: [yaml, fold, ir]
node_type: adr
---
# 0018 Fold YAML once; never interpret topology

## Status

Accepted

Specializes [[docs/adr/0012-yaml-is-knobs-not-topology]] for the policy kernel. Does not allow YAML *charts*.

## Context

Teams need to change plays without rustc: which chords have firepower, thresholds, size, speed, `{tf,bars}` / frame windows. They also invent new *kinds* of truth (a new IR / score type, a new XOR child). Mixing those two in one document either recompiles forever or interprets a statechart every frame.

## Decision

**Once, in Rust** (new ideas):

- Score kinds / IR types (`QuadOversold`, `LowHp`, `Grounded`).
- XOR regions if Newton is the truth backend (`Session`, `Clip`).
- Behavior implementations (`fn enter_long`, `fn set_playback_rate`).
- Gateway policies that are structural (hard drawdown not overridable).
- Enablement DAG *shape* if it is an enum of stages (strict design).

**Forever, in YAML/TOML** (knobs + which plays exist):

- Thresholds, windows, clocks (`{tf,bars}`, `{frames}`, `{ms}`), speed, size.
- Which sleeves exist (`when_exactly`, observe-only).
- Parameters behaviors read (`speed: 1.15`, `size_tier: 2`).
- Numeric limits.
- Optional: `enabled_by: [KURISKO_LONG_1M]` **if Fold resolves names to bit indices at compile-of-YAML**. Still not an interpreter at runtime.

Fold / compiler: `RawDoc → BitIndex + Table<Key, Sleeve> + KnobMap` **once**. Runtime is O(1) lookup on interned keys. No string ids on the hot path. No YAML in `step`.

Topology of **truth kinds** = types. Topology of **plays** = data. Numbers = data.

## Consequences

- A new boolean the designer needs (`ParryWindow`) is a rustc change. A new speed on `DiveAttack` is a file change. That is the product contract.
- Reviewers reject PRs that parse YAML inside `step_entity`.

## Related

- [[docs/adr/0012-yaml-is-knobs-not-topology]]
- [[docs/concepts/sleeve-rom]]
- [[docs/concepts/policy-kernel]]
