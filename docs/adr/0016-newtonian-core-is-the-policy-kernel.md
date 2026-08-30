---
tags: [family, kernel, product]
node_type: adr
aliases: [policy kernel product]
---
# 0016 newtonian-core is the policy kernel

## Status

Accepted

Does **not** unpublish the 0.0.0 ports. 0.0.0 named the ontology. This ADR names the **product** those ports grow into. Does not fold the kernel into `newton-machine` ([[docs/adr/0015-two-crates-exec-is-a-module]] still holds).

## Context

`newton-machine` is UCA truth: typed XOR/AND, TEA `update`, `Cmd` as data, history sidecar. Valuable to Rust engineers as a conservation-law crate. It **refuses** YAML plays, a sleeve ROM, and a gated eval DAG — correctly ([[docs/adr/0012-yaml-is-knobs-not-topology]] as applied to *charts*).

A Quant desk, a game, and a 10 ms vision loop still need a middle machine neither crate shipped: hysteretic predicates → exact configuration key → data-driven behavior, with gated evaluation and effects as data.

Bolting a `HashMap` onto Newton pollutes UCA law 1 (configuration is the type). Filing the serial numbers off a desk Executive (`qsys-engine`, IBKR pacing, lake) makes a fake “generic engine.” Five desks would fight a framework. Interpreting topology every tick is SCXML.

The extractable core is a **configuration-indexed policy kernel**.

## Decision

| Piece | Job | Is not |
| --- | --- | --- |
| `newton-machine` | Typed modal **truth** (optional backend) | Sleeve product |
| **`newtonian-core`** | **Policy kernel**: gated scores + exact superstate key + compiled sleeve ROM + knobs | Executive, broker, lake, renderer |
| Host Executive | Pulse (game loop, 10 ms sequencer, `qsys-engine`, test) | Shared crate |
| Host adapters | Broker / motors / animation / sockets | Shared crate |

**Kernel** = the only step from live scores to a `Cmd`; anti-patterns unrepresentable or fail-closed; alloc-free hot path as the performance law.

**Policy** = what to do is a table compiled from YAML/TOML, not a rustc change.

**Configuration-indexed** = the index is the **exact** product of in-play scores (a bitset), not a single XOR “mode” and not longest-subset fallback ([[docs/adr/0017-exact-superstate-key-not-longest-subset]]).

0.0.0 remains ports (`IntentionMachine`, `BeliefStore`, `Mandate`, `Lift`, `Gateway`, `Skill`, `exec` named). The kernel is the thing we **implement next**, still in this crate, still without a third crates.io name, still without sockets.

Wrong merges (ban):

- Kernel “is” newton-machine + hash table.
- Kernel “is” generic qsys-engine (pulse + lake + `client_id`).
- YAML charts interpreted every tick.
- Domain vocabulary (bars, ROS, DOM) in kernel types. Clocks are generic: **tick / frame / 10 ms sample**. Trading *bars* are one host clock.

## Consequences

- Agents building this crate implement the kernel *here*, not in `newton-machine`.
- Newton remains optional: a session/order/hold XOR can be an `IntentionMachine`; a desk may skip Newton and feed scores from a bitset.
- `exec` stays optional. The kernel is not the pulse.
- Five Quant desks share kernel + maybe Newton. They do not share a broker adapter.

## Related

- [[docs/concepts/policy-kernel]]
- [[docs/goals/configuration-indexed-policy-kernel]]
- [[docs/adr/0015-two-crates-exec-is-a-module]]
- [[docs/adr/0010-not-a-domain-crate]]
- [[docs/concepts/newtonian-program]]
- [[docs/adr/0020-kernel-sits-beside-the-chart]]
