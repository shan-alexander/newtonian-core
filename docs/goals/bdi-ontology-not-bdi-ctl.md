---
tags: [bdi, goals]
node_type: goal
---
# BDI ontology, not BDI-CTL

Bratman (1987) and Rao & Georgeff (PRS / dMARS) gave us three attitudes. We steal the *mapping*, not the logics.

| Attitude | In this family |
| --- | --- |
| Beliefs | [[docs/concepts/belief-store]] |
| Desires | [[docs/concepts/mandate]] |
| Intentions | Newton machine configuration |

PRS already ran this way on the Shuttle RCS and Sydney air traffic: a belief database, a library of plans, an intention structure, an event queue. The Newton machine is a *typed, persistable* intention structure. The Executive is the PRS interpreter minus the Prolog romance.

## Leave

- BDI-CTL / BDI-CTL* as a crate feature
- AgentSpeak / Jason / JACK as a runtime
- Unbounded intention stacks of untyped plans
- Replanning the whole book on every tick (the point of intentions is that you do **not**)

## Related

- [[docs/adr/bdi-ctl]]
- [[docs/concepts/bdi-mapping]]
