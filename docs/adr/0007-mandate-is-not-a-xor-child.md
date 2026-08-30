---
tags: [mandate, policy]
node_type: adr
---
# 0007 Mandate is not a XOR child

## Status

Accepted

## Context

Standing aims (“be docked by 02:00”, “never lose unsaved work”, “flat by 15:55”, “1% daily loss”, “reconnect at most 5 times”) look like states to an author who only has a chart. They are desires. They survive Halted → Armed. They change rarely. When they change, that is news.

## Decision

Mandate is a small, preferably immutable struct the Executive passes into lift and into gateway admission. Changing it is a `Msg` (`MandateRevised` or domain equivalent). It is not a region, not a belief, and not a `Cmd`.

YAML/TOML, if used, feeds **mandate knobs**, not chart topology.

## Consequences

- A trip through a locked/halted configuration does not require re-parsing policy.
- Tests can swap mandate without rebuilding the machine type.

## Related

- [[docs/concepts/mandate]]
- [[docs/adr/0012-yaml-is-knobs-not-topology]]
- [[docs/edge_cases/mandate-survives-halt]]
