//! Justified, freshness-stamped world facts.
//!
//! Beliefs are the information state (Bratman; Rao & Georgeff). They are
//! **not** `Risk::Armed` and they are **not** the machine's `context`.
//!
//! Steal from Doyle JTMS / de Kleer ATMS: a fact is believed *because* a
//! source said so at time *t*. If that justification is withdrawn (reconnect,
//! busted observation, operator correction), the belief goes stale without
//! anyone inventing a fake state `MaybeConnected`.
//!
//! Steal from Hearsay-II: independent writers post on a shared board. The
//! Executive reads. Lift decides which writings become `Msg`. Writers do
//! **not** call `apply`.
//!
//! Leave: Pearl belief nets as the store (a regime estimator may *write* a
//! belief), AGM revision theory as a library, Soar/ACT-R.
//!
//! `Fact` is one associated type. Heterogeneous boards are a host enum,
//! several stores, or a later column API — not type erasure in core.
//! See [[docs/adr/0020-kernel-sits-beside-the-chart]].

use crate::mandate::Mandate;

#[cfg(feature = "alloc")]
use alloc::collections::BTreeMap;

/// How fresh a belief is, from the Executive's point of view.
///
/// The clock lives in the Executive (or a skill). The store only records what
/// it was told.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Freshness {
    /// Observed at `tick` and not withdrawn.
    Fresh {
        /// Monotonic tick the Executive passed in at `revise`.
        tick: u64,
    },
    /// Justification withdrawn, or aged past the mandate's bound by the
    /// Executive. The fact may still be readable; it must not be treated as
    /// current.
    Stale {
        /// Last tick at which this was fresh, if any.
        last_fresh_tick: Option<u64>,
    },
    /// Never observed.
    Unknown,
}

impl Freshness {
    /// Mark fresh facts older than `bound` ticks as stale. Other variants unchanged.
    pub const fn aged(self, now: u64, bound: u64) -> Self {
        match self {
            Freshness::Fresh { tick } if now.saturating_sub(tick) >= bound => Freshness::Stale {
                last_fresh_tick: Some(tick),
            },
            other => other,
        }
    }
}

/// Why we believe a fact.
///
/// Keep this small. A source id plus a tick is enough to start. A full JTMS
/// justification set is an optional later impl, not a required one.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Justification<Source> {
    /// Who asserted this. Sensor adapter, operator, watchdog, child machine.
    pub source: Source,
    /// Executive tick at assertion.
    pub tick: u64,
}

/// One believed fact plus its justification and freshness.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Belief<Fact, Source> {
    /// The payload. Domain-defined. Core does not know about bars, poses, or
    /// cookies.
    pub fact: Fact,
    /// Why we hold it.
    pub because: Justification<Source>,
    /// Fresh or stale *as of the last Executive pulse*.
    pub freshness: Freshness,
}

impl<Fact, Source> Belief<Fact, Source> {
    /// Age this slot against a mandate bound.
    pub fn age(&mut self, now: u64, bound: u64) {
        self.freshness = self.freshness.aged(now, bound);
    }
}

/// Revisable world facts.
///
/// The store can change without a machine transition. That is the point.
/// Context on the machine is a *slice* copied in at `apply` time if the
/// machine needs it for a guard — not a mirror of this store.
pub trait BeliefStore {
    /// Key space. Prefer a small enum over strings on the hot path.
    type Key;

    /// Fact payload.
    type Fact;

    /// Who is allowed to write. Adapter id, not a socket.
    type Source;

    /// Write or overwrite a fact. Does **not** produce a `Msg`. Lift does.
    fn revise(&mut self, key: Self::Key, fact: Self::Fact, because: Justification<Self::Source>);

    /// Withdraw a justification. Implementations may drop the fact or mark it
    /// [`Freshness::Stale`]. Either way, the chart is not called from here.
    fn withdraw(&mut self, key: &Self::Key, source: &Self::Source);

    /// Borrow a belief if present (including stale).
    fn get(&self, key: &Self::Key) -> Option<&Belief<Self::Fact, Self::Source>>;

    /// Mutate a slot (aging, in-place patch). Default: none.
    fn get_mut(&mut self, _key: &Self::Key) -> Option<&mut Belief<Self::Fact, Self::Source>> {
        None
    }

    /// Freshness helper so lift does not re-derive it.
    fn freshness(&self, key: &Self::Key) -> Freshness {
        match self.get(key) {
            Some(b) => b.freshness,
            None => Freshness::Unknown,
        }
    }

    /// Optional: age facts against a mandate bound. Default is a no-op so a
    /// no_std store can ignore clocks. The Executive is expected to call this
    /// (or do the aging itself) on a pulse. Use [`Mandate::freshness_bound`].
    fn age(&mut self, _now: u64, _mandate: &impl Mandate) {}
}

