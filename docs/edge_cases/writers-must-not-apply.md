---
tags: [blackboard, belief]
node_type: edge_case
---
# Writers must not apply

Bar aggregator, fill handler, lidar driver, kube watch, operator console: they **write beliefs**. They do not call `IntentionMachine::apply`.

If they do, you have N mutation doors, no lift policy, and a race between writers about what we are committed to. The Executive is the one lift policy.

Blackboard law. See [[docs/concepts/belief-store]].
