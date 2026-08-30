---
tags: [bdi, bratman, prs]
node_type: concept
aliases: [BDI, Bratman]
---
# BDI mapping

You want Bratman filtered through Rao & Georgeff, not a Bayesian slogan.

A rational agent is three attitudes (Bratman 1987; Rao & Georgeff; PRS / dMARS):

- **Beliefs** — information state. Revisable facts about the world (last sample, mark freshness, pose, cookie, “feed is live”). This is not `Risk::Armed` and not `Conn::Online`.
- **Desires / mandate** — motivational state. Standing aims. A mandate can survive Halted and back. Not a XOR child.
- **Intentions** — deliberative commitments: plans you are not constantly reopening. Once you are `Connecting { attempt: 2 }` or `Long(Add1)` or `Docking`, you have committed. You do not replan the whole book on every tick. That is Bratman’s point: intentions stabilize conduct under time pressure.

PRS ran this way on the Shuttle RCS and Sydney air traffic: a belief database, a library of plans, an intention structure, an event queue. The Newton machine is a typed, persistable intention structure. The Executive is the PRS interpreter minus the Prolog romance.

Do not implement BDI-CTL. See [[docs/adr/bdi-ctl]].

## Related

- [[docs/goals/bdi-ontology-not-bdi-ctl]]
- [[docs/concepts/belief-store]]
- [[docs/concepts/mandate]]
