---
tags: [sticky, history, trap]
node_type: edge_case
---
# Sticky is not Harel history

**Harel history:** on re-entering a composite, restore the last child (shallow) or the full tree (deep). Sidecar of discriminants. Lives on `newton-machine`.

**Sticky / arm:** a score stays in the pool while raw is false, for N units of **its** clock. Lives on the policy kernel (or host `Model`).

Traps:

- Writing sticky into `History` so a snapshot “remembers” an order or a socket.
- Aging every score on the Executive pulse (`5s` burns `1m`; `10ms` burns `1s`).
- Calling `perform()` when sticky expires — that is a score leaving the pool, not an XOR child exiting, unless you *also* have a typed region that must move (then Lift emits one `Msg`).

## Related

- [[docs/adr/0019-gated-hysteretic-scores-are-not-harel-history]]
- [[docs/concepts/hysteretic-score]]
