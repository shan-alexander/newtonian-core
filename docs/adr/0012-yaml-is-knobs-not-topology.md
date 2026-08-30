---
tags: [yaml, mandate, topology]
node_type: adr
---
# 0012 YAML is knobs, not topology

## Status

Accepted

## Context

Teams will ask for “a YAML state machine.” SCXML, XState JSON, Spring SM factories, and python-statemachine class attributes all make topology data. That fights UCA law 1 (the configuration is the type) and makes illegal configurations representable again.

They will also ask to tune thresholds without recompiling. That is legitimate.

## Decision

- **Chart topology** is Rust ADTs in a Newton machine. Not YAML, not JSON, not SCXML.
- **Knobs** (bounds, calendars, limits, name universes) are mandate / lift parameters. Files are allowed. They deserialize into a `Mandate` value, not into states.
- **Sleeve / play topology** (which exact keys have firepower) is data. Fold compiles it once into a ROM ([[docs/adr/0018-fold-yaml-once-never-interpret-topology]]). That is **not** a chart.
- **Score kinds / IR** stay Rust. New boolean kind = rustc. New threshold = file.
- Compile rules once. Do not interpret a document of transitions — or a document of sleeves — **every tick**.

## Consequences

- A “policy file” that lists XOR children is a misuse. Reject it in review.
- A file that lists sleeves and knobs is the kernel product, after Fold.
- A host (or later `exec` helpers) may load knobs. They still do not load charts.

## Related

- [[docs/concepts/mandate]]
- [[docs/adr/0007-mandate-is-not-a-xor-child]]
- [[docs/adr/0018-fold-yaml-once-never-interpret-topology]]
- [[docs/concepts/sleeve-rom]]
