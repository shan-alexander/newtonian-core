---
tags: [lift, kernel, exec]
node_type: adr
---
# 0021 Batch lift, max-age, disarm, PortExecutive

## Status

Accepted

Does not change [[docs/adr/0011-lift-is-not-the-machine]] (lift still does not `apply`). Does not put I/O in `exec`. Desk-specific names (setup/arm/fire, stoch) stay in the host.

## Context

A live desk pulse often has several category changes at once (`SetupTrue` and `ArmTrue` and `FireTrue`). `Lifted` was one `Msg` or `Silence`, so hosts stuffed a `Vec` into `Msg` and `ProgramParts` could not apply honestly.

Sticky-only scores cannot express “in-play at most N own-ticks” or “force off when another classifier is in-play.” Those are still scores, not Harel history ([[docs/adr/0019-gated-hysteretic-scores-are-not-harel-history]]).

The Executive pulse was named in comments only.

## Decision

- [`Lifted::Batch`](symbol:Lifted) (`alloc`) is zero or more `Msg`s this pulse. `into_msgs` is the consumption API. Lift remains pure; the Executive applies each `Msg` in order.
- [`ScoreSpec::max_age`](symbol:ScoreSpec): own-ticks of in-play life. `0` = no cap. Count includes the rising pulse.
- [`ScoreSpec::disarm_mask`](symbol:ScoreSpec): if any of these **already in-play score bits** are set, force this score off (sticky and age discarded). Same DAG as enablement: disarm bits must have lower [`ScoreId`](symbol:ScoreId). Extra / Newton bits do not disarm (author a score).
- [`PortExecutive`](symbol:PortExecutive): lift → apply each msg → admit each cmd. No I/O, no kernel `step_entity`, no belief `revise`. Host still owns ingest, skills, and sleeve ROM.

## Consequences

- `Lifted` is no longer `Copy` when `Batch` exists.
- Fold documents may set `max_age` and `disarm_when: [names]`.
- Refuse still does not auto-inject a reconcile `Msg`; the host does.

## Related

- [[docs/adr/0011-lift-is-not-the-machine]]
- [[docs/adr/0019-gated-hysteretic-scores-are-not-harel-history]]
- [[docs/adr/0015-two-crates-exec-is-a-module]]
- [[docs/edge_cases/lift-storm]]
- symbol:Lifted symbol:ScoreSpec symbol:PortExecutive
