---
tags: [score, sticky, latch]
node_type: concept
aliases: [score, sticky, arm]
---
# Hysteretic score

A **score** is a named boolean in the policy kernel’s pool. It is not a Harel XOR child and not a belief (beliefs are numbers, images, freshness — the tape / camera / blackboard). A score is a **classifier with memory**.

Each score has:

- a raw predicate of beliefs (and maybe Newton config),
- **enablement** — may be gated by a parent ([[docs/concepts/enablement-dag]]),
- **stickiness / max age** — stays in-play while raw is false, on **its** clock,
- disable rules.

Digital analogue: a latch with a gated clock. Trading analogue: `{tf, bars}` arm. Game analogue: “LowHp for 12 frames after the spike.” Robot analogue: “target lost, hold 200 ms.”

A 5 s pulse must not age a 1 m score as one tick. A 10 ms vision frame must not age a 1 s latch as one tick. The **external pulse** is one clock; each score may tick a **slower** clock (count bars / frames / ms of *its* period).

## Related

- [[docs/adr/0019-gated-hysteretic-scores-are-not-harel-history]]
- [[docs/edge_cases/sticky-is-not-harel-history]]
- [[docs/concepts/policy-kernel]]
