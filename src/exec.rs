//! Optional Executive pulse.
//!
//! This is a **module**, not a crate (`docs/adr/0015-two-crates-exec-is-a-module`).
//!
//! A Newton program is this pulse against [`crate::program::ProgramParts`]:
//!
//! ```text
//! ingest sensor / clock / operator
//!   → BeliefStore::revise (or withdraw)
//!   → Lift::lift
//!        Silence → stop
//!        Msg     → IntentionMachine::apply → Cmd
//!   → Gateway::admit
//!        Admit  → host performs I/O
//!        Refuse → enqueue reconcile Msg
//!        Drop   → log / ignore
//!   → refresh Sub / view
//!   → pulse Skills
//! ```
//!
//! # When to ignore this module
//!
//! If you already have a pulse — a test, iced `update`, rtic interrupt, kube
//! `reconcile`, game tick, ROS spin — **you are the Executive**. Call the
//! ports yourself. `tests/session.rs` is that pattern.
//!
//! # What this module must never become
//!
//! - A tokio runtime, a socket, or a clock you are forced to use.
//! - A place that calls `apply` from writers (blackboard law).
//! - A fold of the chart into an actor (that is `newton-machine`'s job to
//!   refuse, and this module's job not to undo).
//!
//! The pulse is **named**. A default helper is not claimed. The policy kernel ([`crate::step_entity`])
//! is a sibling call on the same pulse, not this trait. See
//! [[docs/adr/0020-kernel-sits-beside-the-chart]].

/// One named pulse of a Newtonian program.
///
/// The trait is the sequencer port. Implementations may be this crate's
/// later helper, or a host loop that wants a type name for "I am the
/// Executive." No I/O in the trait.
///
/// `Parts` is usually [`crate::program::ProgramParts`].
pub trait Executive {
    /// The bag this pulse borrows.
    type Parts;

    /// Ingest → revise → lift → apply → admit → present. One step.
    /// Returning does not perform admitted effects; the host does.
    /// Implementations that also run the kernel call [`crate::step_entity`]
    /// here; this trait does not.
    fn pulse(&mut self, parts: &mut Self::Parts);
}
