---
tags: [effects, tea]
node_type: adr
---
# 0009 Effects stay in the Executive

## Status

Accepted

## Context

Boost.SML `/ action`, python-statemachine `on_enter_*`, Spring SM listeners, Qt `onTriggered`, XState actions-with-closures, Unreal BT tasks: all put the world inside the chart. That is convenient and it makes snapshots lie.

## Decision

- Machine actions emit `Cmd` data.
- Lift emits `Msg` or silence.
- Skills emit patches / requests, not I/O as a required part of the trait.
- Gateway admits.
- **The Executive** (or the host that is standing in for it) is the only place a socket, a file, a ROS action, or a DOM write may happen.

## Consequences

- Replay, test, and live share lift + `apply`. They do not share the gateway’s wire.
- Authors coming from XState will feel inconvenienced. That is the product.

## Related

- [[docs/concepts/steal-sheet]]
- [[docs/goals/elm-purity-xstate-instance]]
