---
tags: [steal, research, crates]
node_type: concept
aliases: [prior art, analogues]
---
# Steal sheet

Prior art we studied so this crate is not soup. **Steal** the constraint. **Leave** the cheat. We do not port these libraries.

None of them is a Newton Executive. They are proof the layer wants a name distinct from the machine.

## The ranked list (family law)

See [[docs/goals/absorb-legendary-patterns]]. Short form:

1. BDI ontology (not BDI-CTL)
2. 3T Executive placement (not RAPs as the chart)
3. Elm Program purity
4. XState *interpreter* (not action closures)
5. TMS justifications (not a required JTMS crate)
6. Blackboard (not every writer `apply`)
7. OTP supervision (not one process for chart+socket)
8. Subsumption at L0/L1 only
9. BT/GOAP/HTN as deliberators
10. Production rules as lift

The **policy kernel** additionally steals: Esterel/Lustre **synchrony** (one pulse), decision-table / ROM lookup (Dijkstra guarded commands minus scanning every guard), Flyweight interned tables, Command + CoR. It **leaves**: Rete as the core loop, behavior trees as the step, YAML interpreted every tick. See [[docs/concepts/policy-kernel]].

## JavaScript / TypeScript

| World | Closest object | Steal | Leave |
| --- | --- | --- | --- |
| **XState** | `interpret(machine)` / actor | start/stop, send, subscribe, snapshot; chart is data | actions with closures; machine-as-actor soup in v5 |
| **Robot3** | tiny FSM | small public API | no hierarchy, effects in transitions |
| **Cycle.js** | Model–View–Intent | Intent as `Msg` stream; views as projection | FRP spaghetti; no intention tree |
| **Redux** | store + middleware | single unidirectional door | one flat store for everything |
| **redux-saga / observable** | epic / saga | `Cmd` execution outside the reducer | generators as hidden control state |
| **Elm** | `Browser.application` | pure `update`; `Cmd`/`Sub`; Program is the Executive | flat `Model`; no Harel, no TMS |

## Java / JVM

| World | Closest object | Steal | Leave |
| --- | --- | --- | --- |
| **Spring State Machine** | SM + listeners + guards | listener = Executive port | listeners that mutate freely; string states |
| **Squirrel / Stateless4j** | fluent FSM | explicit transitions | I/O in actions; no beliefs |
| **Akka / Pekko FSM** | actor + FSM | mailbox as pulse | actor owns the socket *and* the chart |
| **Drools / OPS5 / CLIPS** | production rules | “when belief changes, assert Msg” | Rete as the machine |
| **JACK / Jason / JADE / SPADE** | BDI platforms | explicit B/D/I stores; plan libraries | Prolog/AgentSpeak romance; untyped intention stacks |

## C++

| World | Closest object | Steal | Leave |
| --- | --- | --- | --- |
| **Qt** `QStateMachine` | event loop + machine | events on a loop, not in the chart | inheritance-heavy states; `onTriggered` I/O |
| **Boost.Statechart** | compiled HSM | ADT-ish configs, compile-time | effects in reactions; heavy templates |
| **Boost.SML** | `sml::sm` + transition table | table as data; guards; orthogonal regions; `process_event` | `/ action` lambdas that touch the world |
| **SMACC2** (ROS 2) | behavioral SM on Statechart | compile-time charts for robots; clients as skills | ROS I/O inside states |
| **YASMIN** | ROS 2 SM | small runtime | Python/C++ callbacks as design |
| **BehaviorTree.CPP / PyTrees** | BT + blackboard | blackboard ≈ beliefs; tickable skills | BT as a replacement for the intention chart |
| **PLEXIL / Universal Executive** | NASA plan executive | Executive as a *named* component; deterministic node tree | XML plans as topology; not UCA types |
| **NASA cFS / F Prime** | flight executive | command/telemetry, watchdogs as skills | not a typed chart |
| **DS1 Remote Agent** | planner + Smart Executive + Livingstone | three attitudes in flight software | 1990s Lisp/C++ stack, not our API |

## Python

| World | Closest object | Steal | Leave |
| --- | --- | --- | --- |
| **transitions** | callbacks on transitions | teaching toy for “there is a door” | callbacks *as* the design |
| **python-statemachine** | `on_enter_*` / listeners | listeners from *outside* (almost an Executive port) | I/O in `on_enter`; class-as-topology |
| **SPADE** | Python BDI | explicit agents | not UCA |

## Erlang / OTP

| World | Closest object | Steal | Leave |
| --- | --- | --- | --- |
| **`gen_statem`** | the machine | callback mode; keep state in the behaviour | not nested ADTs; effects in callbacks |
| **`gen_server` + supervisors** | Executive + watchdog | die independently; snapshot elsewhere | people still put the socket in the statem |

## Games / Unreal / AI

| World | Closest object | Steal | Leave |
| --- | --- | --- | --- |
| **Unreal** | BT + controller + blackboard | blackboard ≈ beliefs; controller ≈ Executive | BT replacing the chart |
| **GOAP / HTN** | planner | propose mandate / `Msg::SeekCover` | replacing intentions with a plan per tick |
| **Subsumption** (Brooks) | layered reflexes | L0/L1 override | subsuming inside XOR children |

## Rust (chart crates and TEA hosts)

| World | Closest object | Steal | Leave |
| --- | --- | --- | --- |
| **`newton-machine`** | UCA chart | nested ADTs, TEA, sidecar, `Cmd` as data | not beliefs, not Executive (on purpose) |
| **statig** | hierarchical typed SM | typed states | actions in handlers |
| **smlang** | macro FSM | small DSL | no UCA laws |
| **rust-fsm / machine** | FSM / typestate | typestate façade idea | lattice explosion; no sidecar |
| **iced / relm4 / yew** | TEA | `Cmd`/`Sub`/pure update | flat model; GUI runtime |
| **actix / ractor** | actors | mailbox as a pulse | soup |
| **embassy / rtic** | embedded executors | skill-level loops, `no_std` | not a chart |
| **serde** | snapshots | `{config, context, history}` and beliefs as data | putting sockets in Serialize |

## What top-tier engineers actually do with machines

Convergent practice, restated as rules we encode:

1. **Keep the chart pure.** The people who can replay, fuzz, and snapshot a controller are the people who kept I/O out of `update` (Elm, some XState users who only use `.transition()`, aerospace executives).
2. **Put a sequencer between planning and reflex.** Robotics 3T, NASA executives, game AI controllers. The chart is not the sensor loop.
3. **Do not reopen intentions every tick.** Bratman; also every trader who does not flatten on every print; every UI that does not re-route on every keystroke.
4. **Supervise I/O separately from committed stance.** OTP. A socket death must not imply “we forgot we were Connecting.”
5. **Name the blackboard.** Games and Hearsay-II. Writers are many; the lift policy is one.
6. **Authority is not a state.** Aerospace estop, kube admission, broker kill. If the only lock is a XOR child, you are already late.

That is the desired outcome this family is built to make *the default*, not a style guide.

## Related

- [[docs/goals/absorb-legendary-patterns]]
- [[docs/concepts/newtonian-program]]
- [[docs/adr/0009-effects-stay-in-the-executive]]
