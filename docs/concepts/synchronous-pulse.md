---
tags: [clock, pulse, mealy, frame, bar]
node_type: concept
aliases: [clock, frame, bar, tick]
---
# Synchronous pulse

The policy kernel is **Mealy on one external clock**: one tick per entity, no thread per region, same `step_entity` for live / replay / test.

The clock is **generic**. Hosts name it; this crate does not.

| Domain | One pulse is | Not a score clock |
| --- | --- | --- |
| Quant desk | timeframe bar close (1 m, 5 s) or a broker tick | a 1 m score must not age on a 5 s pulse |
| Game | frame (display / sim tick) | animation-hold frames ≠ Harel history |
| Robot / vision | captured image / 10 ms sample | “target lost 200 ms” is a score latch |
| Session / protocol | idle timer or I/O readiness turned into a tick | RTT sample is a belief |

Beliefs arrive **on** the pulse (or are sampled at it). Lift / score eval **at** the pulse. `Cmd` **from** the pulse. The Executive (game loop, 10 ms sequencer, `qsys-engine`, a test) **is** the pulse. `newtonian_core::exec` may offer a helper; hosts with a loop ignore it.

Newton `apply` is optional and **category-change gated**: if a typed XOR child did not move, do not `perform()`. Score eval may still run every pulse (cheap bitsets). That is why the kernel must not be “Newton on every flicker.”

## Related

- [[docs/concepts/policy-kernel]]
- [[docs/concepts/executive]]
- [[docs/adr/0016-newtonian-core-is-the-policy-kernel]]
- [[docs/goals/versatile-not-a-desk]]
