---
tags: [family, program, goals]
node_type: goal
aliases: [Newtonian program, newtonian program]
---
# Establish the Newtonian program

`newton-machine` named the **chart**: Unidirectional Configuration Architecture. This crate names the **live program** that chart sits inside.

A Newton program is an Executive running one or more Newton machines against a belief store and a gateway.

Harel already defined kinematics. Elm already defined a mutation protocol. Bratman already defined three attitudes. Bonasso / Firby / Gat already placed an executive between planner and skills. XState already split chart from interpreter. None of them shipped the conjunction as a typed, persistable, domain-agnostic Rust family. That conjunction is the product.

## Goals

- Keep the five-way split named and typed: configuration, beliefs, mandate, authority, Executive. See [[docs/concepts/four-kinds-of-state]].
- Put the first four in `newtonian-core` as ports. Put the fifth in `newtonian_core::exec` **or in the host**. See [[docs/adr/0015-two-crates-exec-is-a-module]] and [[docs/adr/0005-core-is-ports-not-the-loop]].
- Make a new engineer (or a new AI agent) able to implement a session protocol, a robot sequencer, a UI shell, or a desk *without* inventing a sixth meaning of “state.”
- Absorb the ranked steal list without becoming a port of XState, PRS, or Spring SM. See [[docs/goals/absorb-legendary-patterns]].

## Non-goals

- A new computational class. If you strip the split you are back at an actor with a chart inside.
- Trading as identity. See [[docs/goals/versatile-not-a-desk]].

## Related

- [[docs/concepts/newtonian-program]]
- [[docs/concepts/name-sheet]]
- [[docs/adr/0004-call-it-the-executive]]
