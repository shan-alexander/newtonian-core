//! Hysteretic scores and per-entity latch state.
//!
//! rustbrain: [[docs/concepts/hysteretic-score]]
//! rustbrain: [[docs/adr/0019-gated-hysteretic-scores-are-not-harel-history]]
//! rustbrain: [[docs/concepts/enablement-dag]]

use crate::kernel::key::{Key, Pulse, ScoreId, SCORE_WIDTH};

/// Authored score: identity, own clock, sticky, enablement.
///
/// New *kinds* are new specs (Rust). Numbers (period, sticky) are data.
/// Enablement is score-on-score: `enable_mask` must be a subset of **earlier**
/// specs (strictly increasing [`ScoreId`]). Parent off → this score is forced
/// off this pulse (sticky discarded). No decay in this slice.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScoreSpec {
    /// Bit this score occupies. Must be `< SCORE_WIDTH`.
    pub id: ScoreId,
    /// Own-clock divisor. `0` is treated as `1` (every pulse is an own tick).
    /// A 1 m sticky on a 5 s pulse uses `period` in host units, not Harel history.
    pub period: u64,
    /// Own-clock ticks the bit stays in-play after raw becomes false.
    /// `0` means combinatorial (raw false → off this pulse).
    pub sticky: u16,
    /// Parent bits that must already be in-play. [`Key::EMPTY`] = always enabled.
    pub enable_mask: Key,
}

impl ScoreSpec {
    /// Combinatorial score, period 1, no parent.
    pub const fn new(id: ScoreId) -> Self {
        Self {
            id,
            period: 1,
            sticky: 0,
            enable_mask: Key::EMPTY,
        }
    }

    /// Own-clock divisor.
    pub const fn with_period(mut self, period: u64) -> Self {
        self.period = period;
        self
    }

    /// Extra own-ticks of hold after raw falls.
    pub const fn with_sticky(mut self, sticky: u16) -> Self {
        self.sticky = sticky;
        self
    }

    /// Enabled only when `parent` is already in-play this pulse.
    pub const fn enabled_by(mut self, parent: ScoreId) -> Self {
        self.enable_mask = Key::bit(parent);
        self
    }

    /// Enabled only when every bit of `mask` is already in-play.
    pub const fn enabled_by_mask(mut self, mask: Key) -> Self {
        self.enable_mask = mask;
        self
    }
}

/// Per-entity score latches. Not a chart snapshot.
///
/// Persist this beside beliefs if you want replay-identical sticky. Do not
/// stuff it into Newton history.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EntityState {
    in_play: Key,
    last_key: Key,
    had_hit: bool,
    sticky_left: [u16; SCORE_WIDTH as usize],
    last_period: [u64; SCORE_WIDTH as usize],
}

impl Default for EntityState {
    fn default() -> Self {
        Self::new()
    }
}

impl EntityState {
    /// Cold entity: empty pool, never hit a sleeve.
    pub const fn new() -> Self {
        Self {
            in_play: Key::EMPTY,
            last_key: Key::EMPTY,
            had_hit: false,
            sticky_left: [0; SCORE_WIDTH as usize],
            last_period: [0; SCORE_WIDTH as usize],
        }
    }

    /// In-play **score** bits after the last eval (no extra / Newton bits).
    #[inline]
    pub const fn in_play(&self) -> Key {
        self.in_play
    }

    /// Composed key (scores ∪ extra) from the last [`step_entity`](crate::step_entity).
    #[inline]
    pub const fn last_key(&self) -> Key {
        self.last_key
    }

    /// Whether the last lookup hit a ROM row.
    #[inline]
    pub const fn had_hit(&self) -> bool {
        self.had_hit
    }

    pub(crate) fn record_step(&mut self, in_play: Key, key: Key, hit: bool) {
        self.in_play = in_play;
        self.last_key = key;
        self.had_hit = hit;
    }
}

/// Specs are a DAG in list order: strictly increasing ids, enable bits already seen.
pub fn specs_valid(specs: &[ScoreSpec]) -> bool {
    let mut seen = Key::EMPTY;
    let mut prev: Option<ScoreId> = None;
    for spec in specs {
        if spec.id.bit() >= SCORE_WIDTH {
            return false;
        }
        if let Some(p) = prev {
            if spec.id <= p {
                return false;
            }
        }
        prev = Some(spec.id);
        if !seen.contains(spec.enable_mask) {
            return false;
        }
        seen.set(spec.id);
    }
    true
}

/// Rebuild the in-play score pool for one entity.
///
/// `raw` is the host classifier this pulse (beliefs, not the ROM). Extra /
/// Newton bits are **not** applied here.
pub fn eval_scores(specs: &[ScoreSpec], state: &mut EntityState, raw: Key, pulse: Pulse) -> Key {
    debug_assert!(
        specs_valid(specs),
        "ScoreSpec slice must be a DAG: increasing ScoreId, enable_mask ⊆ earlier ids"
    );

    let mut in_play = Key::EMPTY;
    for spec in specs {
        let i = spec.id.index();
        let period = spec.period.max(1);
        let own = pulse.tick / period;
        let delta = own.saturating_sub(state.last_period[i]);
        state.last_period[i] = own;

        let enabled = spec.enable_mask.is_empty() || in_play.contains(spec.enable_mask);
        if !enabled {
            state.sticky_left[i] = 0;
            continue;
        }

        if raw.has(spec.id) {
            state.sticky_left[i] = spec.sticky;
            in_play.set(spec.id);
        } else if state.sticky_left[i] > 0 {
            in_play.set(spec.id);
            if delta > 0 {
                let drop = core::cmp::min(delta, u64::from(state.sticky_left[i])) as u16;
                state.sticky_left[i] -= drop;
            }
        }
    }
    in_play
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: ScoreId = ScoreId::new(0);
    const B: ScoreId = ScoreId::new(1);

    #[test]
    fn parent_off_forces_child_off_and_drops_sticky() {
        let specs = [
            ScoreSpec::new(A),
            ScoreSpec::new(B).enabled_by(A).with_sticky(4),
        ];
        let mut state = EntityState::new();
        let both = Key::from_ids([A, B]);
        let in_play = eval_scores(&specs, &mut state, both, Pulse::new(0));
        assert_eq!(in_play, both);

        let only_b = Key::bit(B);
        let in_play = eval_scores(&specs, &mut state, only_b, Pulse::new(1));
        assert!(in_play.is_empty(), "A off → B forced off, sticky discarded");
    }

    #[test]
    fn specs_reject_parent_after_child() {
        let bad = [ScoreSpec::new(B).enabled_by(A), ScoreSpec::new(A)];
        assert!(!specs_valid(&bad));
    }
}
