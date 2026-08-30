---
tags: [ports, newton-machine]
node_type: adr
---
# 0014 Intention machine is a port

## Status

Accepted

## Context

The reference chart is `newton-machine`. Binding `newton-core` to that crate at `0.0.0` would (a) block other UCA hosts, (b) create a version tangle before either crate is on crates.io, (c) tempt us to re-export the chart and confuse crate identity.

## Decision

`IntentionMachine` is a trait in this crate: `apply`, `view`, `snapshot`, `in_state`.

Feature `machine` provides `impl<M: Machine + Clone> IntentionMachine for newton_machine::Runtime<M>`. Associated types *are* the chart’s (`Msg`, `Cmd`, `View`, `Snapshot`, `NodeId`). Do not fork them.

Hosts without `newton-machine` impl the trait themselves (`tests/session.rs`). Downstream crates **cannot** write the blanket impl (orphan rules: `Runtime<LocalChart>` is uncovered). That is why the adapter lives here.

## Consequences

- Default features still do not pull `newton-machine`. Firmware / tests that only need ports stay free of it.
- GitHub examples enable `machine` via `required-features`.

## Related

- [[docs/adr/0015-two-crates-exec-is-a-module]]
- symbol:IntentionMachine
