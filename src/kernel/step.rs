//! `step_entity`: gated scores → exact key → ROM. No I/O.
//!
//! rustbrain: [[docs/adr/0020-kernel-sits-beside-the-chart]]
//! rustbrain: [[docs/concepts/synchronous-pulse]]

use crate::kernel::key::{Key, Pulse};
use crate::kernel::score::{eval_scores, specs_valid, EntityState, ScoreSpec};
use crate::kernel::table::Table;

/// What changed in the ROM hit between this pulse and the last.
#[must_use]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Transition {
    /// Same miss, or same authored key still selected.
    Hold,
    /// First hit after a miss (or cold start).
    Enter,
    /// Authored key changed; previous row should be treated as exit.
    Replace,
    /// Had a row; this key is unauthored (none).
    Leave,
}

/// Result of one [`step_entity`]. Sleeve payload is borrowed from the interned table.
#[must_use]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KernelStep<'a, T> {
    /// Composed key this pulse (in-play scores ∪ extra).
    pub key: Key,
    /// Exact ROM hit. [`None`] is unauthored — not a parent sleeve.
    pub sleeve: Option<&'a T>,
    /// Enter / hold / replace / leave relative to this entity's last pulse.
    pub transition: Transition,
}

/// One Mealy pulse for one entity.
///
/// - `raw` — host classifiers this tick (from beliefs). Not Newton `Msg`.
/// - `extra` — disjoint bits to join the lookup key (Newton `project()`, …).
/// - Does **not** call `apply`. Does **not** admit. Does **not** parse YAML.
pub fn step_entity<'a, T>(
    specs: &[ScoreSpec],
    state: &mut EntityState,
    raw: Key,
    extra: Key,
    pulse: Pulse,
    table: &Table<'a, T>,
) -> KernelStep<'a, T> {
    let in_play = eval_scores(specs, state, raw, pulse);
    let key = in_play.union(extra);
    let sleeve = table.lookup(key);
    let hit = sleeve.is_some();
    let transition = match (state.had_hit(), hit) {
        (false, false) => Transition::Hold,
        (false, true) => Transition::Enter,
        (true, false) => Transition::Leave,
        (true, true) if state.last_key() == key => Transition::Hold,
        (true, true) => Transition::Replace,
    };
    state.record_step(in_play, key, hit);
    KernelStep {
        key,
        sleeve,
        transition,
    }
}

/// Output of Fold: score specs + interned table.
///
/// Feature `fold` builds an owned intern that borrows as this view.
/// `step` never sees a document.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Folded<'a, T> {
    /// Enablement DAG + clocks + sticky (Rust-authored kinds).
    pub specs: &'a [ScoreSpec],
    /// Exact ROM.
    pub table: Table<'a, T>,
}

impl<'a, T> Folded<'a, T> {
    /// Intern once. Same object live / replay / test.
    pub fn new(specs: &'a [ScoreSpec], table: Table<'a, T>) -> Self {
        debug_assert!(specs_valid(specs));
        Self { specs, table }
    }

    /// One entity pulse. Does not call [`crate::IntentionMachine::apply`].
    #[inline]
    pub fn step(
        &self,
        state: &mut EntityState,
        raw: Key,
        extra: Key,
        pulse: Pulse,
    ) -> KernelStep<'a, T> {
        step_entity(self.specs, state, raw, extra, pulse, &self.table)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::key::ScoreId;

    const COMBAT: ScoreId = ScoreId::new(0);
    const LOW_HP: ScoreId = ScoreId::new(1);
    const BOSS: ScoreId = ScoreId::new(2);

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum Play {
        Guard,
        Retreat,
        Dive,
    }

    #[test]
    fn exact_unauthored_triple_is_none_not_subset() {
        let specs = [
            ScoreSpec::new(COMBAT),
            ScoreSpec::new(LOW_HP).enabled_by(COMBAT),
            ScoreSpec::new(BOSS).enabled_by(COMBAT),
        ];
        let rows = [
            (Key::bit(COMBAT), Play::Guard),
            (Key::from_ids([COMBAT, LOW_HP]), Play::Retreat),
            (Key::from_ids([COMBAT, BOSS]), Play::Dive),
        ];
        let table = Table::from_sorted(&rows);
        let folded = Folded::new(&specs, table);
        let mut state = EntityState::new();

        let step = folded.step(
            &mut state,
            Key::from_ids([COMBAT, LOW_HP, BOSS]),
            Key::EMPTY,
            Pulse::new(0),
        );
        assert_eq!(step.sleeve, None);
        assert_eq!(step.transition, Transition::Hold);
        assert_eq!(step.key, Key::from_ids([COMBAT, LOW_HP, BOSS]));
    }

