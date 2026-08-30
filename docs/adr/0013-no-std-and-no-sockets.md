---
tags: [no_std, io]
node_type: adr
---
# 0013 no_std, no sockets

## Status

Accepted

## Context

Firmware mode managers and rtic/embassy skills are first-class consumers. A tokio `TcpStream` in a trait default would exile them. Sockets in a store would make snapshots dishonest.

## Decision

- `#![no_std]` + `alloc` + optional `std` + optional `serde`.
- MSRV 1.80. Dual MIT OR Apache-2.0. `unsafe` forbidden. Match `newton-machine`.
- No I/O in this crate. Tests may use `std`.
- Skills, lift, gateway admit, belief revise: no sockets.

## Consequences

- Clocks are ticks the Executive passes in, not `Instant::now()` inside the store.
- `exec` may later take a `Clock` trait behind `std`. The ports do not.

## Related

- [[docs/adr/0005-core-is-ports-not-the-loop]]
- [[docs/adr/0009-effects-stay-in-the-executive]]
