//! Belief × mandate × configuration → `Msg` or silence.
//!
//! Lift is a thin production-rule layer. It is **not** the machine and it is
//! **not** the Executive loop. Drools / OPS5 / Rete are proof that "when
//! belief changes, assert Msg" wants a name. They are not a substitute for
//! UCA.
//!
//! Rule of thumb, restated from the seed:
//!
//! - Last observation moving from 100.01 to 100.02 is a **belief** update.
//! - Last observation aging past the mandate's staleness bound is a **Msg**.
//! - Mixing those two is how a sensor handler becomes 400 lines.
//!
//! Signature: inspect **`now` plus a [`Revision`] of the fact that changed**.
//! Do not clone the whole store to recover `previous`. Other keys are read
//! from `now`; the changed key's prior payload is `revision.previous`.
//! See [[docs/adr/0020-kernel-sits-beside-the-chart]].

/// What lift may emit after inspecting a revision.
#[must_use]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Lifted<Msg> {
    /// Category change. The Executive will `apply` this.
    Msg(Msg),
    /// Nothing decision-relevant happened. The store already has the new fact.
    Silence,
}

/// One belief key that just changed.
///
/// `previous` / `current` are `None` on first observe / withdraw. This is one
/// fact, not a store snapshot.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Revision<K, F> {
    /// Which slot changed.
    pub key: K,
    /// Payload before this revise / withdraw, if any.
    pub previous: Option<F>,
    /// Payload after. `None` if the fact was withdrawn.
    pub current: Option<F>,
}

impl<K, F> Revision<K, F> {
    /// Construct a revision.
    pub const fn new(key: K, previous: Option<F>, current: Option<F>) -> Self {
        Self {
            key,
            previous,
            current,
        }
    }
}

/// The only door from beliefs to the chart.
///
/// Implementations should be pure. No I/O. No `apply`. The Executive calls
/// `lift` *after* `BeliefStore::revise` and *before* `IntentionMachine::apply`.
/// [`crate::step_entity`] is a different door (policy ROM) and does not call
/// `apply` either.
pub trait Lift {
    /// The store (or a borrowed view of it) **after** the revision.
    type Beliefs;

    /// Standing aims.
    type Mandate;

    /// Current configuration / a queryable view of it. Prefer passing
    /// `machine.view()` or a small projection rather than the whole runtime.
    type Config;

    /// Messages the intention machine understands.
    type Msg;

    /// Belief key space. Match [`crate::BeliefStore::Key`].
    type Key;

    /// Fact payload. Match [`crate::BeliefStore::Fact`].
    type Fact;

    /// Inspect a *change*. Returning [`Lifted::Silence`] is the common case.
    fn lift(
        &self,
        now: &Self::Beliefs,
        revision: &Revision<Self::Key, Self::Fact>,
        mandate: &Self::Mandate,
        config: &Self::Config,
    ) -> Lifted<Self::Msg>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct AgeLift {
        bound: u64,
    }

    struct Age {
        tick: u64,
    }

    impl Lift for AgeLift {
        type Beliefs = Age;
        type Mandate = ();
        type Config = ();
        type Msg = &'static str;
        type Key = ();
        type Fact = u64;

        fn lift(
            &self,
            now: &Age,
            rev: &Revision<(), u64>,
            (): &(),
            (): &(),
        ) -> Lifted<&'static str> {
            let prev = rev.previous.unwrap_or(0);
            let crossed = prev < self.bound && now.tick >= self.bound;
            if crossed {
                Lifted::Msg("stale")
            } else {
                Lifted::Silence
            }
        }
    }

    #[test]
    fn ticks_are_silent_until_a_category_change() {
        let lift = AgeLift { bound: 10 };
        let now9 = Age { tick: 9 };
        assert!(matches!(
            lift.lift(&now9, &Revision::new((), Some(8), Some(9)), &(), &()),
            Lifted::Silence
        ));
        let now10 = Age { tick: 10 };
        assert!(matches!(
            lift.lift(&now10, &Revision::new((), Some(9), Some(10)), &(), &()),
            Lifted::Msg("stale")
        ));
    }
}