    #[test]
    fn enter_hold_leave() {
        let specs = [ScoreSpec::new(COMBAT)];
        let rows = [(Key::bit(COMBAT), Play::Guard)];
        let table = Table::from_sorted(&rows);
        let mut state = EntityState::new();

        let a = step_entity(
            &specs,
            &mut state,
            Key::bit(COMBAT),
            Key::EMPTY,
            Pulse::new(0),
            &table,
        );
        assert_eq!(a.transition, Transition::Enter);
        assert_eq!(a.sleeve, Some(&Play::Guard));

        let b = step_entity(
            &specs,
            &mut state,
            Key::bit(COMBAT),
            Key::EMPTY,
            Pulse::new(1),
            &table,
        );
        assert_eq!(b.transition, Transition::Hold);

        let c = step_entity(
            &specs,
            &mut state,
            Key::EMPTY,
            Key::EMPTY,
            Pulse::new(2),
            &table,
        );
        assert_eq!(c.transition, Transition::Leave);
        assert_eq!(c.sleeve, None);
    }

    #[test]
    fn sticky_holds_then_expires() {
        let specs = [ScoreSpec::new(COMBAT).with_sticky(2)];
        let rows = [(Key::bit(COMBAT), Play::Guard)];
        let table = Table::from_sorted(&rows);
        let mut state = EntityState::new();

        let _ = step_entity(
            &specs,
            &mut state,
            Key::bit(COMBAT),
            Key::EMPTY,
            Pulse::new(0),
            &table,
        );
        let hold1 = step_entity(
            &specs,
            &mut state,
            Key::EMPTY,
            Key::EMPTY,
            Pulse::new(1),
            &table,
        );
        assert_eq!(hold1.sleeve, Some(&Play::Guard));
        let hold2 = step_entity(
            &specs,
            &mut state,
            Key::EMPTY,
            Key::EMPTY,
            Pulse::new(2),
            &table,
        );
        assert_eq!(hold2.sleeve, Some(&Play::Guard));
        let gone = step_entity(
            &specs,
            &mut state,
            Key::EMPTY,
            Key::EMPTY,
            Pulse::new(3),
            &table,
        );
        assert_eq!(gone.sleeve, None);
        assert_eq!(gone.transition, Transition::Leave);
    }

    #[test]
    fn extra_bits_join_the_key_not_enablement() {
        let specs = [ScoreSpec::new(COMBAT)];
        let airborne = Key::from_bit(crate::kernel::key::SCORE_WIDTH);
        let rows = [(Key::bit(COMBAT).union(airborne), Play::Dive)];
        let table = Table::from_sorted(&rows);
        let mut state = EntityState::new();

        let miss = step_entity(
            &specs,
            &mut state,
            Key::bit(COMBAT),
            Key::EMPTY,
            Pulse::new(0),
            &table,
        );
        assert_eq!(miss.sleeve, None);

        let hit = step_entity(
            &specs,
            &mut state,
            Key::bit(COMBAT),
            airborne,
            Pulse::new(1),
            &table,
        );
        assert_eq!(hit.sleeve, Some(&Play::Dive));
        assert_eq!(hit.transition, Transition::Enter);
    }

    #[test]
    fn slower_own_clock_does_not_age_every_pulse() {
        let specs = [ScoreSpec::new(COMBAT).with_period(10).with_sticky(1)];
        let rows = [(Key::bit(COMBAT), Play::Guard)];
        let table = Table::from_sorted(&rows);
        let mut state = EntityState::new();
        let _ = step_entity(
            &specs,
            &mut state,
            Key::bit(COMBAT),
            Key::EMPTY,
            Pulse::new(0),
            &table,
        );
        for t in 1..10 {
            let s = step_entity(
                &specs,
                &mut state,
                Key::EMPTY,
                Key::EMPTY,
                Pulse::new(t),
                &table,
            );
            assert_eq!(s.sleeve, Some(&Play::Guard), "tick {t} is same own period");
        }
        let aged = step_entity(
            &specs,
            &mut state,
            Key::EMPTY,
            Key::EMPTY,
            Pulse::new(10),
            &table,
        );
        assert_eq!(
            aged.sleeve,
            Some(&Play::Guard),
            "first own-tick of false still sticky"
        );
        let gone = step_entity(
            &specs,
            &mut state,
            Key::EMPTY,
            Key::EMPTY,
            Pulse::new(20),
            &table,
        );
        assert_eq!(gone.sleeve, None);
    }
}
