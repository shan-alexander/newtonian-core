---
tags: [sleeve, exact, miss]
node_type: edge_case
---
# Unauthored key is none

Under exact match, a pool with no ROM row is **NONE**: no fire, no inherited parent sleeve, no `contains` walk.

If `{A,B,C}` is in-play and the table only has `{A,B}`, the kernel does **not** fire `{A,B}`. Either Fold is missing a row the operator meant, or the extra bit is supposed to veto. Longest-subset would guess. The kernel refuses to guess ([[docs/adr/0017-exact-superstate-key-not-longest-subset]]).

Observe-only sleeves are still rows (they exist; they emit no `Cmd`). Absence of a row is not observe-only.

## Related

- [[docs/concepts/sleeve-rom]]
- [[docs/adr/0017-exact-superstate-key-not-longest-subset]]
- [[docs/adr/0020-kernel-sits-beside-the-chart]]
- symbol:Table symbol:step_entity
