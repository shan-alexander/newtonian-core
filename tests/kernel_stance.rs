//! HOST example: stance scores + exact sleeve ROM. Not a desk, not a chart.
//!
//! Combat / LowHp / BossNear. Enablement is score-on-score. Unauthored
//! product is none. Extra bits (a host XOR) join the lookup key.

use newtonian_core::prelude::*;

const COMBAT: ScoreId = ScoreId::new(0);
const LOW_HP: ScoreId = ScoreId::new(1);
const BOSS: ScoreId = ScoreId::new(2);
/// Host XOR "airborne" mapped into the extra range — not a score.
const AIRBORNE: u32 = SCORE_WIDTH;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Play {
    Guard,
    Retreat,
    Dive,
}

#[test]
fn authored_rows_fire_exact_only() {
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

    let guard = folded.step(&mut state, Key::bit(COMBAT), Key::EMPTY, Pulse::new(0));
    assert_eq!(guard.sleeve, Some(&Play::Guard));
    assert_eq!(guard.transition, Transition::Enter);

    let dive = folded.step(
        &mut state,
        Key::from_ids([COMBAT, BOSS]),
        Key::EMPTY,
        Pulse::new(1),
    );
    assert_eq!(dive.sleeve, Some(&Play::Dive));
    assert_eq!(dive.transition, Transition::Replace);

    let miss = folded.step(
        &mut state,
        Key::from_ids([COMBAT, LOW_HP, BOSS]),
        Key::EMPTY,
        Pulse::new(2),
    );
    assert_eq!(miss.sleeve, None);
    assert_eq!(miss.transition, Transition::Leave);
}

#[test]
fn extra_truth_bit_selects_a_disjoint_row() {
    let specs = [ScoreSpec::new(COMBAT)];
    let air = Key::from_bit(AIRBORNE);
    let rows = [
        (Key::bit(COMBAT), Play::Guard),
        (Key::bit(COMBAT).union(air), Play::Dive),
    ];
    let table = Table::from_sorted(&rows);
    let mut state = EntityState::new();

    let grounded = step_entity(
        &specs,
        &mut state,
        Key::bit(COMBAT),
        Key::EMPTY,
        Pulse::new(0),
        &table,
    );
    assert_eq!(grounded.sleeve, Some(&Play::Guard));

    let air_hit = step_entity(
        &specs,
        &mut state,
        Key::bit(COMBAT),
        air,
        Pulse::new(1),
        &table,
    );
    assert_eq!(air_hit.sleeve, Some(&Play::Dive));
    assert_eq!(air_hit.transition, Transition::Replace);
}
