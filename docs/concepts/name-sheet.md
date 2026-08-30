---
tags: [vocabulary, naming]
node_type: concept
aliases: [vocabulary]
---
# Name sheet

Keep this stable so “Newton” does not mean five things.

| Term | Meaning |
| --- | --- |
| Newton machine | UCA chart: typed config + TEA update + history (**truth**) |
| Policy kernel | gated scores + exact superstate key + compiled sleeve ROM (**this crate’s product**) |
| Intention | current configuration (BDI sense); Newton XOR/AND or a bitset projection |
| Belief store | justified, freshness-stamped world facts (sensors, rings, frames, blackboard) |
| Score | hysteretic named boolean; latch + gated clock; **not** Harel history |
| Superstate key | exact bitset of in-play scores; unauthored → NONE |
| Sleeve / ROM | authored exact key → parameterized behavior; interned at Fold |
| Enablement DAG | parent score allows child scores to evaluate |
| Fold | YAML/TOML → bit indices + table + knobs **once**; never in `step` |
| Mandate | standing desires / knobs, **not** a XOR child |
| Executive | live binder / **pulse**: sense, revise, step, effect, present |
| Gateway | wire authority |
| Skill / effector | tiny reflex the Executive may run |
| Lift | belief × mandate × config → Newton `Msg` or silence (category-change) |
| Context | slice of beliefs the last step needed |
| Program | Executive + optional Newton + policy kernel + gateway |
| Pulse / clock | one Mealy tick: bar close, game frame, 10 ms vision sample — host names it |
| Desk / robot / shell | a Program in a domain; not a crate name |

README phrase: *Newton is UCA truth. newtonian-core is the configuration-indexed policy kernel. The Executive is still yours.*

Seed for agents: [[docs/concepts/policy-kernel]].

## Related

- [[docs/adr/0004-call-it-the-executive]]
- [[docs/concepts/newtonian-program]]
- [[docs/concepts/policy-kernel]]
- [[docs/adr/0016-newtonian-core-is-the-policy-kernel]]
