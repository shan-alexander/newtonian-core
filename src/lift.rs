//! Belief × mandate × configuration → `Msg` or silence (or several `Msg`s).
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
//! See [[docs/adr/0020-kernel-sits-beside-the-chart]]
//! [[docs/adr/0021-batch-lift-max-age-disarm-port-executive]].

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// What lift may emit after inspecting a revision.
///
/// One pulse may have several category changes. Prefer [`Lifted::into_msgs`]
/// over matching a single [`Lifted::Msg`]. Not `Copy` (`Batch` holds a `Vec`).
#[must_use]
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Lifted<Msg> {
    /// Nothing decision-relevant happened. The store already has the new fact.
    Silence,
    /// One category change. The Executive will `apply` this.
    Msg(Msg),
    /// Several category changes this pulse, in apply order.
    #[cfg(feature = "alloc")]
    Batch(Vec<Msg>),
}

impl<Msg> Lifted<Msg> {
    /// True when this pulse is silent.
    pub const fn is_silence(&self) -> bool {
        matches!(self, Lifted::Silence)
    }

    /// Collapse 0 / 1 / many into a `Vec` for the Executive.
    #[cfg(feature = "alloc")]
    pub fn into_msgs(self) -> Vec<Msg> {
        match self {
            Lifted::Silence => Vec::new(),
            Lifted::Msg(m) => alloc::vec![m],
            Lifted::Batch(v) => v,
        }
    }

    /// Build Silence / Msg / Batch from an iterator.
    #[cfg(feature = "alloc")]
    pub fn from_msgs(msgs: impl IntoIterator<Item = Msg>) -> Self {
        let mut v: Vec<Msg> = msgs.into_iter().collect();
        match v.len() {
            0 => Lifted::Silence,
            1 => Lifted::Msg(v.pop().expect("len 1")),
            _ => Lifted::Batch(v),
        }
    }
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
/// `lift` *after* `BeliefStore::revise` and *before* `IntentionMachine::apply`
/// (once per message in [`Lifted::Batch`]).
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

    #[cfg(feature = "alloc")]
    #[test]
    fn batch_is_several_msgs_one_pulse() {
        let b = Lifted::from_msgs(["setup", "arm", "fire"]);
        assert_eq!(b.into_msgs(), ["setup", "arm", "fire"]);
        assert!(Lifted::<u8>::from_msgs([]).is_silence());
        assert!(matches!(Lifted::from_msgs([1]), Lifted::Msg(1)));
    }
}
