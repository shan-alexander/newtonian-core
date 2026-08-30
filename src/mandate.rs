//! Standing desires / policy. Not a state.
//!
//! Bratman's *desires*. Rao & Georgeff's motivational state. In 3T this is
//! closer to what the planner hands the sequencer than to a RAP itself.
//!
//! A mandate can survive a trip through `Halted` and back. That is why it is
//! not a XOR child. When it *does* change, the Executive injects a `Msg`
//! (`MandateRevised` or domain equivalent) so the intention machine can
//! react — flatten, replan, lock — without the mandate living in the tree.
//!
//! Knobs that belong here: limits, calendars, "must be done by," universe of
//! names the program is allowed to talk about. Knobs that do **not**: chart
//! topology. Topology is Rust ADTs.

/// Standing aims the Executive passes into lift and (as a read-only borrow)
/// into guards.
///
/// Keep implementations small and preferably immutable. Clone on revise, do
/// not mutate in place on the hot path.
pub trait Mandate {
    /// Stable identity so snapshots and logs can say *which* mandate was in
    /// force. A hash, a version integer, or a `&'static str` name is enough.
    type Id: Clone + PartialEq;

    /// Identity of this mandate value.
    fn id(&self) -> Self::Id;

    /// Executive ticks after which a still-held fresh belief is stale.
    ///
    /// `None` (default) means the store's [`crate::BeliefStore::age`] should
    /// no-op unless the host ages slots itself. One bound for the whole
    /// mandate is crude; hosts with per-key bounds age in the Executive.
    fn freshness_bound(&self) -> Option<u64> {
        None
    }
}

/// A mandate with no standing orders. Useful in tests and in hosts that have
/// not yet grown policy.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EmptyMandate;

impl Mandate for EmptyMandate {
    type Id = ();

    fn id(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    struct Limits {
        version: u16,
        max_inflight: u8,
    }

    impl Mandate for Limits {
        type Id = u16;
        fn id(&self) -> u16 {
            self.version
        }
    }

    #[test]
    fn mandate_is_not_a_region() {
        let m = Limits {
            version: 3,
            max_inflight: 4,
        };
        assert_eq!(m.id(), 3);
        let _ = m.max_inflight;
    }
}
