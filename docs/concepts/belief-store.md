---
tags: [belief, tms, blackboard]
node_type: concept
---
# Belief store

Justified, freshness-stamped world facts. The missing third pillar (machine + executive + **beliefs**). Not a rename of `Context`.

## Steal

- **TMS** (Doyle JTMS; de Kleer ATMS). Facts carry justifications. “Position = 400” is believed because drop-copy said so at *t*, not because the chart entered `Long`. If the justification is withdrawn (reconnect, busted fill, sensor fault), the belief goes stale without a fake state `MaybeLong`. The Executive lifts category changes into messages. The machine does not store the tape.
- **Blackboard** (Hearsay-II). Independent writers post; specialists react. Bar aggregator, fill handler, operator console, lidar driver, kube watch — writers. The Executive reads and decides which writings become `Msg`. Do not let every writer call `apply`.

## Leave unless you truly need them

Pearl belief nets as the *store* (use them as a regime estimator that *writes* a belief). AGM revision theory as a library. Soar / ACT-R.

## Related

- [[docs/adr/0006-beliefs-are-not-context]]
- [[docs/edge_cases/writers-must-not-apply]]
- symbol:BeliefStore symbol:Justification symbol:Freshness
