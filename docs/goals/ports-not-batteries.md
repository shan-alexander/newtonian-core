---
tags: [ports, scope, goals]
node_type: goal
---
# Ports, not batteries

`newtonian-core` **0.0.0** defines the ports an Executive needs. It does not *require* our Executive. Hosts that already have a pulse ignore `exec`.

The **end-goal** is still not batteries-included I/O. It *is* a policy kernel (scores + exact ROM + knobs) in this same crate. The kernel is not a game engine and not a broker. See [[docs/goals/configuration-indexed-policy-kernel]].

Elm's `Browser.application` is a host. XState's `interpret(machine)` is a host. OTP's `gen_server` is a host. If those live in the **chart** crate, authors put I/O in actions. Putting an optional pulse in the *ports* crate is different — and it is still not a third crates.io name. See [[docs/adr/0015-two-crates-exec-is-a-module]].

## Goals

- Traits: `IntentionMachine`, `BeliefStore`, `Mandate`, `Lift`, `Gateway`, `Skill`, `ProgramParts`.
- No required `run()`, no sockets, no clocks, no async runtime, no YAML loader.
- `newtonian_core::exec` may later provide a pulse, a memory belief store, lift helpers, still no sockets. Feature-gate when deps grow.

## Related

- [[docs/adr/0005-core-is-ports-not-the-loop]]
- [[docs/adr/0013-no-std-and-no-sockets]]
- symbol:ProgramParts
