# Examples

GitHub-only. Not part of the crates.io package (`include` in `Cargo.toml`
skips this directory). Domain types live **here**, not in `newtonian-core`
(`docs/adr/0010-not-a-domain-crate.md`).

Requires default `std` and feature `machine` (`IntentionMachine` for
`newton_machine::Runtime<M>`). Cargo does not auto-enable `required-features`
on `cargo run --example`.

```bash
cargo run --example aapl_1m --release --features machine
cargo run --example mosquito --release --features machine,fold
```

| Example | Clock | What it shows |
| --- | --- | --- |
| `aapl_1m` | 1-minute bar (April 2026 lake CSV) | Host EMA/stoch, Newton `Desk` (quad × ema), lift on stance change, exact sleeve ROM, paper gateway, snapshot restore |
| `mosquito` | 500 ms fake camera | Size/speed gates, hysteretic confidence, lure → track → fire → cooldown, interlock Halt |

Neither example opens a socket, a broker, or a laser. `Cmd` is printed.
