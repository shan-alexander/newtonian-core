---
tags: [ports, scope]
node_type: adr
---
# 0005 Core is ports, not the loop

## Status

Accepted (amended by [[docs/adr/0015-two-crates-exec-is-a-module]])

## Context

XState ships `createMachine` *and* `interpret` in one package. Convenient, and it trains authors to put actions on the machine. Elm ships `Browser.application` in `elm/browser`, not in `elm/core`. The *chart vs host* grain is the one we want. The *ports vs our optional pulse* grain does not need a second crates.io name.

## Decision

`newtonian-core` *identity* is ports and vocabulary. The optional Executive lives in `newtonian_core::exec` so we do not publish `newton-exec`.

The crate does not ship, as *required* API:

- a pulse you must call
- a default clock
- sockets, files, or async
- a batteries-included belief store (a tiny example in tests is allowed)

`ProgramParts` is a bag of handles, not `fn run()`. `exec` may later offer a pulse; hosts that already have a loop never import it.

## Consequences

- Hosts can embed the ports in rtic, embassy, iced, tokio, or a paper test without taking a sequencer.
- `exec` helpers that need `std` are feature-gated when they exist. This crate stays `no_std` + `alloc` at the root.

## Related

- [[docs/goals/ports-not-batteries]]
- [[docs/adr/0013-no-std-and-no-sockets]]
- symbol:ProgramParts
