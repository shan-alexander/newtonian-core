---
tags: [scope, domain, goals]
node_type: goal
aliases: [not a desk, not trading]
---
# Versatile, not a desk

The motivating *conversation* that produced this family included equity bars. The motivating *pattern* did not. A crate whose rustdoc says “mark,” “locate,” or “RTH” will be filed under trading and ignored by the protocol, robotics, UI, and firmware engineers it is for.

## Goals

- Core types know nothing about orders, bars, brokers, ROS, DOM, or pods.
- The pulse is generic: **tick / frame / sample**. A Quant 1 m bar, a game frame, and a 10 ms vision capture are host names for [[docs/concepts/synchronous-pulse]].
- README examples lead with session / robot / game / operator / firmware. A desk is a row of a table, not the hero snippet.
- Domain crates (optional, later, never required) may exist. They are not this crate and they are not `newton-machine`.

## Test

If a panicked engineer can `impl Mandate` and accidentally depend on a broker crate from `newtonian-core`, the family has been broken.

## Related

- [[docs/adr/0010-not-a-domain-crate]]
- [[docs/concepts/newtonian-program]]
