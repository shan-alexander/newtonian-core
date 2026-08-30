---
tags: [elm, xstate, goals]
node_type: goal
---
# Elm purity, XState instance

Two thefts that must not be fused into one object.

**From Elm.** `update` is pure. Subscriptions and commands are how a program becomes interactive. `Browser.application` is the Executive. We already encoded this in `newton-machine` UCA law 2. `newtonian-core` restates it as ports so an Executive cannot “helpfully” put I/O back into lift or gateway-as-trait-with-a-socket.

**From XState.** The chart is data; the running thing is an interpreter (later an actor/service). It holds current state, handles delays, talks to the outside, snapshots, starts and stops. Borrow the *instance*. Do not borrow XState’s habit of putting effects inside machine actions.

The Executive is Elm’s Program and XState’s interpreter under one roof — and robotics’ sequencer, which is why we still call it the Executive and not `Interpreter`.

## Related

- [[docs/adr/0004-call-it-the-executive]]
- [[docs/adr/0009-effects-stay-in-the-executive]]
- [[docs/concepts/executive]]
