//! Fold: YAML/TOML → interned ROM **once**.
//!
//! Score *kinds*, extra bits, and play *kinds* are Rust ([`Catalog`]). The
//! document may change sticky/period, `enabled_by`, which exact chords exist,
//! and numeric knobs. Unknown names fail at Fold, not at `step`. YAML uses
//! `serde_yaml` 0.9 (MSRV 1.80); TOML is `toml` 0.8.
//!
//! rustbrain: [[docs/adr/0018-fold-yaml-once-never-interpret-topology]]
//! rustbrain: [[docs/adr/0012-yaml-is-knobs-not-topology]]
//! symbol:Catalog symbol:Policy symbol:Sleeve

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

use serde::Deserialize;

use crate::{
    specs_valid, step_entity, EntityState, Folded, KernelStep, Key, Pulse, ScoreId, ScoreSpec,
    Table, SCORE_WIDTH,
};

/// Maximum interned numeric knobs on a sleeve. Fold-time names; ids on the hot path.
pub const KNOB_SLOTS: usize = 8;

/// Interned knob slot. Assigned from [`Catalog::knob`] order.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct KnobId(u8);

impl KnobId {
    /// Slot index `0..KNOB_SLOTS`.
    #[inline]
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

/// Copy numeric knobs. No strings.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Knobs {
    values: [Option<f64>; KNOB_SLOTS],
}

impl Knobs {
    /// All empty.
    pub const fn empty() -> Self {
        Self {
            values: [None; KNOB_SLOTS],
        }
    }

    /// Value interned as `id`, if the document set it.
    #[inline]
    pub const fn get(self, id: KnobId) -> Option<f64> {
        self.values[id.0 as usize]
    }
}

/// Interned row payload: play kind + knobs. Not I/O.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sleeve<P> {
    /// Behavior kind from the catalog (Rust).
    pub play: P,
    /// Numeric parameters from the document.
    pub knobs: Knobs,
    /// Authored row that emits no firepower. Absence of a row is still miss.
    pub observe_only: bool,
}

/// Why Fold refused a document. Never returned from [`crate::step_entity`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FoldError {
    /// YAML/TOML did not deserialize.
    Parse(String),
    /// Document named a score/extra/play/knob not in the catalog (new *kind* = rustc).
    UnknownName(String),
    /// Same name registered twice in the catalog, or as both score and extra.
    DuplicateName(String),
    /// Two sleeves interned to the same exact key.
    DuplicateKey,
    /// `enabled_by` parent is not a catalog *score* (extras cannot enable).
    EnablementNotScore(String),
    /// Specs after Fold violate the DAG (parent id must be lower).
    EnablementOrder,
    /// Extra bit must be `SCORE_WIDTH..128`.
    ExtraBitRange {
        /// Catalog name.
        name: String,
        /// Illegal bit.
        bit: u32,
    },
    /// Score id must be `< SCORE_WIDTH`.
    ScoreIdRange(String),
    /// More knobs than [`KNOB_SLOTS`].
    TooManyKnobs,
}

