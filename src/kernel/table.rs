//! Exact sleeve ROM. Unauthored key → none.
//!
//! rustbrain: [[docs/concepts/sleeve-rom]]
//! rustbrain: [[docs/edge_cases/unauthored-key-is-none]]
//! rustbrain: [[docs/adr/0017-exact-superstate-key-not-longest-subset]]

use crate::kernel::key::Key;

/// Interned exact table. All entities share one. Flyweight.
///
/// `T` is the row payload (behavior id + knobs). Not I/O. Not a chart node.
///
/// Rows must be **sorted by [`Key`] and unique**. Lookup is binary search
/// on equality. There is no longest-subset walk.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Table<'a, T> {
    rows: &'a [(Key, T)],
}

impl<'a, T> Table<'a, T> {
    /// Borrow interned rows. Debug-asserts sorted unique keys.
    pub fn from_sorted(rows: &'a [(Key, T)]) -> Self {
        debug_assert!(
            is_sorted_unique(rows),
            "Table rows must be sorted by Key and unique (handwritten intern / Fold output)"
        );
        Self { rows }
    }

    /// Exact lookup. Miss is [`None`], not a parent sleeve.
    #[inline]
    pub fn lookup(&self, key: Key) -> Option<&'a T> {
        match self.rows.binary_search_by(|probe| probe.0.cmp(&key)) {
            Ok(i) => Some(&self.rows[i].1),
            Err(_) => None,
        }
    }

    /// Authored rows.
    #[inline]
    pub const fn rows(&self) -> &'a [(Key, T)] {
        self.rows
    }
}

fn is_sorted_unique<T>(rows: &[(Key, T)]) -> bool {
    rows.windows(2).all(|w| w[0].0 < w[1].0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::key::ScoreId;

    #[test]
    fn unauthored_key_is_none() {
        let a = ScoreId::new(0);
        let b = ScoreId::new(1);
        let c = ScoreId::new(2);
        let rows = [(Key::from_ids([a, b]), "ab")];
        let table = Table::from_sorted(&rows);
        assert_eq!(table.lookup(Key::from_ids([a, b])), Some(&"ab"));
        assert_eq!(
            table.lookup(Key::from_ids([a, b, c])),
            None,
            "exact: extra bit is a miss, not longest-subset"
        );
        assert_eq!(table.lookup(Key::bit(a)), None);
    }
}
