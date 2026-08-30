//! A configuration-indexed **policy kernel** plus the ports of a Newtonian program.
//!
//! End-goal: gated hysteretic scores → exact superstate key → compiled sleeve
//! ROM → `Cmd`. `0.1.0` ships a **handwritten** kernel ([`kernel`]):
//! [`Key`], [`Table`], [`ScoreSpec`], [`step_entity`]. Feature `fold` compiles
//! YAML/TOML **once** into an interned ROM (no document in `step`). Pair with
//! `newton-machine` for typed
//! modal truth (optional feature `machine`: [`IntentionMachine`] for
//! `newton_machine::Runtime<M>`). The optional sequencer is [`exec`] — a
//! **module**, not a third crate. The host owns the pulse (tick, frame, sample)
//! and the adapters.
//!
//! The kernel sits **beside** the chart: [`step_entity`] does not call
//! [`IntentionMachine::apply`]. See [[docs/adr/0020-kernel-sits-beside-the-chart]].
//!
//! Hosts that already have a pulse (a test, iced, rtic, a kube reconcile)
//! should ignore [`exec`] and call the ports themselves.
//!
//! # Four kinds of live state, and a binder
//!
//! People mash these into one struct. Do not.
//!
//! 1. **Configuration / intention** — the Newton machine (`IntentionMachine`).
//! 2. **Beliefs** — justified, freshness-stamped world facts (`BeliefStore`).
//! 3. **Mandate** — standing desires / policy, not a XOR child (`Mandate`).
//! 4. **Authority** — what we may do to the world (`Gateway`).
//! 5. **Executive** — the binder, not a fifth kind of state. Optional module
//!    [`exec`]; you may *be* it.
//!
//! # Laws this crate encodes
//!
//! - Beliefs are not context. Context is the slice of beliefs the last step
//!   needed. The store is larger and can change without a transition.
//! - A belief becomes a `Msg` only through [`Lift`]. Ticks are not messages.
//! - Effects are data. [`Gateway`] admits or refuses a `Cmd`; it does not live
//!   inside `update`.
//! - Mandate changes rarely. When it changes, that is a message, not a region.
//! - Kernel lookup is exact. Unauthored key → none. No longest-subset default.
//! - No I/O in these traits or in [`step_entity`]. No sockets. No domain vocabulary.
//!
//! rustbrain: [[docs/concepts/policy-kernel]]
//! rustbrain: [[docs/concepts/newtonian-program]]
//! rustbrain: [[docs/adr/0016-newtonian-core-is-the-policy-kernel]]
//! rustbrain: [[docs/adr/0020-kernel-sits-beside-the-chart]]
//! rustbrain: [[docs/adr/0015-two-crates-exec-is-a-module]]
//! symbol:step_entity symbol:IntentionMachine symbol:BeliefStore symbol:Mandate symbol:Lift symbol:Gateway symbol:Executive

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod belief;
pub mod exec;
#[cfg(feature = "fold")]
pub mod fold;
pub mod gateway;
pub mod intention;
pub mod kernel;
pub mod lift;
pub mod mandate;
pub mod prelude;
pub mod program;
pub mod skill;

#[cfg(feature = "machine")]
mod machine_port;

#[cfg(feature = "alloc")]
pub use belief::MemoryStore;
pub use belief::{Belief, BeliefStore, Freshness, Justification};
pub use exec::Executive;
#[cfg(feature = "fold")]
pub use fold::{Catalog, FoldError, KnobId, Knobs, Policy, Sleeve, KNOB_SLOTS};
pub use gateway::{Admission, Gateway, OpenGateway};
pub use intention::{IntentionMachine, Step};
pub use kernel::{
    eval_scores, specs_valid, step_entity, EntityState, Folded, KernelStep, Key, Pulse, ScoreId,
    ScoreSpec, Table, Transition, SCORE_WIDTH,
};
pub use lift::{Lift, Lifted, Revision};
pub use mandate::{EmptyMandate, Mandate};
pub use program::{NoSkill, ProgramParts};
pub use skill::Skill;
