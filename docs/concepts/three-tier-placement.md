---
tags: [3t, robotics, plexil]
node_type: concept
aliases: [3T, ATLANTIS, RAPs, PLEXIL]
---
# Three-tier placement

Bonasso, Firby (RAPs), Gat (ATLANTIS): planner on top, sequencer / executive in the middle, reactive skills at the bottom. The executive activates and deactivates skill sets as the world changes. That is exactly “run the Newton machine against a live host.”

NASA PLEXIL / Universal Executive, DS1 Remote Agent (planner + Smart Executive + Livingstone), MIT RMPL: same grain — an executive is not the planner and not the controller.

SMACC2 (ROS 2, Boost.Statechart) is a C++ cousin that still tends to put effects in state callbacks. Steal compile-time charts. Leave I/O in states.

## Mapping

| 3T / ATLANTIS | Newton family |
| --- | --- |
| Deliberator / planner | optional; proposes `Msg::SeekSafe` / mandate changes. BT/GOAP/HTN live *here* |
| Sequencer / executive | `newtonian_core::exec`, or the host pulse |
| Skills / controller | `Skill` + gateway effectors. L0/L1 may subsume the Executive |
| (missing in 3T) | typed UCA intention + justified beliefs + pure admit |

Kill switch and heartbeat are skills. They do not live in XOR children.

## Related

- [[docs/concepts/executive]]
- [[docs/concepts/skill]]
- [[docs/adr/0004-call-it-the-executive]]
