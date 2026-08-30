---
tags: [gateway, authority]
node_type: adr
---
# 0008 Gateway is authority

## Status

Accepted

## Context

`newton-machine` already split intent from authority (its ADR 0012). This crate restates the split as a **port**, so an Executive cannot “just call the client” from lift.

## Decision

- The machine emits `Cmd` (intent, data).
- `Gateway::admit` is a **pure** decision: Admit / Refuse / Drop.
- The Executive (or a process beside it) performs admitted effects.
- Actuate out of band. Reconcile in band via `Msg`.
- Do not give the machine, the belief store, or lift a `Client` trait.

The gateway may panic, restart, or lock the wire without tearing down the intention snapshot. OTP: they die independently.

## Consequences

- `OpenGateway` exists for tests and is named as a hole.
- Hardware estop, venue COD, kube admission, UI sandbox — all gateways. None of them are chart children.

## Related

- [[docs/concepts/gateway]]
- [[docs/edge_cases/exec-and-gateway-die-independently]]
- [[docs/adr/0009-effects-stay-in-the-executive]]
