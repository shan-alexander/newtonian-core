//! Configuration-indexed policy kernel.
//!
//! Gated hysteretic scores → exact [`Key`] → interned [`Table`] → sleeve payload.
//! Same [`step_entity`] live / replay / test. No I/O. No YAML. No `apply`.
//!
//! rustbrain: [[docs/concepts/policy-kernel]]
//! rustbrain: [[docs/adr/0020-kernel-sits-beside-the-chart]]
//! rustbrain: [[docs/adr/0016-newtonian-core-is-the-policy-kernel]]
//! symbol:step_entity symbol:Table symbol:Key symbol:ScoreSpec

mod key;
mod score;
mod step;
mod table;

pub use key::{Key, Pulse, ScoreId, SCORE_WIDTH};
pub use score::{eval_scores, specs_valid, EntityState, ScoreSpec};
pub use step::{step_entity, Folded, KernelStep, Transition};
pub use table::Table;
