---
tags: [rejected, rules]
node_type: adr
---
# Rejected: Rete as the machine

## Status

Rejected

## Context

Production systems are good at “when belief changes, assert Msg.” They are a poor intention structure: no XOR/AND configuration type, no inertial history, no TEA snapshot.

## Decision

Rejected as the core loop. A Rete (or a boring `match`) may implement `Lift`. The Newton machine remains the intention structure.

## Related

- [[docs/adr/0011-lift-is-not-the-machine]]
- [[docs/concepts/lift]]
