---
tags: [program, seed, family]
node_type: concept
aliases: [Newtonian program, seed]
---
# Newtonian program

**Seed concept for agents and humans.** If you read one note in this repo besides the README, read this.

A **Newton program** is an Executive running optional Newton machines and a **policy kernel** against a belief store and a gateway.

Newton (`newton-machine`) is UCA **truth**: typed XOR/AND, TEA, snapshot. `newtonian-core` is the **policy kernel**: gated hysteretic scores, exact superstate key, compiled sleeve ROM ([[docs/concepts/policy-kernel]]). The Executive is still **yours** (game loop, 10 ms sequencer, `qsys-engine`, a test).

It is not a new automaton. It is the *live system* you get when you refuse to mash four kinds of state into one struct, refuse to put the world inside the chart, and refuse to pretend the chart is the sleeve product.

## Why this crate exists

`newton-machine` answers: *what are we committed to?* That is necessary and not sufficient. The moment a host has sensors, a clock, an operator, and something it is not allowed to do, authors invent:

- more XOR children (`MaybeConnected`, `FeedLooksStale`),
- callbacks on transitions,
- a god `Context` with a socket in it.

Call that layer the **Executive**. Do not call it “the behavior layer.” Do not fold it into the Newton machine. Those two mistakes are how this family collapses back into actor soup.

The machine is committed stance: you are `Online`, or `Armed`, or `Docking`. The Executive is practical reason in contact with the world: it senses, revises what it believes, offers messages to the machine, executes commands the machine is not allowed to run, and exposes the system to operators. That split is old. We steal it on purpose.

## The five-way split

| # | Kind | Question | Lives in |
| --- | --- | --- | --- |
| 1 | Configuration (intention) | What modal stance are we committed to? | Newton machine |
| 2 | Beliefs | What do we take to be true right now? | Belief store |
| 3 | Mandate | What are we trying to achieve, standing orders? | Mandate |
| 4 | Authority | What are we allowed to do to the world? | Gateway |
| 5 | Executive | Who binds 1–4 to sensors, effectors, humans? | `newtonian_core::exec`, or the host |

`newtonian-core` types **2, 3, 4** and the **ports** that 5 needs against 1. Item 5 is an optional module. Hosts with a pulse already should not use it.

```text
sensors, clocks, operator
            │
            ▼
      ┌─────────── Executive ───────────┐
      │  ingest → revise beliefs         │
      │  maybe lift a belief into a Msg  │
      │  step the Newton machine         │
      │  hand Cmds to the gateway        │
      │  refresh subscriptions / view    │
      └───────────┬─────────────────────┘
                  │
         Msg      │      Cmd
                  ▼
           Newton machine          Gateway → world
           (intentions)            (authority)
```

Interactive behavior is this loop. A UI, a bar feed, a lidar topic, a kube watch, and a kill switch are all Executive ports.

## What goes where

**Executive (interactive system)** — host pulse, or optional `exec` module:

- Sensor adapters
- Belief revision (write facts, mark freshness, justify source)
- Lift (only decision-relevant belief changes become `Msg`)
- `apply` on the machine
- Dispatch `Cmd` to the gateway (or UI, disk, alerts)
- Subscription management (Elm `subscriptions` as a function of config + beliefs)
- Views / telemetry projections
- Lifecycle: start, restore snapshot, stop, graceful lock
- Spawn/compose child machines (supervisor)

**Newton machine:**

- Configuration tree
- History sidecar
- Guards that read a *slice* of beliefs and mandate
- Transitions and emitted intents (`Cmd`)

**Mandate:**

- Limits, calendars, “must be done by,” name universe
- Changes rarely; when it changes, that is a `Msg`

**Gateway:**

- Authority. Unchanged by panic in the Executive logic. No I/O in the trait; I/O beside it.

**Belief store:**

- Justified facts. Last observation, pose freshness, cookie, pod condition, “feed is live.”
- Not `Risk::Armed`. Not `Conn::Online`.

A belief should enter the chart only when it changes what we are committed to. Last sample moving from 100.01 to 100.02 is a belief update. Last sample aging past a staleness bound is a `Msg`. Mixing those two is how a sensor handler became 400 lines in every language we surveyed.

## Novel contribution (honest)

We did not invent state machines, BDI, TEA, or three-layer robotics. We named a **closed set of constraints** that the legendary crates each violate in a different place, and we made that set *the public API of a Rust crate family*.

| If you only have… | You are missing… |
| --- | --- |
| Harel / SCXML / SML / Spring SM | TEA purity, persistable sidecar, beliefs ≠ states |
| Elm / iced / relm4 | hierarchical intentions, justified beliefs, authority |
| XState | effects-outside-the-chart as *law*, BDI split |
| PRS / JACK / Jason | typed persistable intention, UCA snapshots |
| 3T / PLEXIL / SMACC2 | typed chart + TEA + belief justifications as ports |
| OTP `gen_statem` | UCA nested ADTs, lift, mandate |
| Blackboard / TMS | a typed intention machine to lift *into* |
| Behavior trees | a committed stance that is not a tick of the tree |

The family is useful when at least two of these are true: hierarchy, orthogonality, history, replay, host-executed effects, beliefs that change without a transition, authority that can act when `update` is not on the path.

## Crate map

| Crate | Role |
| --- | --- |
| `newton-machine` | Intention machine (UCA) |
| `newtonian-core` | Ports, ontology, optional `exec` module (this crate) |

No third crate. See [[docs/adr/0015-two-crates-exec-is-a-module]] and [[docs/adr/0001-crate-identity-and-name-collision]].

## Related

- [[docs/concepts/name-sheet]]
- [[docs/concepts/steal-sheet]]
- [[docs/concepts/executive]]
- [[docs/concepts/bdi-mapping]]
- [[docs/goals/establish-the-newtonian-program]]
- [[docs/adr/0020-kernel-sits-beside-the-chart]]
- symbol:ProgramParts symbol:IntentionMachine symbol:BeliefStore symbol:step_entity
