---
tags: [superstate, sleeve, exact]
node_type: adr
---
# 0017 Exact superstate key, not longest-subset

## Status

Accepted

## Context

A pool `{A,B,C,D}` can have authored chords `{A,B}`, `{A,C,D}`, `{A,B,D}`. Two lookup laws exist:

- **Exact:** key = the whole pool. Unauthored key → NONE. Authored key → one sleeve.
- **Longest-subset:** ignore extra atoms; `{A,B,C,D}` can fire `{A,B,D}` because `N=3 > 2`. Same-length `{A,C,D}` vs `{A,B,D}` is a race (`Tie::Refuse` or author order). `newton-machine` shipped that as a **host helper** (`ChordTable`), not as UCA law.

A policy kernel that five desks and a game share cannot default to subset match. Specificity in the kernel is “you authored the chord, so the key grew bits”: `{A}` → `{A,B}` → `{A,B,C}` via enablement, each an **exact** row. Unauthored products do not silently inherit a parent sleeve.

## Decision

The kernel’s ROM is **exact lookup**. Unauthored superstate → NONE ([[docs/edge_cases/unauthored-key-is-none]]).

Longest-subset stays **off** in `newtonian-core`. A host that wants it may use `newton-machine`’s `ChordTable` (or their own table) **outside** the kernel step. That is not the product.

Enablement DAG ([[docs/concepts/enablement-dag]]) is how a more specific sleeve becomes possible: parent in-play **allows** child scores to turn on, which **changes the exact key**. That is not subset fallback.

## Consequences

- Fold must intern every play the operator cares about. Missing a row is a miss, not a guess.
- Same-length races do not exist under exact match (keys are unique).
- `contains` as a default matcher is banned on the hot path.

## Related

- [[docs/concepts/sleeve-rom]]
- [[docs/concepts/policy-kernel]]
- [[docs/adr/0016-newtonian-core-is-the-policy-kernel]]
