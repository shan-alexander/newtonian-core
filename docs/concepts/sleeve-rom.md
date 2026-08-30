---
tags: [sleeve, rom, table, fold]
node_type: concept
aliases: [sleeve, ROM, play]
---
# Sleeve ROM

A **sleeve** is an authored row: exact superstate key → parameterized **behavior**. The interned table of sleeves is a **ROM** (Flyweight): all entities share it. Fold compiles YAML/TOML into `Table<Key, Sleeve> + KnobMap` once. Runtime is O(1) lookup. Unauthored key → NONE ([[docs/edge_cases/unauthored-key-is-none]]).

A sleeve is not only “if this chord, fire.” While it (or its sticky) holds, it may **open a gate**: further scores are allowed to become true; the exact key may grow `{A}` → `{A,B}` → `{A,B,C}` and select a more specific row; or emit update/cancel `Cmd`s. That growth is still **exact** lookup ([[docs/adr/0017-exact-superstate-key-not-longest-subset]]).

Behaviors emit **Command data** only (`Enter` / `Update` / `Exit` / `Cancel`, or `PlayClip { id, speed }` / `Stop`). Gateway admits. Host executes. Fill / animation-complete / motor-ack comes back as a **belief** (or a Newton `Msg` if it changes modal stance).

Knobs (`speed: 1.15`, `size_tier: 2`) live on the row. New *behavior kinds* are Rust. New numbers are files ([[docs/adr/0018-fold-yaml-once-never-interpret-topology]]).

Game sketch: enums `Grounded | Airborne`, `Combat | Explore` (Newton truth, optional); scores `LowHp`, `BossNear`. Sleeve `{Airborne, Combat, BossNear}` → `DiveAttack` with knobs `{ speed: 1.4, damage: 12 }`. Designer edits speed in TOML. New boolean `ParryWindow` is a new score kind → rustc.

## Related

- [[docs/concepts/policy-kernel]]
- [[docs/adr/0017-exact-superstate-key-not-longest-subset]]
- [[docs/adr/0020-kernel-sits-beside-the-chart]]
- [[docs/concepts/enablement-dag]]
- symbol:Table symbol:Folded symbol:step_entity symbol:Catalog symbol:Policy
