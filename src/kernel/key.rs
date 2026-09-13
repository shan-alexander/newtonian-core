//! Compact superstate key. Not a Harel node, not a string id.
//!
//! rustbrain: [[docs/adr/0017-exact-superstate-key-not-longest-subset]]
//! rustbrain: [[docs/adr/0020-kernel-sits-beside-the-chart]]

/// Maximum number of **scores** an [`EntityState`](crate::EntityState) tracks.
///
/// [`Key`] is `u128`. Bits `0..SCORE_WIDTH` are the score pool. Bits
/// `SCORE_WIDTH..128` are reserved for host **extra** (Newton `project()`,
/// a session XOR, …). Hosts must keep the two ranges disjoint.
pub const SCORE_WIDTH: u32 = 64;

/// Identity of one hysteretic score: bit index in `0..SCORE_WIDTH`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ScoreId(u8);

impl ScoreId {
    /// Bit `0..SCORE_WIDTH`.
    pub const fn new(bit: u8) -> Self {
        Self(bit)
    }

    /// Index into per-score arrays.
    #[inline]
    pub const fn index(self) -> usize {
        self.0 as usize
    }

    /// Bit position as `u32`.
    #[inline]
    pub const fn bit(self) -> u32 {
        self.0 as u32
    }
}

/// Exact superstate key (`u128` bitmask).
///
/// Equality **is** the matcher. Do not walk subsets. Unauthored key → miss.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Key(u128);

impl Key {
    /// No bits.
    pub const EMPTY: Self = Self(0);

    /// Wrap a raw mask. Prefer [`Key::from_ids`] / [`Key::bit`] in handwritten tables.
    #[inline]
    pub const fn from_raw(raw: u128) -> Self {
        Self(raw)
    }

    /// Underlying mask. Feed this to a host map if you already have one.
    #[inline]
    pub const fn raw(self) -> u128 {
        self.0
    }

    /// Single score bit. `id` should be `< SCORE_WIDTH` for pool bits;
    /// higher `ScoreId` values still occupy their bit (host extra).
    #[inline]
    pub const fn bit(id: ScoreId) -> Self {
        Self(1u128 << id.0)
    }

    /// Single bit `1 << index`. Debug-asserts `index < 128`.
    #[inline]
    pub fn from_bit(index: u32) -> Self {
        debug_assert!(
            index < 128,
            "Key::from_bit index {index} >= 128 (mapping bug, not a kernel miss)"
        );
        Self(1u128 << index)
    }

    /// Union of score ids.
    pub fn from_ids(ids: impl IntoIterator<Item = ScoreId>) -> Self {
        let mut k = Self::EMPTY;
        for id in ids {
            k.set(id);
        }
        k
    }

    /// Set a score bit.
    #[inline]
    pub fn set(&mut self, id: ScoreId) {
        *self = self.union(Self::bit(id));
    }

    /// True when every bit of `mask` is set (`mask ⊆ self`). Enablement uses this.
    /// This is **not** the ROM matcher.
    #[inline]
    pub const fn contains(self, mask: Self) -> bool {
        self.0 & mask.0 == mask.0
    }

    /// True when score `id` is set.
    #[inline]
    pub const fn has(self, id: ScoreId) -> bool {
        self.contains(Self::bit(id))
    }

    /// Bitwise or.
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Bitwise and.
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    /// At least one shared bit. Disarm uses this (not the ROM matcher).
    #[inline]
    pub const fn intersects(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }

    /// Bits in `self` and not in `other`.
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    /// Hamming weight.
    #[inline]
    pub const fn count(self) -> u32 {
        self.0.count_ones()
    }

    /// No bits set.
    #[inline]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
}

impl core::ops::BitOr for Key {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        self.union(rhs)
    }
}

impl core::ops::BitAnd for Key {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        self.intersection(rhs)
    }
}

/// One external Mealy tick. Host names it (frame, sample, bar close).
///
/// Scores may tick a **slower** clock via [`ScoreSpec::period`](crate::ScoreSpec::period).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pulse {
    /// Monotonic host tick. Units are the host's.
    pub tick: u64,
}

impl Pulse {
    /// Construct a pulse.
    #[inline]
    pub const fn new(tick: u64) -> Self {
        Self { tick }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extra_bits_do_not_collide_with_score_width() {
        let score = Key::bit(ScoreId::new(0));
        let extra = Key::from_bit(SCORE_WIDTH);
        assert!(score.intersection(extra).is_empty());
        assert_eq!(score.union(extra).count(), 2);
    }
}
