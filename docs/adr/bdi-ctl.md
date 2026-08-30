---
tags: [rejected, bdi]
node_type: adr
---
# Rejected: BDI-CTL as a library feature

## Status

Rejected

## Context

Rao & Georgeff logics are real. Implementing them would make this an agents-research crate and bury the ports.

## Decision

Rejected. Steal the ontology (belief / desire / intention as three stores). Do not implement BDI-CTL, AgentSpeak, Jason, or JACK.

## Related

- [[docs/goals/bdi-ontology-not-bdi-ctl]]
- [[docs/concepts/bdi-mapping]]
