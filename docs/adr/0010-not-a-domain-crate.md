---
tags: [scope, domain]
node_type: adr
---
# 0010 Not a domain crate

## Status

Accepted

## Context

The first conversation that needed an Executive was a trading desk. If that vocabulary leaks, the crate is unusable for sessions, robots, UIs, operators, and firmware — and it duplicates `newton-machine` ADR 0011’s refusal in the wrong place.

## Decision

`newtonian-core` is domain-agnostic. No orders, bars, symbols, ROS types, DOM events, or pod specs in this crate. Examples and tests use a **session protocol** (Offline / Connecting / Online) or equally generic names (door, thermostat, pipeline gate).

A Quant 1 m bar, a game frame, and a 10 ms vision capture are the **same** kernel pulse with host names ([[docs/concepts/synchronous-pulse]]). Do not encode `Bar` in core.

A later `newton-trading` / `newton-ros` / `newton-ui` companion, if any, is a domain crate. It is not this crate.

## Consequences

- README hero is the five-way split, not a blotter.
- Reviewers reject PRs that add market-data types “because we have an example.”

## Related

- [[docs/goals/versatile-not-a-desk]]
- tests/session.rs
