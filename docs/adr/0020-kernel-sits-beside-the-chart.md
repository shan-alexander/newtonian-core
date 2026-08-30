---
tags: [kernel, composition, lift, sleeve]
node_type: adr
aliases: [kernel composition, beside the chart]
---
# 0020 Kernel sits beside the chart

## Status

Accepted

Does not supersede [[docs/adr/0016-newtonian-core-is-the-policy-kernel]] (product) or [[docs/adr/0011-lift-is-not-the-machine]] (lift is still the only door to Newton `Msg`). Refines *how* the kernel, the chart, lift, and `newton-machine`’s `ChordTable` share a pulse.

## Context

0.0.0 named two paths that both emit `Cmd`:

1. **Chart path:** `BeliefStore::revise` → `Lift` → `IntentionMachine::apply` → `Cmd`.
2. **Kernel path:** beliefs → gated scores → exact key → sleeve ROM → `Cmd`.

Docs never said whether `step_entity` may call `apply`, whether Newton `project()` *is* the ROM key, whether machine `Cmd` and sleeve payload are the same type, or how this crate relates to `newton_machine::ChordTable` (already an exact / longest-subset host table). Without that law, the next patch is one of the banned merges: HashMap inside `update`, or a second `ChordTable` with no hysteresis.

Handwritten expansion before macros (and before YAML Fold) is the same grain `newton-machine` used for charts: a typed `Table<Key, T>` you can author in Rust, then Fold fills it from a document.

## Decision

### Placement

The policy kernel is a **sibling** of the intention machine, not a child and not a wrapper.

```text
beliefs ──► Lift ──► Msg ──► IntentionMachine::apply ──► chart Cmd
   │
   └──► raw classifiers ──► gated scores ──► Key ──► Table ──► sleeve (T)
                              extra bits ────┘
                                      │
                                      ▼
                               Gateway::admit
```

- `step_entity` **does not** call `apply`. Category-change of a typed XOR child is still lift → `apply` ([[docs/adr/0011-lift-is-not-the-machine]], [[docs/adr/0019-gated-hysteretic-scores-are-not-harel-history]]).
- Score flicker inside sticky is **not** a Newton `Msg`.
- `ProgramParts` stays the ports bag (machine, beliefs, mandate, gateway, lift, skills). The kernel is **not** a seventh field. The host (or later `exec`) sequences both.

### Key composition

- In-play **scores** form a bitset `Key` (exact product). Unauthored → none ([[docs/adr/0017-exact-superstate-key-not-longest-subset]]).
- Optional Newton truth may **join** that key as `extra` bits (`Runtime::project()` mapped into a disjoint range). Extra bits are **not** the ROM and **not** the enablement DAG.
- Hosts must keep score bits and truth bits disjoint. Overlap is a mapping bug.
- Enablement is **score-on-score**. If a XOR child must gate a classifier, the host zeros that score’s raw bit (or authors a score that mirrors the child). The kernel does not read the chart.

### Two doors, one authority

| Door | Input | Output | Owns |
| --- | --- | --- | --- |
| `Lift` | belief revision × mandate × config | `Msg` or silence | chart category-change |
| `step_entity` | raw bits × extra × pulse | sleeve `T` or none | policy ROM |

- Lift sees **`now` plus a `Revision` of the fact that changed**, not a cloned previous store. Crossing a bound still uses other keys from `now` plus `revision.previous` for the changed key. That is the same law as [[docs/adr/0011-lift-is-not-the-machine]]; the signature no longer requires a second store snapshot.
- Sleeve payload `T` is interned data (behavior id + knobs). It is **not** I/O.
- Machine `Cmd` and sleeve `T` may be the **same** type, or the host maps `T` → `Cmd` before admit. The kernel does not invent a second effect algebra.
- `Gateway` admits world effects. One gateway if the types already match; otherwise map, then admit. The kernel does not call the gateway.

### ChordTable stays in newton-machine

`newton_machine::ChordTable` is a **host helper** over a projected config bitset (exact *or* longest-subset). It is not this crate’s product.

- Kernel matcher is **exact only**. Unauthored → none. No `contains` walk, no tie-break.
- Hosts that want longest-subset keep using `ChordTable` **outside** `step_entity`.
- This crate does not re-export or wrap `ChordTable`. Hysteresis, enablement, and Fold are why a kernel exists; another exact `HashMap` is not.

### Handwritten ROM before Fold

Fold’s *output* is `Folded { specs, table }`. YAML/TOML later *fills* that. Until Fold exists, hosts intern rows in Rust (sorted unique `Key`s). No YAML in `step`. No string ids on the hot path. No macros until a second handwritten host exists.

### Beliefs stay host-typed

`BeliefStore::Fact` remains one associated type. Heterogeneous boards (pose + battery + hello) are a **host enum**, several stores, or a later column API. Core will not `Box<dyn>` the blackboard.

### Snapshot restore

`IntentionMachine::restore` is the inverse of `snapshot`. Restoring is still not beliefs ([[docs/edge_cases/restore-snapshot-is-not-beliefs]]). The kernel’s `EntityState` is a separate persistable; it is not stuffed into the chart sidecar.

## Consequences

- An agent implementing the kernel writes `src/kernel/` here, not `newton-machine::update`.
- A host pulse: ingest → revise → lift → maybe `apply` → raw scores → `step_entity` → admit chart `Cmd` and/or sleeve `T` → present.
- Five desks share kernel + optional Newton. They do not share `ChordTable` longest-subset as kernel law.
- Criterion on `step_entity` is this crate’s budget.

## Related

- [[docs/concepts/policy-kernel]]
- [[docs/concepts/sleeve-rom]]
- [[docs/adr/0016-newtonian-core-is-the-policy-kernel]]
- [[docs/adr/0017-exact-superstate-key-not-longest-subset]]
- [[docs/adr/0018-fold-yaml-once-never-interpret-topology]]
- [[docs/edge_cases/unauthored-key-is-none]]
- symbol:step_entity symbol:Table symbol:Key symbol:Lift symbol:ProgramParts
