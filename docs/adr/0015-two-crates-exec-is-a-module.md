---
tags: [family, crates, exec, sprawl]
node_type: adr
aliases: [two crates, crate sprawl]
---
# 0015 Two crates; Executive is a module

## Status

Accepted

Supersedes the *distribution* part of [[docs/adr/0002-family-is-three-crates]]. Does **not** supersede the ontology: chart vs beliefs vs mandate vs gateway vs Executive remain distinct. The Executive is still not the machine.

## Context

A third crate `newton-exec` was sketched so the ports crate would not grow a `run()` and become XState-in-one-package. That fear is real — but it is a *module* fear, not a *crates.io* fear. Three questions decide the grain:

1. Would engineers take `newton-exec` **without** `newtonian-core` or `newton-machine`?
2. Is `newtonian-core` useful **without** our Executive?
3. Is `newtonian-core` useful **without** `newton-machine`?

### 1. Nobody takes exec alone

`newton-exec` is not a runtime product. Tokio, embassy, iced, rtic, and XState `interpret` already exist. The Executive is *this pulse* against *these ports*:

`ingest → revise → lift → apply → admit → present`

Without `newtonian-core` it has nothing to pulse. Without *some* `IntentionMachine` it has no stance to step. A crate that is only useful after you have already added the other two is crate sprawl: version tango, “which crate do I add?”, docs.rs fragmentation. Analog: nobody publishes `tracing-the-default-fmt-layer` as a reason to skip `tracing`. Analog that fails: XState shipping `interpret` *on the chart* — that trains authors to put closures in actions. Our Executive would sit on the **ports** crate, not on `newton-machine`. That is the split that prevents soup.

### 2. Core without exec is the embedding story

Yes, and it is the *main* story. Hosts that already have a pulse should ignore `exec`:

| Host pulse | Uses from this crate |
| --- | --- |
| A unit test (`tests/session.rs`) | all ports; the test *is* the loop |
| iced / relm4 / Elm | TEA already; add beliefs + lift + gateway |
| rtic / embassy | interrupt/tick is the Executive; `no_std` ports only |
| kube controller reconcile | `reconcile()` is the pulse |
| game tick / ROS spin | same |
| paper replay | lift + `apply`, no I/O |

This is `tracing` without `tracing-subscriber`. If exec were a required crate, firmware and GUI hosts would not come.

### 3. Core without newton-machine is thinner, but real

`IntentionMachine` is a **port**. A handwritten enum, a flat TEA `Model`, a wrapper around someone else’s FSM can impl it. Beliefs, mandate, lift, and gateway do not require XOR/AND. The family is *strongest* when `newton-machine` is the reference intention crate. It must not be a hard dep of `newtonian-core` at `0.0.0` (still [[docs/adr/0014-intention-machine-is-a-port]]).

Adapter `impl IntentionMachine for newton_machine::Runtime<M>` can live behind an optional feature later, or in an examples/adapter. Not a third crate.

## Decision

**Publish two crates, not three:**

| Crate | Owns |
| --- | --- |
| `newton-machine` | UCA chart (intentions / **truth**) |
| `newtonian-core` | **policy kernel** (ports at 0.0.0; scores + exact ROM as the product) + optional `exec` module |

**Do not publish `newton-exec`.** The pulse lives at `newtonian_core::exec`.

Module law (still [[docs/adr/0005-core-is-ports-not-the-loop]], amended):

- The crate’s *identity* is ports. You can depend on `newtonian-core` and never touch `exec`.
- `exec` must not introduce sockets, tokio, or a required clock.
- A memory `BeliefStore` / lift helpers may live under `exec` and may require `std` / `alloc`. Feature-gate when they grow deps.
- Folding exec into `newton-machine` is still rejected ([[docs/adr/actor-soup]]).

When to split `exec` out later (not now): it grows a runtime-specific stack that even a feature cannot hide from compile times, or a second Executive (bevy schedule, ROS spin) deserves its own crate. Until then, two crates.

Cargo name: **`newtonian-core`** ([[docs/adr/0001-crate-identity-and-name-collision]]). Directory may remain `newton-core`.

## Consequences

- README family table has two rows, plus `::exec` as a module.
- Agents must not open `C:\dev\newton-exec`.
- Agents must not put `run()` on `newton_machine::Runtime`.
- `0.0.0` names the `exec` module; it does not claim a batteries loop.

## Related

- [[docs/adr/0002-family-is-three-crates]]
- [[docs/goals/ports-not-batteries]]
- [[docs/concepts/executive]]
- symbol:ProgramParts
