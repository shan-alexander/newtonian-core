# newtonian-core

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![crates.io](https://img.shields.io/crates/v/newtonian-core.svg)](https://crates.io/crates/newtonian-core)
[![docs.rs](https://docs.rs/newtonian-core/badge.svg)](https://docs.rs/newtonian-core)

**A configuration-indexed policy kernel.** Newton is UCA **truth**. This crate is gated scores → exact superstate key → compiled sleeve ROM → `Cmd`. The Executive (pulse) is still yours. 

> Okay so here's the gist. Several scenarios in software engineering can call for a state machine and this `newtonian-core` crate intends to integrate with a state machine and provide ideal rust design patterns for implementing desired behaviors as a reaction to the current state, with some unique features like sticky-state lifetimes (the engineer can optionally define how long a state should persist in 'current state' even when raw state no longer has that state). Typically, the running application will have a current state, derived data that is generated from the current state, and intended behaviors to execute when certain states are true which are somewhat like triggers/listeners of state... ideally all data can be converted into scores and boolean gates. you define the scoring methodology and `newtonian-core` is giving you a lightweight approach to ideal rusty design patterns (or helping your AI Agent deliver scalable maintainable code rather than sloppy code sprawl). The `newton-machine` rust crate provides ideal rusty design patterns for a Mealy, Harel-like state machine -- in other words, an AI Agent using `newton-machine` is guardrailed into proper, performant architecture and will get helpful compiler errors if the code violates any principles of a state machine built on Unidirectional Configuration Architecture (UCA). The `newton-machine`  (or any state machine) names what is true, and the `newtonian-core` is the policy kernel that encodes what to do in a state (or overlapping states). The host (ie you) owns the clock (all Mealy machines rely on a synchronous clock, whether its OHLCV bars, gaming frames, 10 ms vision snapshots for robotics, etc) and the host (you) also owns the adapters (your external api, or data broker, robotics motors, frame renderer, etc). Additionally, `newtonian-core` provides an approach to using a YAML file to declaratively configure the behavioral values, so that you can modify/tune the behavior without having to recompile. 

**Status:** `0.2.0` ships ports, the kernel (`Key`, exact `Table`, hysteretic scores, `max_age` / `disarm_mask`, `step_entity`), Fold (feature `fold`), `Lifted::Batch`, `PortExecutive`, and the `newton-machine` adapter (feature `machine`). `0.x` is not SemVer-stable. `unsafe` is forbidden. `#![no_std]` + `alloc` is the default shape. See [ADR 0016](docs/adr/0016-newtonian-core-is-the-policy-kernel.md) and [ADR 0020](docs/adr/0020-kernel-sits-beside-the-chart.md).

**crates.io name:** **`newtonian-core`**. The name `newton-core` is taken (an unrelated zkVM SDK). This github repo directory may stay `newton-core`. See [ADR 0001](docs/adr/0001-crate-identity-and-name-collision.md). We do **not** publish a third crate `newton-exec`: the pulse is the [`exec`](src/exec.rs) module. See [ADR 0015](docs/adr/0015-two-crates-exec-is-a-module.md).

This crate could be used in a variety of scenarios: \a session protocol, a robot sequencer, a UI shell, a cluster operator, a quant trading desk, and a firmware mode manager -- they can all use the same pattern I provide in `newtonian-core` + `newton-machine`.

## Start here

You are looking at one of **two** published crates:

| Crate | Question it answers | Ships / will ship |
| --- | --- | --- |
| [`newton-machine`](https://crates.io/crates/newton-machine) | What modal stance are we *committed* to? | Typed UCA chart: XOR enum, AND struct, TEA `update`, history sidecar |
| **`newtonian-core` (this crate)** | What should we **do**, given in-play predicates? | **Policy kernel:** hysteretic scores, exact bitset key, sleeve ROM, Fold YAML/TOML once. Optional `exec` module |

There is no `newton-exec` crate. The Executive is either **you** (a test, iced, rtic, kube reconcile) or `newtonian_core::exec`. Do not fold it into the machine. Do not call this “the behavior layer.” Those two mistakes are how this family becomes actor soup.

This crate is useful **without** `newton-machine` (impl `IntentionMachine` yourself) and **without** `exec` (bring your own pulse). `exec` is not useful without these ports. That is why it is a module.

Sibling crate [`newton-machine`](https://github.com/shan-alexander/newton-machine) is the **intention** machine (Bratman / Rao & Georgeff). 

If you clone the repo to tinker with the crate, you can use the `rustbrain` CLI to learn a lot about the architecture: [rustbrain](https://docs.rs/rustbrain) under `docs/`. A new agent should run `rustbrain context "policy kernel"` then `rustbrain context "what is a newtonian program"` before editing. Seed note: [docs/concepts/policy-kernel.md](docs/concepts/policy-kernel.md).

## The five things a live program actually is

People mash these into one struct. That is the bug this family exists to refuse.

| # | Kind | Question | Lives in |
| --- | --- | --- | --- |
| 1 | **Configuration** (intention) | What modal stance are we committed to? | Newton machine |
| 2 | **Beliefs** | What do we take to be true *right now*? | Belief store |
| 3 | **Mandate** | What are we trying to achieve — standing orders? | Mandate (policy / ends) |
| 4 | **Authority** | What are we allowed to do to the world? | Gateway |
| 5 | **Executive** | Who binds 1–4 to sensors, effectors, and humans? | `newtonian_core::exec`, **or the host** |

This crate types **2, 3, 4** and the **ports** that 5 needs against 1. Item 5 is an optional module. Hosts with a pulse already should not use it.

```text
sensors, clocks, operator
            │
            ▼
      ┌─────────── Executive (host or ::exec) ───────────┐
      │  ingest → revise beliefs                       │
      │  maybe lift a belief into a Msg                │
      │  step the Newton machine                       │
      │  hand Cmds to the gateway                      │
      │  refresh subscriptions / view                  │
      └───────────┬────────────────────────────────────┘
                  │
         Msg      │      Cmd
                  ▼
           Newton machine          Gateway → world
           (intentions)            (authority)
```

Interactive behavior is **this loop**. It is not “more states.” A UI, a sensor feed, a fill stream, a systemd watchdog, and a kill switch are all Executive *ports*.

A belief should enter the chart only when it changes what we are committed to. Last print moving from 100.01 to 100.02 is a belief update. Last print aging past a staleness bound is a `Msg`. Mixing those two is how `on_bar` became 400 lines — in *any* domain.

## Why this is not soup

The novelty is not “we invented state machines” and not “we invented BDI.” Each legendary crate already got **one** of these right and then violated another. The contribution is a **closed set of constraints** — the same move `newton-machine` made for UCA.

| Constraint | Meaning |
| --- | --- |
| The chart is intentions | Nested ADTs, TEA `update`, persistable `{config, context, history}` |
| Beliefs are not context | The store is larger, justified, freshness-stamped, and can change without a transition |
| Mandate is not a XOR child | Standing aims survive Halted → Armed; changing them is a `Msg`, not a region |
| Effects are not in the chart | `Cmd` is data. The Executive (or the host) executes. Gateway admits |
| Lift is the only door from beliefs to `Msg` | Production rules as a thin layer, not a Rete machine and not `on_enter` closures |
| Authority is outside both | The machine may lock itself. Only the gateway may lock the wire |

If you strip the laws you are back at Harel + callbacks. If you strip hierarchy you are back at Elm. If you put effects in actions you are back at XState. If you keep an untyped intention stack you are back at PRS. Honesty matters.

## What we steal, and what we leave

Ranked. These are the ideas this crate *absorbs*. We do not port the crates.

1. **BDI** (Bratman; Rao & Georgeff; PRS / dMARS) — the ontology. Machine = intentions. Beliefs = information state. Mandate = desires. **Leave:** BDI-CTL, Prolog facts, an untyped intention stack.
2. **3T Executive** (Bonasso, Firby RAPs, Gat ATLANTIS) — placement in the stack. Planner on top, sequencer in the middle, reactive skills at the bottom. **Leave:** RAPs as a replacement for the typed chart.
3. **Elm `Program`** — purity of `update`, interactivity via `Cmd` / `Sub`. **Leave:** a flat `Model` as the only state.
4. **XState interpreter** — running instance, subscribe, snapshot, start/stop. **Leave:** effects as closures inside machine actions.
5. **Truth maintenance** (Doyle JTMS; de Kleer ATMS) — facts carry justifications and can be withdrawn. **Leave:** a full TMS as a required dependency; start with source + freshness.
6. **Blackboard** (Hearsay-II) — many writers, one lift policy. **Leave:** every writer calling `apply` on the machine.
7. **OTP** — `gen_statem` is the machine; `gen_server` + supervisors are Executive and watchdog. They die independently. **Leave:** one process that is both chart and socket.
8. **Subsumption** — only for L0/L1 reflexes that must override the Executive. **Leave:** subsuming inside the chart.
9. **Behavior trees / GOAP / HTN** — fine as a *deliberator* that proposes mandate changes (`Msg::SeekSafe`). **Leave:** replacing the intention machine.
10. **Production rules** — “if belief and config then Msg.” That is lift. **Leave:** Rete as the core loop.

Full steal sheet (JS / Java / C++ / Python / Rust / NASA / games): [docs/concepts/steal-sheet.md](docs/concepts/steal-sheet.md).

## Name sheet

Keep the vocabulary stable so “Newton” does not mean five things.

| Term | Meaning |
| --- | --- |
| Newton machine | UCA chart: typed config + TEA update + history |
| Intention | current configuration (BDI sense) |
| Belief store | justified, freshness-stamped world facts |
| Mandate | standing desires / policy, **not** a state |
| Executive | live binder: sense, revise, step, effect, present |
| Gateway | wire authority; admits or refuses `Cmd` |
| Skill / effector | tiny reflex the Executive may run (heartbeat, enable, COD pulse) |
| Lift | belief × mandate × config → `Msg` (or silence) |
| Context | the slice of beliefs the *last step* needed; not the store |
| Program | one Executive + one or more machines + one gateway |
| Desk / robot / shell | a Program in a particular domain. Not a crate name |

## What this crate ships at 0.2.0

Traits, vocabulary, and a **handwritten** kernel. No sockets. No YAML interpreter. No YAML chart loader.

- `Key` / `ScoreSpec` / `Table` / `Folded` / `step_entity` — gated scores → exact ROM. Unauthored → none.
- Feature `fold` — YAML/TOML → interned `Policy` **once**. New score/play *kinds* stay Rust. No document in `step`.
- `IntentionMachine` — ports `newton-machine::Runtime` (and any other UCA host) must present: `apply`, `view`, `snapshot`, `restore`, `in_state`. Feature `machine` impls it for `Runtime<M>`.
- `BeliefStore` — revise, withdraw, read, freshness; `MemoryStore` behind `alloc`
- `Mandate` — standing aims; typically a small immutable struct
- `Lift` — category changes become messages (`Silence` / `Msg` / `Batch`); ticks do not
- `PortExecutive` — named lift → apply → admit pulse; host still does I/O and kernel `step`
- `Gateway` — admit / refuse `Cmd`; no I/O in the trait
- `Skill` — named reflex the Executive may invoke; still data in, data out
- `ProgramParts` — the composition type (handles, not the loop; kernel sits beside it)
- `exec` — optional sequencer module (named pulse; not a required loop)

YAML/TOML is for **knobs and which exact chords have firepower**, compiled **once** (Fold). Chart topology and score *kinds* are Rust. The kernel does not interpret a document every tick. See [ADR 0012](docs/adr/0012-yaml-is-knobs-not-topology.md) and [ADR 0018](docs/adr/0018-fold-yaml-once-never-interpret-topology.md).

## What this crate refuses

- A *required* batteries-included Executive. `exec` is optional; many hosts *are* the loop.
- A third crate `newton-exec`. See [ADR 0015](docs/adr/0015-two-crates-exec-is-a-module.md).
- I/O inside lift, inside the machine, or inside a belief revision that “helpfully” calls the world.
- Domain vocabulary (orders, bars, ROS topics, DOM events) in core types. A bar, a game frame, and a 10 ms vision sample are **host names** for one Mealy pulse.
- Longest-subset as the kernel matcher (exact key; unauthored → none).
- Sleeve ROM or gated eval inside `newton-machine::update`.
- Folding beliefs into the chart as fake states (`MaybeConnected`, `MaybeLong`).
- Every sensor writer calling `apply`.
- Implementing BDI logics, Soar, ACT-R, or Pearl nets as the store.
- SCXML, GoF `Box<dyn State>`, one thread per region.

## Domains (the pattern is the product)

Same ports. Different sensors and gateways.

| Domain | Intention (machine) | Beliefs | Mandate | Gateway |
| --- | --- | --- | --- | --- |
| Session / protocol | Offline / Connecting / Online{auth, sync} | last RTT, peer hello, clock skew | reconnect budget, idle timeout | socket + TLS |
| Robot / 3T | Navigate / Dock / Idle | pose freshness, bump, battery | “be docked by 02:00”, keep-out zones | motor / estop hardware |
| UI shell | Route × Modal | form dirty, auth cookie, feature flags | “never lose unsaved”, a11y | DOM / IPC |
| Cluster operator | Desired × Observed phase | API server facts, pod conditions | SLO, disruption budget | Kubernetes API |
| Firmware | Sleep / Run / Calibrate | ADC, thermal, bus fault | duty cycle, safety interlock | GPIO / IRQ (skills may preempt) |
| Game AI | Stance × Alert | blackboard: last seen, cover | “hold the point”, ammo conserve | animation / navmesh |
| Robot / vision | Navigate / Dock | pose, 10 ms frames | keep-out, dock-by | motors / estop |
| Trading desk | Risk × Session × Book | mark, locate, feed age | “flat by 15:55”, 1% daily loss | broker / venue |

The last rows are *examples*. They are not the crate. Same kernel: scores on a pulse → exact key → ROM → `Cmd`.

## Install

```toml
[dependencies]
newtonian-core = { version = "0.2", features = ["machine"] }
newton-machine = "0.2"
```

MSRV **1.80**.

## Examples (GitHub only)

Not shipped on crates.io. Domain types stay in the example, not in this crate.

```bash
cargo run --example aapl_1m --release --features machine
cargo run --example mosquito --release --features machine,fold
```

Feature `machine` implements `IntentionMachine` for `newton_machine::Runtime<M>`. It is optional so ports-only hosts never take a `newton-machine` dependency.

## rustbrain

This repository is indexed by [rustbrain](https://docs.rs/rustbrain). Goals, ADRs, concepts, and edge cases live under `docs/`.

```bash
rustbrain setup --yes
rustbrain context "what is a newtonian program"
rustbrain query "executive" --scores
```

A new AI agent with no prior session **must** read, in order:

1. This README
2. [docs/concepts/policy-kernel.md](docs/concepts/policy-kernel.md)
3. [docs/goals/configuration-indexed-policy-kernel.md](docs/goals/configuration-indexed-policy-kernel.md)
4. [docs/adr/0016-newtonian-core-is-the-policy-kernel.md](docs/adr/0016-newtonian-core-is-the-policy-kernel.md)
5. [docs/adr/0017-exact-superstate-key-not-longest-subset.md](docs/adr/0017-exact-superstate-key-not-longest-subset.md)
6. [docs/adr/0020-kernel-sits-beside-the-chart.md](docs/adr/0020-kernel-sits-beside-the-chart.md)
7. [docs/adr/0010-not-a-domain-crate.md](docs/adr/0010-not-a-domain-crate.md)

## License

MIT OR Apache-2.0.
