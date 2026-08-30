---
tags: [score, sticky, history, clock]
node_type: adr
---
# 0019 Gated hysteretic scores are not Harel history

## Status

Accepted

## Context

Harel deep/shallow history restores the last child when you **re-enter a composite**. Sticky / arm / max-age keeps a **predicate in the pool** while the raw classifier is false, on its own clock (`{tf,bars}` in trading; `{frames}` or `{ms}` in a game or robot). Mixing them is how `on_bar` became 400 lines and how Newton’s sidecar gets sockets and “stay in-play N bars.”

A 5 s pulse must not burn a 1 m sticky. A 10 ms vision frame must not age a 1 s “target lost” latch as if it were one tick. A game frame must not treat animation-hold as chart history.

## Decision

Each **score** is a named boolean with:

- enablement (may be gated by a parent score or stage — [[docs/concepts/enablement-dag]]),
- stickiness / max age,
- disable rules,
- its **own clock** ([[docs/concepts/synchronous-pulse]]).

This is a latch with a gated clock, not a Harel region. It lives in the **policy kernel**, or in host `Model` if the host has not taken the kernel yet. It does **not** live in `newton-machine` history.

`Lift` remains the door from beliefs to Newton `Msg` ([[docs/adr/0011-lift-is-not-the-machine]]). Category-change of a *typed XOR child* is a `Msg`. A score flickering inside sticky is **not** a Newton transition.

Parent drops (or sticky expires) → children are forced off this tick, or given a documented decay. No silent leftover bits in the superstate key.

## Consequences

- `perform()` on every score flicker is a misuse of Newton.
- Criterion on `step_entity` (kernel) is the budget; Newton benches stay apply/LCA.

## Related

- [[docs/concepts/hysteretic-score]]
- [[docs/edge_cases/sticky-is-not-harel-history]]
- [[docs/concepts/enablement-dag]]
