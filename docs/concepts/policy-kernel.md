---
tags: [kernel, product, seed]
node_type: concept
aliases: [configuration-indexed policy kernel, policy kernel]
---
# Policy kernel

**Seed concept for agents building this crate.** If you read one note besides the README after 0.0.0, read this.

`newtonian-core`’s **end-goal** is a **configuration-indexed policy kernel**: hysteretic predicates form an exact superstate key; a compiled sleeve table binds that key to parameterized behaviors; YAML/TOML may change knobs and which chords have firepower, never the IR or the step law; execution is Command + gateway, replay-identical.

It is **not** “Newton with a hash table bolted on.” It is **not** a desk Executive with the broker serial numbers filed off. It is **not** a Rete. It is **not** a game engine.

Newton named the conservation laws (UCA). A Quant desk named a trading product (sleeves, TF arm, lake, broker). The extractable core is the **middle machine** a desk, a game, and a 10 ms robot would share.

```text
beliefs (sensors, rings, animation blackboard, vision frames)
    → gated hysteretic scores     [in-play predicates; not Harel history]
    → superstate key (bitset)     [exact configuration]
    → sleeve table (compiled)     [policy ROM]
    → Behavior(knobs) → Cmd       [intent]
    → Gateway                     [authority]
    → host adapter                [broker / gameplay / motors]
```

The host still owns the **pulse** ([[docs/concepts/synchronous-pulse]]) and the adapters. Optional Newton sits *beside* scores as typed modal truth (session / order / clip XOR), not *as* the ROM.

## Four classical objects, not a new automaton

1. **Synchronous pulse (Mealy, one clock).** One external tick per entity. No thread per region. Same function for live, replay, test. Esterel/Lustre synchrony + Elm `update`. Newton already insists on this for the chart; the kernel insists on it for scores→Cmd.
2. **Hysteretic classifiers (scores).** Named booleans with enablement, sticky, max age, own clock. Latch + gated clock. [[docs/concepts/hysteretic-score]]
3. **Exact product configuration.** Pool = superstate. Key = bitset (or packed Newton discriminants). Unauthored → NONE. Authored → one sleeve. Decision table / ROM, interned at Fold. [[docs/concepts/sleeve-rom]]
4. **Staged gates.** The pool is not a flat bag that always evaluates every score. Evaluation is an enablement DAG. Parent false → children forced off (or documented decay). Not a behavior tree (those are imperative, one-token-at-a-time). [[docs/concepts/enablement-dag]]

## What to call it (and what not)

| Name | Why it almost fits | Why it is the wrong job title |
| --- | --- | --- |
| Engine | Hosts think “game / trading engine” | Too big: lake, sockets, renderer, broker |
| Framework | YAML + traits | Implies you live inside it; 5 desks fight it |
| Interpreter | YAML knobs, IR | Must not interpret topology every tick |
| UCA / Newton machine | Truth as typed config, TEA, Cmd | Refuses YAML plays; no sleeve ROM; no gated eval DAG |
| Rules engine (Rete) | Many booleans | Rete is for sparse changing fact bases — not a pulse of 20 scores × N entities |
| OS kernel | Laws, syscalls, no bypass | Metaphor only |

Speech: **policy kernel**. CS: configuration-indexed policy kernel.

## Family placement

```text
Newton / UCA          typed modal truth, TEA, snapshot, no I/O
Policy kernel   ←──   gated scores + exact sleeve ROM + knobs   (this crate's product)
Executive             your pulse (qsys-engine, game loop, 10 ms sequencer)
Adapters              IBKR, other brokers, input/animation, motors
```

- Newton **without** the kernel: AAPL demo — truth is clean; plays are a handwritten `overlap()`. Five desks copy-paste policy.
- Kernel **without** Newton: scores + ROM as-is; session/order XOR is a pile of enums without LCA/history/snapshot. Fine for a first kernel ship; weaker as a family.
- Kernel “is” newton-machine + hash table: **wrong merge**. Sleeve ROM and gated eval would pollute UCA law 1.
- “Generic qsys-engine”: **wrong merge**. Pulse + lake + IB pacing is a desk.

## Engineer contract

[[docs/adr/0018-fold-yaml-once-never-interpret-topology]]: new *kinds* = rustc. Knobs and which chords exist = files. Enablement edges are the gray zone: strict (Rust stages) or desk-friendly (`enabled_by` names resolved at Fold).

## Design shapes (Rust, not a GoF kit)

| Intent | Kernel shape |
| --- | --- |
| Interpreter | Closed IR enum + `match`; YAML only after Fold |
| Fold / compiler | `RawDoc → BitIndex + Table<Key, Sleeve> + KnobMap` once |
| Flyweight | Interned table; all entities share it |
| Command | `Cmd` enum; host executes |
| Strategy | `B: Execute<Knobs>` on the hot path; `dyn` only at boot |
| Template Method | `step_entity`; hard gateway not a hook |
| CoR | Gateway chain |
| Type-state | fire permission only from a table hit |
| Newtype | `ScoreId`, `Key`, `EntityId`, clock |
| Data-oriented | SoA bitsets; no `Vec` per entity per tick; no YAML in `step` |

## Bans

I/O in `step`, string ids on the hot path, `contains` as default match, dual interpreter (live vs replay), `Rc<RefCell>` graphs, thread per region, closures in snapshots, topology-as-YAML interpreted every tick, longest-subset as the kernel law, domain types (bars, ROS, DOM) in this crate.

Performance **is** the representation: bitset + HashMap/perfect hash of authored keys, book-level scores once, skip disabled gates, parallel by entity, category-change optional when a Newton XOR actually moves. Criterion on `step` is part of the crate.

## Related

- [[docs/adr/0016-newtonian-core-is-the-policy-kernel]]
- [[docs/adr/0017-exact-superstate-key-not-longest-subset]]
- [[docs/adr/0020-kernel-sits-beside-the-chart]]
- [[docs/goals/configuration-indexed-policy-kernel]]
- [[docs/concepts/synchronous-pulse]]
- [[docs/concepts/newtonian-program]]
- symbol:step_entity symbol:Table symbol:Key symbol:ScoreSpec
