---
tags: [ontology]
node_type: concept
aliases: [five-way split]
---
# Four kinds of state

Plus a binder. The README table, as a note.

| Kind | Question | Lives in | Changes when |
| --- | --- | --- | --- |
| Configuration | Committed stance? | Newton machine | `apply(Msg)` |
| Beliefs | True right now? | Belief store | every sensor write; no `Msg` required |
| Mandate | Standing aims? | Mandate value | rarely; then a `Msg` |
| Authority | Allowed to the world? | Gateway | lock, quota, interlock — out of band |
| Executive | Who binds them? | `exec` module or the host | lifecycle, not a fact |

Context on the machine is **not** a fifth kind. It is a slice of beliefs (and maybe mandate) copied in for a guard.

## Related

- [[docs/adr/0003-four-kinds-of-state]]
- [[docs/concepts/newtonian-program]]