/// In-memory store. `alloc` (`BTreeMap`). Homogeneous `Fact`.
///
/// rustbrain: [[docs/adr/0020-kernel-sits-beside-the-chart]]
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemoryStore<K, F, S>
where
    K: Ord,
{
    inner: BTreeMap<K, Belief<F, S>>,
}

#[cfg(feature = "alloc")]
impl<K, F, S> MemoryStore<K, F, S>
where
    K: Ord,
{
    /// Empty board.
    pub const fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }
}

#[cfg(feature = "alloc")]
impl<K: Ord, F, S> Default for MemoryStore<K, F, S> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "alloc")]
impl<K, F, S> BeliefStore for MemoryStore<K, F, S>
where
    K: Clone + Ord,
    S: PartialEq,
{
    type Key = K;
    type Fact = F;
    type Source = S;

    fn revise(&mut self, key: K, fact: F, because: Justification<S>) {
        let tick = because.tick;
        self.inner.insert(
            key,
            Belief {
                fact,
                because,
                freshness: Freshness::Fresh { tick },
            },
        );
    }

    fn withdraw(&mut self, key: &K, source: &S) {
        if let Some(b) = self.inner.get_mut(key) {
            if &b.because.source == source {
                b.freshness = Freshness::Stale {
                    last_fresh_tick: match b.freshness {
                        Freshness::Fresh { tick } => Some(tick),
                        Freshness::Stale { last_fresh_tick } => last_fresh_tick,
                        Freshness::Unknown => None,
                    },
                };
            }
        }
    }

    fn get(&self, key: &K) -> Option<&Belief<F, S>> {
        self.inner.get(key)
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut Belief<F, S>> {
        self.inner.get_mut(key)
    }

    fn age(&mut self, now: u64, mandate: &impl Mandate) {
        let Some(bound) = mandate.freshness_bound() else {
            return;
        };
        for b in self.inner.values_mut() {
            b.age(now, bound);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct OneCell {
        inner: Option<Belief<u32, &'static str>>,
    }

    impl BeliefStore for OneCell {
        type Key = ();
        type Fact = u32;
        type Source = &'static str;

        fn revise(&mut self, (): (), fact: u32, because: Justification<&'static str>) {
            self.inner = Some(Belief {
                fact,
                because,
                freshness: Freshness::Fresh { tick: because.tick },
            });
        }

        fn withdraw(&mut self, (): &(), _source: &&'static str) {
            if let Some(b) = self.inner.as_mut() {
                b.freshness = Freshness::Stale {
                    last_fresh_tick: match b.freshness {
                        Freshness::Fresh { tick } => Some(tick),
                        Freshness::Stale { last_fresh_tick } => last_fresh_tick,
                        Freshness::Unknown => None,
                    },
                };
            }
        }

        fn get(&self, (): &()) -> Option<&Belief<u32, &'static str>> {
            self.inner.as_ref()
        }

        fn get_mut(&mut self, (): &()) -> Option<&mut Belief<u32, &'static str>> {
            self.inner.as_mut()
        }
    }

    #[test]
    fn withdraw_stales_without_inventing_a_state() {
        let mut s = OneCell { inner: None };
        s.revise(
            (),
            400,
            Justification {
                source: "drop-copy",
                tick: 10,
            },
        );
        assert!(matches!(s.freshness(&()), Freshness::Fresh { tick: 10 }));
        s.withdraw(&(), &"drop-copy");
        assert!(matches!(s.freshness(&()), Freshness::Stale { .. }));
        assert_eq!(s.get(&()).unwrap().fact, 400);
    }

    #[test]
    fn freshness_ages_past_bound() {
        let f = Freshness::Fresh { tick: 1 };
        assert!(matches!(
            f.aged(5, 4),
            Freshness::Stale {
                last_fresh_tick: Some(1)
            }
        ));
        assert!(matches!(f.aged(4, 4), Freshness::Fresh { tick: 1 }));
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn memory_store_ages_from_mandate_bound() {
        struct Bound;
        impl Mandate for Bound {
            type Id = ();
            fn id(&self) {}
            fn freshness_bound(&self) -> Option<u64> {
                Some(3)
            }
        }

        let mut s = MemoryStore::<u8, i32, &'static str>::new();
        s.revise(
            1,
            42,
            Justification {
                source: "adc",
                tick: 0,
            },
        );
        s.age(2, &Bound);
        assert!(matches!(s.freshness(&1), Freshness::Fresh { tick: 0 }));
        s.age(3, &Bound);
        assert!(matches!(s.freshness(&1), Freshness::Stale { .. }));
    }
}
