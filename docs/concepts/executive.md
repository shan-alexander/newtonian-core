---
tags: [executive, 3t, elm, xstate]
node_type: concept
aliases: [Executive, sequencer]
---
# Executive

The Executive is the loop that binds configuration, beliefs, mandate, and authority to sensors, effectors, and humans.

It is Elm’s `Program`, XState’s interpreter, robotics’ sequencer, OTP’s `gen_server`+supervisor, and NASA’s Universal Executive under one roof. We use the robotics name because it means *sequencer between deliberation and reflex*, not “the chart itself” and not “a JS runtime.”

## Pulse

```text
ingest sensor / clock / operator
  → BeliefStore::revise (or withdraw)
  → Lift::lift
       Silence → stop
       Msg     → IntentionMachine::apply → Cmd
  → Gateway::admit
       Admit  → host performs I/O
       Refuse → enqueue reconcile Msg
       Drop   → log / ignore
  → refresh Sub / view
  → pulse Skills (or skills preempt on their own clock)
```

That pulse lives in `newtonian_core::exec` **or in the host**. Hosts that already have a loop should not import `exec`. See [[docs/adr/0015-two-crates-exec-is-a-module]].

## What it is not

- Not more XOR children.
- Not a behavior tree.
- Not `newton-machine::Runtime` (that is the TEA owner of `{config, context, history}`).
- Not the gateway (authority can and should outlive it).

## Related

- [[docs/adr/0004-call-it-the-executive]]
- [[docs/concepts/three-tier-placement]]
- [[docs/goals/elm-purity-xstate-instance]]
