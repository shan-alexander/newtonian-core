---
tags: [naming, executive]
node_type: adr
---
# 0004 Call it the Executive

## Status

Accepted

## Context

The job already has names. Inventing “behavior layer,” “policy engine,” “newton-policy,” or “the runtime” would collide with TEA `Runtime`, with trading policy, and with every game-AI article.

Robotics 3T (Bonasso, Firby RAPs, Gat ATLANTIS): planner on top, **sequencer / executive** in the middle, reactive skills at the bottom. NASA PLEXIL / Universal Executive, DS1 Remote Agent Smart Executive, MIT RMPL: same grain. XState: interpreter. Elm: `Program`. OTP: `gen_server` + supervisor.

## Decision

Call that layer the **Executive**.

- Do not call it “the behavior layer.”
- Do not fold it into the Newton machine.
- Do not call this crate `newton-policy`. Mandate is policy-as-ends; the Executive is practical reason. Those are different.

Those two mistakes (behavior-layer, fold-into-machine) are how this family collapses back into actor soup. See [[docs/adr/behavior-layer]] and [[docs/adr/actor-soup]].

## Consequences

- README sentence: *a Newton program is an Executive running one or more Newton machines against a belief store and a gateway.*
- Module that *implements* the loop: `newtonian_core::exec`, not `newton_machine`. Hosts may ignore it and be the Executive themselves. Not a third crate. See [[docs/adr/0015-two-crates-exec-is-a-module]].

## Related

- [[docs/concepts/executive]]
- [[docs/concepts/three-tier-placement]]
- [[docs/goals/elm-purity-xstate-instance]]
