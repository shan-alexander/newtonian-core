---
tags: [rejected, naming]
node_type: adr
---
# Rejected: “the behavior layer”

## Status

Rejected

## Context

Authors reaching for a name after `newton-machine` exists will say “behavior engine,” “behavioral layer,” or `newton-policy`. Game AI uses “behavior tree.” Trading uses “policy.” Both would misfile this crate and invite folding scoring, lift, *and* the chart into one object.

## Decision

Rejected. The layer is the **Executive**. Mandate is policy-as-ends. Lift is category-change detection. Skills are reflexes. None of those is “behavior” as a crate name.

## Related

- [[docs/adr/0004-call-it-the-executive]]
