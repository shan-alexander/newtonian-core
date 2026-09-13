---
tags: [lift, rules]
node_type: adr
---
# 0011 Lift is not the machine

## Status

Accepted

## Context

Drools, OPS5, CLIPS, and every “when this metric crosses, do that” dashboard prove that *category changes in beliefs* want a name. If we let that name be “the state machine,” we rebuild a Rete and throw UCA away. If we put it in `update`, `on_tick` becomes 400 lines.

## Decision

`Lift` is a pure function: `(now, revision of the changed fact, mandate, config) -> Silence | Msg | Batch`.

One pulse may emit several `Msg`s (`Lifted::Batch`). The Executive applies them in order. Lift still does not `apply`. See [[docs/adr/0021-batch-lift-max-age-disarm-port-executive]].

Do not clone the whole previous store. Other keys are read from `now`; the changed key’s prior payload is `revision.previous`. See [[docs/adr/0020-kernel-sits-beside-the-chart]].

- Ticks and continuous measurements: silence (store already revised).
- Crossing a bound, appearing, disappearing, justification withdrawn: maybe `Msg`.
- Writers never call `apply`. Lift never calls `apply`. The Executive applies.

A host may implement lift as a small match, a list of predicates, or (later) a generated table. A Rete network is an optional *implementation of Lift*, not a replacement for the intention machine.

## Consequences

- Storm risk moves to lift (emitting a `Msg` every tick). Cap it in `exec` or the host pulse. See [[docs/edge_cases/lift-storm]].

## Related

- [[docs/concepts/lift]]
- [[docs/adr/rete-as-machine]]
- [[docs/adr/0020-kernel-sits-beside-the-chart]]
- [[docs/edge_cases/belief-change-is-not-a-msg]]
