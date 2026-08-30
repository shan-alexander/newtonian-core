---
tags: [steal, research, goals]
node_type: goal
aliases: [steal sheet]
---
# Absorb legendary patterns

Do not publish a soupy crate that “has a state machine and some middleware.” Cherry-pick the *constraint* each legendary system got right, and refuse the place each one then cheated.

Ranked, and frozen as family law:

1. **BDI** — ontology. Machine = intentions. [[docs/concepts/bdi-mapping]]
2. **3T Executive** — placement. [[docs/concepts/three-tier-placement]]
3. **Elm Program** — purity of `update`, interactivity via Cmd/Sub.
4. **XState interpreter** — running instance, subscribe, snapshot, start/stop. Effects stay out.
5. **TMS / justifications** — beliefs have sources and can be withdrawn.
6. **Blackboard** — many writers, one lift policy.
7. **OTP supervision** — Executive and gateway die independently; machine state is the snapshot.
8. **Subsumption** — L0/L1 skills only, never inside the chart.
9. **BT / GOAP / HTN** — deliberators that propose mandate changes, not a substitute for the intention machine.
10. **Production rules** — lift. Not the core loop.

The full map across JS, Java, C++, Python, Rust, NASA, and games: [[docs/concepts/steal-sheet]].

## Related

- [[docs/goals/bdi-ontology-not-bdi-ctl]]
- [[docs/goals/elm-purity-xstate-instance]]
- [[docs/adr/0009-effects-stay-in-the-executive]]
