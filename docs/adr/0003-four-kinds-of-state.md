---
tags: [ontology, state]
node_type: adr
---
# 0003 Four kinds of state (and a binder)

## Status

Accepted

## Context

Live systems keep one `AppState` / `Context` / `World` struct. It holds mode, last sensor, standing limits, and a client handle. That struct is why `on_event` becomes 400 lines and why snapshots contain sockets.

## Decision

A live Newton program has **four kinds of state** plus one binder:

1. **Configuration** — committed modal stance. Newton machine.
2. **Beliefs** — what we take to be true right now. Belief store.
3. **Mandate** — standing aims. Small immutable struct.
4. **Authority** — what we may do to the world. Gateway.
5. **Executive** — the loop that binds 1–4 to sensors, effectors, humans. Not a fifth kind of *state*; a sequencer.

Do not store (2)(3)(4) as XOR children of (1). Do not store (1) as a string on the blackboard.

## Consequences

- Snapshot of the machine is not a snapshot of the program. Beliefs and mandate persist separately (or in a *program* snapshot that is a product type, not a chart).
- Context on the machine stays the slice of beliefs the last step needed.

## Related

- [[docs/concepts/four-kinds-of-state]]
- [[docs/adr/0006-beliefs-are-not-context]]
- [[docs/adr/0007-mandate-is-not-a-xor-child]]
