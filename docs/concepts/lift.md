---
tags: [lift, rules]
node_type: concept
---
# Lift

The only door from beliefs to the chart.

`(now, revision of the changed fact, mandate, config) -> Msg | Silence`

Do not clone the previous store. Other keys are read from `now`. See [[docs/adr/0020-kernel-sits-beside-the-chart]].

Last print 100.01 → 100.02: **silence** (store already has it). Last print aging past the mandate bound: **`Msg::FeedStale`**. Appearance / disappearance of a justification: maybe a `Msg`.

Production rules (Drools, OPS5, CLIPS) are the ancestor. A thin `match` is the usual impl. Rete is allowed *as an impl of this trait*, not as the machine. See [[docs/adr/rete-as-machine]].

Lift is pure. It does not `apply`. It does not I/O. The Executive applies what lift emits.

Do not confuse Lift with **score eval**. Lift turns a *category change of modal stance* into a Newton `Msg`. Scores form the policy kernel’s bitset every pulse (with sticky and enablement). A score flicker that does not move an XOR child is silence for Newton and still a pool update for the ROM. See [[docs/concepts/hysteretic-score]] and [[docs/adr/0019-gated-hysteretic-scores-are-not-harel-history]].

## Related

- [[docs/adr/0011-lift-is-not-the-machine]]
- [[docs/adr/0020-kernel-sits-beside-the-chart]]
- [[docs/edge_cases/belief-change-is-not-a-msg]]
- [[docs/edge_cases/lift-storm]]
- symbol:Lift symbol:Lifted symbol:Revision
