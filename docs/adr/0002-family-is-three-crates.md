---
tags: [family, crates]
node_type: adr
---
# 0002 Family ontology (crate count superseded)

## Status

Superseded by [[docs/adr/0015-two-crates-exec-is-a-module]] for *how many crates we publish*. The five-way ontology in this note still stands.

## Context

One crate that is chart + beliefs + loop + sockets is actor soup. Three crates that do not talk are a brochure. We need a grain that matches the ontology.

## Decision

| Crate | Owns | Does not own |
| --- | --- | --- |
| `newton-machine` | UCA chart, TEA runtime, history sidecar, `Cmd` as data | beliefs, mandate, Executive, gateway I/O |
| `newton-core` (this) | ports: intention, belief, mandate, lift, gateway, skill, program parts | the pulse, sockets, domain types |
| `newton-exec` (later) | Executive loop, memory store, lift helpers | sockets, chart topology, domain types |

Together they are a **Newtonian program**. An application adds sensors, a concrete gateway, and (optionally) skills.

`newton-core` may later depend on `newton-machine` for a blanket `impl IntentionMachine for Runtime<M>`. At `0.0.0` it does **not**, so the port stays honest: any UCA host can satisfy it.

`newton-exec` lives in this family. It may start as a second package in this repo (workspace) or a sibling directory. It is not this crate.

## Consequences

- Agents must not “just add `run()` to newton-core.”
- Agents must not re-implement XOR/AND here.

## Related

- [[docs/goals/establish-the-newtonian-program]]
- [[docs/goals/ports-not-batteries]]
- [[docs/concepts/newtonian-program]]
