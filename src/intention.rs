//! Ports a Newton machine (or any UCA host) must present to an Executive.
//!
//! The reference implementation is `newton_machine::Runtime<M>`. Enable
//! feature `machine` for the blanket [`IntentionMachine`] impl (orphan
//! rules: that impl cannot live in a host crate). Without the feature, hosts
//! impl the trait themselves (`tests/session.rs`). See
//! [[docs/adr/0014-intention-machine-is-a-port]].
//!
//! Steal from XState: start / stop / send / subscribe / snapshot live on the
//! *running instance*, not on the chart. Leave XState's habit of putting
//! effect closures inside machine actions — [`Step::cmd`] is data.

/// One TEA step: the command the host (Executive / gateway) must execute.
#[must_use]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Step<Cmd> {
    /// Effect data. Empty means "nothing to do to the world."
    pub cmd: Cmd,
}

/// Ports of a committed-stance machine.
///
/// The Executive is allowed to:
///
/// - feed a `Msg` (`apply`)
/// - read a projection (`view`)
/// - persist / restore a phase-space point (`snapshot`)
/// - ask "are we in this node?" (`in_state`)
///
/// The Executive is **not** allowed to mutate configuration except through
/// `apply`. That is UCA law 2, restated as a port.
pub trait IntentionMachine {
    /// Applied force. Decision-relevant. Produced by [`crate::Lift`] or by an
    /// operator / sensor adapter the Executive owns.
    type Msg;

    /// Reaction data. Never a hidden call. The gateway sees this, not the
    /// machine.
    type Cmd;

    /// Projection for humans and telemetry. Read-only.
    type View;

    /// Persistable phase space. For a Newton machine this is
    /// `{config, context, history}`. Closures and sockets are forbidden here.
    type Snapshot;

    /// Node identity for `in_state`. Prefer a typed id over a string; `&'static
    /// str` is acceptable in handwritten machines.
    type NodeId: ?Sized;

    /// The only mutation door.
    fn apply(&mut self, msg: Self::Msg) -> Step<Self::Cmd>;

    /// Read-only projection of the current configuration (and whatever context
    /// the machine is willing to show).
    fn view(&self) -> Self::View;

    /// Clone the persistable triple. Restoring is a host concern: restore,
    /// *then* reconcile with the world via messages. Never treat a snapshot as
    /// a substitute for beliefs.
    fn snapshot(&self) -> Self::Snapshot;

    /// Harel-style query: is this node in the current configuration?
    fn in_state(&self, node: &Self::NodeId) -> bool;

    /// Inverse of [`IntentionMachine::snapshot`].
    ///
    /// Restore is not beliefs. After this, sensors must write fresh facts and
    /// lift may emit a resync `Msg`. See
    /// [[docs/edge_cases/restore-snapshot-is-not-beliefs]].
    fn restore(&mut self, snapshot: Self::Snapshot);
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Stub {
        n: u8,
    }

    impl IntentionMachine for Stub {
        type Msg = u8;
        type Cmd = ();
        type View = u8;
        type Snapshot = u8;
        type NodeId = str;

        fn apply(&mut self, msg: u8) -> Step<()> {
            self.n = msg;
            Step { cmd: () }
        }

        fn view(&self) -> u8 {
            self.n
        }

        fn snapshot(&self) -> u8 {
            self.n
        }

        fn in_state(&self, node: &str) -> bool {
            node == "armed" && self.n > 0
        }

        fn restore(&mut self, snapshot: u8) {
            self.n = snapshot;
        }
    }

    #[test]
    fn apply_is_the_only_door() {
        let mut m = Stub { n: 0 };
        assert!(!m.in_state("armed"));
        let _ = m.apply(1);
        assert!(m.in_state("armed"));
        assert_eq!(m.snapshot(), 1);
        m.restore(0);
        assert!(!m.in_state("armed"));
    }
}
