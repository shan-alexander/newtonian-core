---
tags: [gate, dag, enablement]
node_type: concept
aliases: [clock enable, feature tree, staged gates]
---
# Enablement DAG

The score pool is **not** a flat bag that always evaluates every classifier. Evaluation is a DAG of **enablement**:

```text
SESSION_RTH (book, key:false)  ──enables──►  SETUP_LONG
SETUP_LONG in-play             ──enables──►  GREEN_ENTRY, CHANNEL_SLOPE, …
sleeve long_hot selected       ──enables──►  exit-ladder scores while position exists
```

Game: `Combat` in-play enables `BossNear` classifiers; `Explore` does not. Robot: `TargetLock` enables `ApproachOk`; lost lock forces children off this tick (or documented decay).

Parent drops (or sticky expires) → children **forced off** this pulse, unless a decay rule is authored. Leftover child bits in the key are a bug.

This is clock-enable hierarchy / a feature tree / short-circuit production rules with an explicit context stack. It is **not** a behavior tree (imperative, one token at a time) and **not** Harel XOR (exactly one child).

Enablement edges are the gray zone ([[docs/adr/0018-fold-yaml-once-never-interpret-topology]]):

- **Strict:** parent/child gates are Rust (or a compiled IR enum). New gate kind = rustc. Safer for a kernel.
- **Desk-friendly:** YAML `enabled_by: [SETUP_LONG]` as long as Fold resolves names to bit indices. Still not an interpreter at runtime.

## Related

- [[docs/adr/0019-gated-hysteretic-scores-are-not-harel-history]]
- [[docs/concepts/hysteretic-score]]
- [[docs/concepts/policy-kernel]]
