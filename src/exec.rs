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
//!        Msg | Batch → IntentionMachine::apply each → Cmd
//!   → Gateway::admit each Cmd
//!        Admit  → host performs I/O
//!        Refuse → enqueue reconcile Msg
//!        Drop   → log / ignore
//!   → refresh Sub / view
//!   → pulse Skills
//! ```
//!
//! [`PortExecutive`] runs lift → apply → admit only. The host still revises
//! beliefs, pulses skills, and calls [`crate::step_entity`]. No I/O here.
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
//! rustbrain: [[docs/adr/0021-batch-lift-max-age-disarm-port-executive]]
//! rustbrain: [[docs/adr/0020-kernel-sits-beside-the-chart]]

/// One named pulse of a Newtonian program.
///
/// The trait is the sequencer port. Implementations may be [`PortExecutive`],
/// or a host loop that wants a type name for "I am the Executive." No I/O
/// in the trait.
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

/// Optional helper: lift → apply each [`crate::Lifted`] msg → admit each cmd.
///
/// Does **not** revise beliefs, pulse skills, or step the kernel. Does **not**
/// perform admitted effects. Refuse does not auto-inject a reconcile `Msg`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PortExecutive;

/// Result of [`PortExecutive::pulse`]. Cmds are inside [`Admission`](crate::Admission);
/// the host executes `Admit` out of band.
#[cfg(feature = "alloc")]
#[derive(Clone, Debug)]
pub struct PulseOutcome<Msg, Cmd, Reason, View> {
    /// Messages lift emitted this pulse (empty = silence).
    pub msgs: alloc::vec::Vec<Msg>,
    /// Admit / refuse / drop for each applied command, in apply order.
    pub admissions: alloc::vec::Vec<crate::Admission<Cmd, Reason>>,
    /// Machine view after every apply.
    pub view: View,
}

#[cfg(feature = "alloc")]
impl PortExecutive {
    /// Named ports pulse after the host has revised beliefs.
    ///
    /// `world` is the gateway's belief slice (not the whole store).
    /// `Msg` is cloned once so the outcome can report what was applied.
    pub fn pulse<M, B, P, G, L, S>(
        parts: &mut crate::program::ProgramParts<M, B, P, G, L, S>,
        revision: &crate::Revision<B::Key, B::Fact>,
        world: &G::World,
    ) -> PulseOutcome<M::Msg, M::Cmd, G::Reason, M::View>
    where
        M: crate::IntentionMachine,
        M::Msg: Clone,
        B: crate::BeliefStore,
        P: crate::Mandate,
        G: crate::Gateway<Cmd = M::Cmd, Mandate = P>,
        L: crate::Lift<
            Beliefs = B,
            Mandate = P,
            Config = M::View,
            Msg = M::Msg,
            Key = B::Key,
            Fact = B::Fact,
        >,
        S: crate::Skill,
    {
        let view = parts.machine.view();
        let msgs = parts
            .lift
            .lift(&parts.beliefs, revision, &parts.mandate, &view)
            .into_msgs();
        let mut admissions = alloc::vec::Vec::with_capacity(msgs.len());
        for msg in msgs.iter().cloned() {
            let step = parts.machine.apply(msg);
            admissions.push(parts.gateway.admit(step.cmd, &parts.mandate, world));
        }
        PulseOutcome {
            msgs,
            admissions,
            view: parts.machine.view(),
        }
    }
}

#[cfg(all(test, feature = "alloc"))]
mod tests {
    use super::*;
    use crate::prelude::*;

    struct Chart {
        n: u8,
    }

    impl IntentionMachine for Chart {
        type Msg = u8;
        type Cmd = u8;
        type View = u8;
        type Snapshot = u8;
        type NodeId = ();

        fn apply(&mut self, msg: u8) -> Step<u8> {
            self.n = self.n.saturating_add(msg);
            Step { cmd: msg }
        }
        fn view(&self) -> u8 {
            self.n
        }
        fn snapshot(&self) -> u8 {
            self.n
        }
        fn in_state(&self, (): &()) -> bool {
            true
        }
        fn restore(&mut self, n: u8) {
            self.n = n;
        }
    }

    struct Store;

    impl BeliefStore for Store {
        type Key = ();
        type Fact = ();
        type Source = ();
        fn revise(&mut self, (): (), (): (), _: Justification<()>) {}
        fn withdraw(&mut self, (): &(), (): &()) {}
        fn get(&self, (): &()) -> Option<&Belief<(), ()>> {
            None
        }
    }

    struct BatchLift;

    impl Lift for BatchLift {
        type Beliefs = Store;
        type Mandate = EmptyMandate;
        type Config = u8;
        type Msg = u8;
        type Key = ();
        type Fact = ();

        fn lift(&self, _: &Store, _: &Revision<(), ()>, _: &EmptyMandate, _: &u8) -> Lifted<u8> {
            Lifted::from_msgs([1, 2, 4])
        }
    }

    struct Open;

    impl Gateway for Open {
        type Cmd = u8;
        type Reason = ();
        type Mandate = EmptyMandate;
        type World = ();

        fn admit(&self, cmd: u8, _: &EmptyMandate, _: &()) -> Admission<u8, ()> {
            Admission::Admit(cmd)
        }
    }

    #[test]
    fn port_executive_applies_a_batch() {
        let mut parts = ProgramParts::new(
            Chart { n: 0 },
            Store,
            EmptyMandate,
            Open,
            BatchLift,
            crate::NoSkill,
        );
        let out = PortExecutive::pulse(&mut parts, &Revision::new((), None, None), &());
        assert_eq!(out.msgs, [1, 2, 4]);
        assert_eq!(out.view, 7);
        assert_eq!(out.admissions.len(), 3);
    }
}
