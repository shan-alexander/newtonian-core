---
tags: [belief, context]
node_type: adr
---
# 0006 Beliefs are not context

## Status

Accepted

## Context

`newton-machine` has `context` / `Model` on the machine: the extended state the last step needed. Hosts will be tempted to dump the world there. Then every tick is a transition, history explodes, and “feed went stale” is a fake XOR child.

## Decision

- **Belief store:** justified, freshness-stamped, larger than the chart, may change without a transition.
- **Context:** slice copied in at `apply` if a guard needs it. Not a mirror of the store.
- **Configuration:** committed stance. Not a cache of the last observation.

A belief enters the chart only when it changes what we are committed to.

Facts carry a justification (`source`, `tick`). Withdrawal marks stale. No `MaybeOnline` state.

## Consequences

- Restore of a machine snapshot does **not** restore beliefs. See [[docs/edge_cases/restore-snapshot-is-not-beliefs]].
- Lift, not `update`, notices category changes.

## Related

- [[docs/concepts/belief-store]]
- [[docs/edge_cases/belief-change-is-not-a-msg]]
