---
tags: [non-goals, scope]
node_type: goal
---
# Non-goals of newton-core

A crate is defined as much by what it refuses as by what it ships.

## Will not ship in this crate

- A *required* Executive loop (`run`, tokio, clocks, sockets). Optional `exec` is a module, not a third crate. See [[docs/adr/0005-core-is-ports-not-the-loop]] and [[docs/adr/0015-two-crates-exec-is-a-module]].
- A generic SCXML / XState JSON interpreter.
- GoF `Box<dyn State>` as the intention model (that is `newton-machine`'s refusal; we do not undo it).
- BDI-CTL, Soar, ACT-R, Pearl nets as the belief store.
- Rete / Drools as the core loop. Lift may *look* like a tiny rule list.
- YAML (or JSON, or SCXML) as **chart** topology. Sleeve YAML after Fold is the kernel product, not a chart. See [[docs/adr/0012-yaml-is-knobs-not-topology]] and [[docs/adr/0018-fold-yaml-once-never-interpret-topology]].
- Domain vocabulary: orders, bars, ROS messages, DOM events, pod specs. Clocks are tick / frame / sample.
- I/O in `Lift`, `Gateway::admit`, `BeliefStore::revise`, `Skill`, or kernel `step`.
- Folding the Executive into `newton-machine`.
- Folding the sleeve ROM into `newton-machine::update`.
- Calling this layer “the state machine,” “a game engine,” or “the behavior layer.” The job title is **policy kernel** ([[docs/concepts/policy-kernel]]).
- Longest-subset as the kernel matcher ([[docs/adr/0017-exact-superstate-key-not-longest-subset]]).
- A required pulse; `exec` stays optional.

## Still in scope later, not 0.0.0 claims

- A default `exec` pulse (named at 0.0.0; implemented later).
- Per-key mandate bounds (one [`Mandate::freshness_bound`] for the whole mandate is the hook).

## Related

- [[docs/adr/behavior-layer]]
- [[docs/adr/actor-soup]]
- [[docs/adr/rete-as-machine]]