impl fmt::Display for FoldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FoldError::Parse(s) => write!(f, "fold parse: {s}"),
            FoldError::UnknownName(n) => {
                write!(f, "fold: {n:?} is not in the catalog (new kind = rustc)")
            }
            FoldError::DuplicateName(n) => write!(f, "fold: duplicate catalog name {n:?}"),
            FoldError::DuplicateKey => {
                write!(f, "fold: two sleeves interned to the same exact key")
            }
            FoldError::EnablementNotScore(n) => {
                write!(f, "fold: enabled_by {n:?} is not a score")
            }
            FoldError::EnablementOrder => {
                write!(
                    f,
                    "fold: enabled_by parent must have a lower ScoreId than the child"
                )
            }
            FoldError::ExtraBitRange { name, bit } => {
                write!(
                    f,
                    "fold: extra {name:?} bit {bit} must be {SCORE_WIDTH}..128"
                )
            }
            FoldError::ScoreIdRange(n) => write!(f, "fold: score {n:?} id must be < {SCORE_WIDTH}"),
            FoldError::TooManyKnobs => write!(f, "fold: more than {KNOB_SLOTS} knobs"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for FoldError {}

/// Rust name universe. Fold resolves document strings against this once.
#[derive(Clone, Debug)]
pub struct Catalog<P> {
    scores: Vec<(&'static str, ScoreId)>,
    extras: Vec<(&'static str, u32)>,
    plays: Vec<(&'static str, P)>,
    knobs: Vec<&'static str>,
}

impl<P> Default for Catalog<P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P> Catalog<P> {
    /// Empty catalog.
    pub const fn new() -> Self {
        Self {
            scores: Vec::new(),
            extras: Vec::new(),
            plays: Vec::new(),
            knobs: Vec::new(),
        }
    }

    /// Register a score *kind*. Duplicate names fail at Fold.
    pub fn score(mut self, name: &'static str, id: ScoreId) -> Self {
        self.scores.push((name, id));
        self
    }

    /// Register a host extra bit (Newton `project()`, a mode, …). `bit >= SCORE_WIDTH`.
    pub fn extra(mut self, name: &'static str, bit: u32) -> Self {
        self.extras.push((name, bit));
        self
    }

    /// Register a play *kind*.
    pub fn play(mut self, name: &'static str, play: P) -> Self {
        self.plays.push((name, play));
        self
    }

    /// Register a knob name. Order is [`KnobId`] order.
    pub fn knob(mut self, name: &'static str) -> Self {
        self.knobs.push(name);
        self
    }

    /// Interned id for a catalog knob name.
    pub fn knob_id(&self, name: &str) -> Option<KnobId> {
        self.knobs
            .iter()
            .position(|n| *n == name)
            .map(|i| KnobId(i as u8))
    }
}

impl<P: Clone> Catalog<P> {
    /// Compile TOML once. The string is dropped; `step` never sees it.
    pub fn fold_toml(&self, src: &str) -> Result<Policy<P>, FoldError> {
        let doc: RawDoc =
            toml::from_str(src).map_err(|e| FoldError::Parse(alloc::format!("{e}")))?;
        self.fold_raw(doc)
    }

    /// Compile YAML once. The string is dropped; `step` never sees it.
    pub fn fold_yaml(&self, src: &str) -> Result<Policy<P>, FoldError> {
        let doc: RawDoc =
            serde_yaml::from_str(src).map_err(|e| FoldError::Parse(alloc::format!("{e}")))?;
        self.fold_raw(doc)
    }

    fn fold_raw(&self, doc: RawDoc) -> Result<Policy<P>, FoldError> {
        self.check_catalog()?;
        if self.knobs.len() > KNOB_SLOTS {
            return Err(FoldError::TooManyKnobs);
        }

        let mut specs: Vec<ScoreSpec> = self
            .scores
            .iter()
            .map(|(_, id)| ScoreSpec::new(*id))
            .collect();

        for (name, raw) in &doc.scores {
            let id = self
                .score_id(name)
                .ok_or_else(|| FoldError::UnknownName(name.clone()))?;
            let spec = specs
                .iter_mut()
                .find(|s| s.id == id)
                .expect("catalog score");
            spec.period = raw.period.max(1);
            spec.sticky = raw.sticky;
            spec.max_age = raw.max_age;
            spec.enable_mask = Key::EMPTY;
            spec.disarm_mask = Key::EMPTY;
            for parent in &raw.enabled_by {
                let pid = self
                    .score_id(parent)
                    .ok_or_else(|| FoldError::EnablementNotScore(parent.clone()))?;
                spec.enable_mask = spec.enable_mask.union(Key::bit(pid));
            }
            for peer in &raw.disarm_when {
                let pid = self
                    .score_id(peer)
                    .ok_or_else(|| FoldError::EnablementNotScore(peer.clone()))?;
                spec.disarm_mask = spec.disarm_mask.union(Key::bit(pid));
            }
        }

        specs.sort_by_key(|s| s.id);
        if !specs_valid(&specs) {
            return Err(FoldError::EnablementOrder);
        }

        let mut rows = Vec::with_capacity(doc.sleeves.len());
        for sleeve in &doc.sleeves {
            let mut key = Key::EMPTY;
            for n in &sleeve.when_exactly {
                key = key.union(self.resolve_bit(n)?);
            }
            let play = self
                .play_of(&sleeve.play)
                .ok_or_else(|| FoldError::UnknownName(sleeve.play.clone()))?;
            let mut knobs = Knobs::empty();
            for (kname, val) in &sleeve.knobs {
                let id = self
                    .knob_id(kname)
                    .ok_or_else(|| FoldError::UnknownName(kname.clone()))?;
                knobs.values[id.index()] = Some(*val);
            }
            rows.push((
                key,
                Sleeve {
                    play,
                    knobs,
                    observe_only: sleeve.observe_only,
                },
            ));
        }

        rows.sort_by_key(|r| r.0);
        if rows.windows(2).any(|w| w[0].0 == w[1].0) {
            return Err(FoldError::DuplicateKey);
        }

        Ok(Policy { specs, rows })
    }

    fn check_catalog(&self) -> Result<(), FoldError> {
        let mut seen = BTreeMap::new();
        for (n, id) in &self.scores {
            if id.bit() >= SCORE_WIDTH {
                return Err(FoldError::ScoreIdRange((*n).into()));
            }
            if seen.insert(*n, "score").is_some() {
                return Err(FoldError::DuplicateName((*n).into()));
            }
        }
        for (n, bit) in &self.extras {
            if *bit < SCORE_WIDTH || *bit >= 128 {
                return Err(FoldError::ExtraBitRange {
                    name: (*n).into(),
                    bit: *bit,
                });
            }
            if seen.insert(*n, "extra").is_some() {
                return Err(FoldError::DuplicateName((*n).into()));
            }
        }
        let mut plays = BTreeMap::new();
        for (n, _) in &self.plays {
            if !plays.insert(*n, ()).is_none() {
                return Err(FoldError::DuplicateName((*n).into()));
            }
        }
        let mut knobs = BTreeMap::new();
        for n in &self.knobs {
            if !knobs.insert(*n, ()).is_none() {
                return Err(FoldError::DuplicateName((*n).into()));
            }
        }
        Ok(())
    }

    fn score_id(&self, name: &str) -> Option<ScoreId> {
        self.scores
            .iter()
            .find(|(n, _)| *n == name)
            .map(|(_, id)| *id)
    }

    fn play_of(&self, name: &str) -> Option<P> {
        self.plays
            .iter()
            .find(|(n, _)| *n == name)
            .map(|(_, p)| p.clone())
    }

    fn resolve_bit(&self, name: &str) -> Result<Key, FoldError> {
        if let Some(id) = self.score_id(name) {
            return Ok(Key::bit(id));
        }
        if let Some((_, bit)) = self.extras.iter().find(|(n, _)| *n == name) {
            return Ok(Key::from_bit(*bit));
        }
        Err(FoldError::UnknownName(name.into()))
    }
}

/// Owned interned policy. Flyweight: all entities share one. No document.
#[derive(Clone, Debug, PartialEq)]
pub struct Policy<P> {
    specs: Vec<ScoreSpec>,
    rows: Vec<(Key, Sleeve<P>)>,
}

impl<P> Policy<P> {
    /// Score specs (DAG, clocks, sticky).
    pub fn specs(&self) -> &[ScoreSpec] {
        &self.specs
    }

    /// Exact ROM.
    pub fn table(&self) -> Table<'_, Sleeve<P>> {
        Table::from_sorted(&self.rows)
    }

    /// Borrow as the kernel [`Folded`] view.
    pub fn folded(&self) -> Folded<'_, Sleeve<P>> {
        Folded::new(&self.specs, self.table())
    }

    /// One entity pulse. Does not parse YAML. Does not call `apply`.
    pub fn step(
        &self,
        state: &mut EntityState,
        raw: Key,
        extra: Key,
        pulse: Pulse,
    ) -> KernelStep<'_, Sleeve<P>> {
        step_entity(&self.specs, state, raw, extra, pulse, &self.table())
    }
}

#[derive(Debug, Deserialize)]
struct RawDoc {
    #[serde(default)]
    scores: BTreeMap<String, RawScore>,
    #[serde(default)]
    sleeves: Vec<RawSleeve>,
}

#[derive(Debug, Deserialize)]
struct RawScore {
    #[serde(default = "default_period")]
    period: u64,
    #[serde(default)]
    sticky: u16,
    #[serde(default)]
    max_age: u16,
    #[serde(default)]
    enabled_by: Vec<String>,
    #[serde(default)]
    disarm_when: Vec<String>,
}

fn default_period() -> u64 {
    1
}

#[derive(Debug, Deserialize)]
struct RawSleeve {
    when_exactly: Vec<String>,
    play: String,
    #[serde(default)]
    knobs: BTreeMap<String, f64>,
    #[serde(default)]
    observe_only: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum Play {
        Scan,
        Fire,
    }

    fn cat() -> Catalog<Play> {
        Catalog::new()
            .score("HighConf", ScoreId::new(0))
            .score("OnTarget", ScoreId::new(1))
            .extra("Engage", SCORE_WIDTH)
            .play("Scan", Play::Scan)
            .play("Fire", Play::Fire)
            .knob("power")
    }

    const TOML: &str = r#"
[scores.HighConf]
sticky = 4

[scores.OnTarget]
sticky = 2
enabled_by = ["HighConf"]

[[sleeves]]
when_exactly = ["Engage"]
play = "Scan"

[[sleeves]]
when_exactly = ["HighConf", "OnTarget", "Engage"]
play = "Fire"
knobs = { power = 0.8 }
"#;

    const YAML: &str = r#"
scores:
  HighConf:
    sticky: 4
  OnTarget:
    sticky: 2
    enabled_by: [HighConf]
sleeves:
  - when_exactly: [Engage]
    play: Scan
  - when_exactly: [HighConf, OnTarget, Engage]
    play: Fire
    knobs:
      power: 0.8
"#;

    #[test]
    fn toml_and_yaml_intern_the_same_rom() {
        let c = cat();
        let a = c.fold_toml(TOML).expect("toml");
        let b = c.fold_yaml(YAML).expect("yaml");
        assert_eq!(a.specs(), b.specs());
        assert_eq!(a.rows.len(), 2);
        assert_eq!(a, b);
        let power = c.knob_id("power").unwrap();
        let fire = a
            .table()
            .lookup(
                Key::from_ids([ScoreId::new(0), ScoreId::new(1)]).union(Key::from_bit(SCORE_WIDTH)),
            )
            .unwrap();
        assert_eq!(fire.play, Play::Fire);
        assert_eq!(fire.knobs.get(power), Some(0.8));
        assert!(!fire.observe_only);
    }

    #[test]
    fn unknown_score_kind_is_a_fold_error() {
        let err = cat()
            .fold_toml(
                r#"
[[sleeves]]
when_exactly = ["ParryWindow"]
play = "Fire"
"#,
            )
            .unwrap_err();
        assert!(matches!(err, FoldError::UnknownName(_)));
    }

    #[test]
    fn duplicate_exact_key_fails() {
        let err = cat()
            .fold_toml(
                r#"
[[sleeves]]
when_exactly = ["Engage"]
play = "Scan"
[[sleeves]]
when_exactly = ["Engage"]
play = "Fire"
"#,
            )
            .unwrap_err();
        assert_eq!(err, FoldError::DuplicateKey);
    }

    #[test]
    fn unauthored_product_is_none_after_fold() {
        let p = cat().fold_toml(TOML).unwrap();
        let mut state = EntityState::new();
        let step = p.step(
            &mut state,
            Key::bit(ScoreId::new(0)),
            Key::from_bit(SCORE_WIDTH),
            Pulse::new(0),
        );
        assert!(step.sleeve.is_none());
    }

    #[test]
    fn max_age_and_disarm_when_fold() {
        let p = cat()
            .fold_toml(
                r#"
[scores.HighConf]
max_age = 8

[scores.OnTarget]
disarm_when = ["HighConf"]
"#,
            )
            .unwrap();
        let specs = p.specs();
        assert_eq!(specs[0].max_age, 8);
        assert_eq!(specs[1].disarm_mask, Key::bit(ScoreId::new(0)));
    }

    #[test]
    fn enabled_by_extra_is_rejected() {
        let err = cat()
            .fold_toml(
                r#"
[scores.OnTarget]
enabled_by = ["Engage"]
"#,
            )
            .unwrap_err();
        assert!(matches!(err, FoldError::EnablementNotScore(_)));
    }
}
