---
tags: [rejected, actors]
node_type: adr
---
# Rejected: fold the Executive into the machine (actor soup)

## Status

Rejected

## Context

XState v5: a running machine *is* an actor. Actix: the actor *is* the state. Convenient. Then beliefs, sockets, and supervision all share one mailbox, snapshots contain the world, and a panic in I/O kills the intention.

## Decision

Rejected. The machine is committed stance. The Executive is practical reason in contact with the world. Gateway and Executive die independently. Child machines are composed by the Executive (or an OTP-style supervisor), not spawned from an entry action closure.

Steal XState’s instance API (start/stop/send/subscribe/snapshot). Leave “the machine is the actor.”

## Related

- [[docs/adr/0004-call-it-the-executive]]
- [[docs/adr/0009-effects-stay-in-the-executive]]
