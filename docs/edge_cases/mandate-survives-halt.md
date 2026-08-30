---
tags: [mandate]
node_type: edge_case
---
# Mandate survives halt

Standing aims outlive a trip through `Halted` / `Locked` / `Offline`. If “be docked by 02:00” was a XOR child, halt would destroy it or force a fake history restore of policy.

Mandate is a value the Executive still holds. When the machine comes back, lift still sees the same aims. Changing the aims is a `Msg`, not an entry action.

## Related

- [[docs/adr/0007-mandate-is-not-a-xor-child]]
