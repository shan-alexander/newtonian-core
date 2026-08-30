---
tags: [lift, rtc, storm]
node_type: edge_case
---
# Lift storm

`newton-machine` caps internal RTC drains. Lift can still feed `apply` every pulse if a predicate is written as `if stale { Msg::Stale }` instead of “became stale.”

Lift must be edge-triggered: previous vs now. `exec` (or the host pulse) should additionally cap consecutive lift-emitted messages per pulse and surface a storm outcome, the same way the chart surfaces an internal-event storm.

## Related

- [[docs/adr/0011-lift-is-not-the-machine]]
- [[docs/edge_cases/belief-change-is-not-a-msg]]
