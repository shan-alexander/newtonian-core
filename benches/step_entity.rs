use criterion::{black_box, criterion_group, criterion_main, Criterion};
use newtonian_core::prelude::*;

const COMBAT: ScoreId = ScoreId::new(0);
const LOW_HP: ScoreId = ScoreId::new(1);
const BOSS: ScoreId = ScoreId::new(2);

fn bench_step_entity(c: &mut Criterion) {
    let specs = [
        ScoreSpec::new(COMBAT),
        ScoreSpec::new(LOW_HP).enabled_by(COMBAT).with_sticky(2),
        ScoreSpec::new(BOSS).enabled_by(COMBAT),
    ];
    let rows = [
        (Key::bit(COMBAT), 1u8),
        (Key::from_ids([COMBAT, LOW_HP]), 2u8),
        (Key::from_ids([COMBAT, BOSS]), 3u8),
    ];
    let table = Table::from_sorted(&rows);
    let raw = Key::from_ids([COMBAT, BOSS]);

    c.bench_function("step_entity_hit", |b| {
        let mut state = EntityState::new();
        b.iter(|| {
            let step = step_entity(
                black_box(&specs),
                &mut state,
                black_box(raw),
                Key::EMPTY,
                Pulse::new(1),
                black_box(&table),
            );
            black_box(step.sleeve)
        });
    });
}

criterion_group!(benches, bench_step_entity);
criterion_main!(benches);
