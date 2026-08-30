---
tags: [lift, belief]
node_type: edge_case
---
# Belief change is not a Msg

A store revision is the common case. A `Msg` is a category change: crossing a bound, appearing, disappearing, justification withdrawn, mandate revised.

If lift emits a `Msg` on every tick, you have rebuilt `on_bar` / `on_pose` / `on_frame` as a storm. The machine will spend RTC draining noise. History will record junk. See [[docs/edge_cases/lift-storm]].

**Test:** `tests/session.rs` — hello age 1..=4 is `Silence`; crossing the bound is `FeedStale`.

## Related

- [[docs/adr/0011-lift-is-not-the-machine]]
- [[docs/concepts/lift]]
