//! # AAPL 1m — a Newtonian *program* (chart + policy kernel)
//!
//! Run: `cargo run --example aapl_1m --release --features machine`
//!
//! GitHub-only (not crates.io). Domain types live **here**, not in
//! `newtonian-core` ([[docs/adr/0010-not-a-domain-crate]]).
//!
//! April 2026 1-minute OHLCV: `examples/aapl_1m_2026-04.csv`. Indicators are
//! **host** incremental `Ema` / `Stoch` (const packs, no TA crate). The chart
//! never sees a lookback. The kernel never sees a bar.
//!
//! ## Two crates, one pulse
//!
//! ```text
//! bar (clock)
//!   → HOST TA → beliefs          (numbers; not states)
//!   → Lift                       (category change → Msg, else silence)
//!   → newton-machine::apply      (XOR/AND truth, LCA, history sidecar)
//!   → project() extra bits
//!   → newtonian-core::step_entity (hysteretic scores → exact ROM)
//!   → Gateway::admit             (paper sleeve / alerts; no broker)
//! ```
//!
//! `step_entity` does **not** call `apply`. Ticks are not messages.
//! Unauthored superstates miss. See [[docs/adr/0020-kernel-sits-beside-the-chart]].
//!
//! ## Chart (newton-machine) — what is *true*
//!
//! ```text
//! Desk {
//!   quad: Warmup | Neutral | QuadOversold | QuadOverbought
//!   ema:  Warmup | Neutral | BelowTwoOrMore | Split | AboveTwoOrMore
//! }
//! ```
//!
//! Overlaps such as `QuadOversold ∩ Split` are **not** a third parent.
//!
//! ## Kernel (newtonian-core) — what to *do*
//!
//! Scores (beliefs, sticky): `ImpulseUp` / `ImpulseDown` / `VolumeHot`.
//! Extra bits: `Runtime::project()` of the live XOR children (bits 64+).
//! Exact table: only authored chords have firepower (LongLift, LongSuper,
//! ShortFade, LongSuperHot). Everything else is NONE.
//!
//! ## This file vs `newton-machine` `examples/aapl_1m.rs`
//!
//! That file applies **every bar** and scores overlaps in the host. This file
//! lifts only on stance change, runs the policy kernel every bar, and admits
//! through a gateway. Same lake; different program.

use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use newton_machine::bits::Bits;
use newton_machine::cmd::Cmd;
use newton_machine::machine::{Boot, Machine};
use newton_machine::runtime::Runtime;
use newton_machine::snapshot::Snapshot;
use newton_machine::topology::Topology;
use newton_machine::transition::{perform, Transitional};

use newtonian_core::prelude::*;

// =============================================================================
// HOST — incremental TA (not the chart, not the kernel, not a crate dep)
// =============================================================================

struct Ema {
    period: usize,
    alpha: f64,
    seed: VecDeque<f64>,
    value: Option<f64>,
}

impl Ema {
    fn new(period: usize) -> Self {
        assert!(period >= 1, "ema period");
        Self {
            period,
            alpha: 2.0 / (period as f64 + 1.0),
            seed: VecDeque::with_capacity(period),
            value: None,
        }
    }

    fn push(&mut self, close: f64) -> Option<f64> {
        if let Some(prev) = self.value {
            let next = self.alpha * close + (1.0 - self.alpha) * prev;
            self.value = Some(next);
            return Some(next);
        }
        self.seed.push_back(close);
        if self.seed.len() < self.period {
            return None;
        }
        let seed = self.seed.iter().sum::<f64>() / self.period as f64;
        self.seed.clear();
        self.value = Some(seed);
        Some(seed)
    }

    fn last(&self) -> Option<f64> {
        self.value
    }
}

#[derive(Clone, Copy, Debug)]
struct StochParams {
    k_period: usize,
    k_smooth: usize,
    d_period: usize,
}

impl StochParams {
    const fn fast(k_period: usize, d_period: usize) -> Self {
        Self {
            k_period,
            k_smooth: 1,
            d_period,
        }
    }
}

const EMA9: usize = 9;
const EMA21: usize = 21;
const EMA50: usize = 50;
const EMA200: usize = 200;
const FAST_9_3: StochParams = StochParams::fast(9, 3);
const FAST_14_3: StochParams = StochParams::fast(14, 3);
const FAST_40_3: StochParams = StochParams::fast(40, 3);
const FAST_60_3: StochParams = StochParams::fast(60, 3);
const VOL_MA: usize = 20;

struct Stoch {
    params: StochParams,
    highs: VecDeque<f64>,
    lows: VecDeque<f64>,
    raw_k: VecDeque<f64>,
    smooth_k: VecDeque<f64>,
    prev_raw_k: Option<f64>,
}

impl Stoch {
    fn new(params: StochParams) -> Self {
        assert!(params.k_period >= 1 && params.k_smooth >= 1 && params.d_period >= 1);
        Self {
            params,
            highs: VecDeque::with_capacity(params.k_period),
            lows: VecDeque::with_capacity(params.k_period),
            raw_k: VecDeque::with_capacity(params.k_smooth),
            smooth_k: VecDeque::with_capacity(params.d_period),
            prev_raw_k: None,
        }
    }

    fn push(&mut self, high: f64, low: f64, close: f64) -> Option<(f64, f64)> {
        push_window(&mut self.highs, self.params.k_period, high);
        push_window(&mut self.lows, self.params.k_period, low);
        if self.highs.len() < self.params.k_period {
            return None;
        }
        let hh = self.highs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let ll = self.lows.iter().copied().fold(f64::INFINITY, f64::min);
        let range = hh - ll;
        let raw = if range == 0.0 {
            self.prev_raw_k.unwrap_or(50.0)
        } else {
            100.0 * (close - ll) / range
        };
        self.prev_raw_k = Some(raw);
        push_window(&mut self.raw_k, self.params.k_smooth, raw);
        if self.raw_k.len() < self.params.k_smooth {
            return None;
        }
        let k = mean(&self.raw_k);
        push_window(&mut self.smooth_k, self.params.d_period, k);
        if self.smooth_k.len() < self.params.d_period {
            return None;
        }
        Some((k, mean(&self.smooth_k)))
    }
}

fn push_window(buf: &mut VecDeque<f64>, cap: usize, v: f64) {
    if buf.len() == cap {
        buf.pop_front();
    }
    buf.push_back(v);
}

fn mean(buf: &VecDeque<f64>) -> f64 {
    buf.iter().sum::<f64>() / buf.len() as f64
}

#[derive(Clone, Debug)]
struct Bar {
    ts: String,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

struct Ta {
    ema9: Ema,
    ema21: Ema,
    ema50: Ema,
    ema200: Ema,
    stoch9: Stoch,
    stoch14: Stoch,
    stoch40: Stoch,
    stoch60: Stoch,
    volumes: VecDeque<f64>,
    bar_i: u32,
}

impl Ta {
    fn new() -> Ta {
        Ta {
            ema9: Ema::new(EMA9),
            ema21: Ema::new(EMA21),
            ema50: Ema::new(EMA50),
            ema200: Ema::new(EMA200),
            stoch9: Stoch::new(FAST_9_3),
            stoch14: Stoch::new(FAST_14_3),
            stoch40: Stoch::new(FAST_40_3),
            stoch60: Stoch::new(FAST_60_3),
            volumes: VecDeque::with_capacity(VOL_MA),
            bar_i: 0,
        }
    }

    fn push(&mut self, bar: &Bar) -> Snap {
        self.bar_i += 1;
        let _ = self.ema9.push(bar.close);
        let _ = self.ema21.push(bar.close);
        let _ = self.ema50.push(bar.close);
        let _ = self.ema200.push(bar.close);
        push_window(&mut self.volumes, VOL_MA, bar.volume);
        let vol_ma = if self.volumes.len() == VOL_MA {
            Some(mean(&self.volumes))
        } else {
            None
        };
        Snap {
            ts: bar.ts.clone(),
            bar_i: self.bar_i,
            close: bar.close,
            volume: bar.volume,
            vol_ma,
            ema9: self.ema9.last(),
            ema21: self.ema21.last(),
            ema50: self.ema50.last(),
            ema200: self.ema200.last(),
            k9: self
                .stoch9
                .push(bar.high, bar.low, bar.close)
                .map(|kd| kd.0),
            k14: self
                .stoch14
                .push(bar.high, bar.low, bar.close)
                .map(|kd| kd.0),
            k40: self
                .stoch40
                .push(bar.high, bar.low, bar.close)
                .map(|kd| kd.0),
            k60: self
                .stoch60
                .push(bar.high, bar.low, bar.close)
                .map(|kd| kd.0),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Snap {
    ts: String,
    bar_i: u32,
    close: f64,
    volume: f64,
    vol_ma: Option<f64>,
    ema9: Option<f64>,
    ema21: Option<f64>,
    ema50: Option<f64>,
    ema200: Option<f64>,
    k9: Option<f64>,
    k14: Option<f64>,
    k40: Option<f64>,
    k60: Option<f64>,
}

impl Snap {
    fn emas(&self) -> [Option<f64>; 4] {
        [self.ema9, self.ema21, self.ema50, self.ema200]
    }

    fn stoch_k(&self) -> [Option<f64>; 4] {
        [self.k9, self.k14, self.k40, self.k60]
    }

    fn above_below(&self) -> (u8, u8) {
        let mut above = 0u8;
        let mut below = 0u8;
        for e in self.emas() {
            match e {
                Some(v) if self.close > v => above += 1,
                Some(v) if self.close < v => below += 1,
                _ => {}
            }
        }
        (above, below)
    }
}

const STOCH_OVERSOLD: f64 = 27.0;
const STOCH_OVERBOUGHT: f64 = 79.0;

// =============================================================================
// LIB — nested ADTs. Desk is Newton configuration (what is true).
// =============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum QuadStoch {
    Warmup,
    Neutral,
    QuadOversold,
    QuadOverbought,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EmaStance {
    Warmup,
    Neutral,
    BelowTwoOrMore,
    Split,
    AboveTwoOrMore,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Desk {
    quad: QuadStoch,
    ema: EmaStance,
}

impl Default for Desk {
    fn default() -> Self {
        Self {
            quad: QuadStoch::Warmup,
            ema: EmaStance::Warmup,
        }
    }
}

fn classify_quad(s: &Snap) -> QuadStoch {
    let ks = s.stoch_k();
    if ks.iter().any(Option::is_none) {
        return QuadStoch::Warmup;
    }
    let ks = [
        ks[0].unwrap(),
        ks[1].unwrap(),
        ks[2].unwrap(),
        ks[3].unwrap(),
    ];
    if ks.iter().all(|k| *k < STOCH_OVERSOLD) {
        QuadStoch::QuadOversold
    } else if ks.iter().all(|k| *k > STOCH_OVERBOUGHT) {
        QuadStoch::QuadOverbought
    } else {
        QuadStoch::Neutral
    }
}

fn classify_ema(s: &Snap) -> EmaStance {
    if s.emas().iter().any(Option::is_none) {
        return EmaStance::Warmup;
    }
    let (above, below) = s.above_below();
    match (above >= 2, below >= 2) {
        (true, true) => EmaStance::Split,
        (true, false) => EmaStance::AboveTwoOrMore,
        (false, true) => EmaStance::BelowTwoOrMore,
        (false, false) => EmaStance::Neutral,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Node {
    Root,
    Quad,
    Ema,
    QWarmup,
    QNeutral,
    QOversold,
    QOverbought,
    EWarmup,
    ENeutral,
    EBelow,
    ESplit,
    EAbove,
}

impl QuadStoch {
    fn node(self) -> Node {
        match self {
            QuadStoch::Warmup => Node::QWarmup,
            QuadStoch::Neutral => Node::QNeutral,
            QuadStoch::QuadOversold => Node::QOversold,
            QuadStoch::QuadOverbought => Node::QOverbought,
        }
    }
}

impl EmaStance {
    fn node(self) -> Node {
        match self {
            EmaStance::Warmup => Node::EWarmup,
            EmaStance::Neutral => Node::ENeutral,
            EmaStance::BelowTwoOrMore => Node::EBelow,
            EmaStance::Split => Node::ESplit,
            EmaStance::AboveTwoOrMore => Node::EAbove,
        }
    }
}

impl Topology for Desk {
    type Node = Node;
    fn parent(node: Node) -> Option<Node> {
        match node {
            Node::Root => None,
            Node::Quad | Node::Ema => Some(Node::Root),
            Node::QWarmup | Node::QNeutral | Node::QOversold | Node::QOverbought => {
                Some(Node::Quad)
            }
            Node::EWarmup | Node::ENeutral | Node::EBelow | Node::ESplit | Node::EAbove => {
                Some(Node::Ema)
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
struct History {
    last_quad: Option<QuadStoch>,
    last_ema: Option<EmaStance>,
    bars_in_last_quad: u32,
    last_extreme_ts: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ChartCmd {
    AlertOversold,
    AlertOverbought,
    Persist,
}

impl Transitional for Desk {
    type Ctx = Model;
    type Hist = History;
    type Cmd = Cmd<ChartCmd>;

    fn exit(&mut self, node: Node, ctx: &mut Model, hist: &mut History) -> Cmd<ChartCmd> {
        match node {
            Node::QOversold | Node::QOverbought => {
                hist.last_quad = Some(self.quad);
                hist.bars_in_last_quad = ctx.bars_in_quad;
                hist.last_extreme_ts = Some(ctx.ts.clone());
                Cmd::single(ChartCmd::Persist)
            }
            Node::EAbove | Node::EBelow | Node::ESplit => {
                hist.last_ema = Some(self.ema);
                Cmd::none()
            }
            _ => Cmd::none(),
        }
    }

    fn enter(&mut self, node: Node, ctx: &mut Model, _: &mut History) -> Cmd<ChartCmd> {
        match node {
            Node::QOversold => {
                ctx.bars_in_quad = 0;
                Cmd::single(ChartCmd::AlertOversold)
            }
            Node::QOverbought => {
                ctx.bars_in_quad = 0;
                Cmd::single(ChartCmd::AlertOverbought)
            }
            Node::QNeutral | Node::QWarmup => {
                ctx.bars_in_quad = 0;
                Cmd::none()
            }
            _ => Cmd::none(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Model {
    ts: String,
    close: f64,
    bars_in_quad: u32,
    bar_i: u32,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            ts: String::new(),
            close: 0.0,
            bars_in_quad: 0,
            bar_i: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Stance {
    quad: QuadStoch,
    ema: EmaStance,
    ts: String,
    close: f64,
    bar_i: u32,
}

impl Machine for Desk {
    type Flags = ();
    type Model = Model;
    type Msg = Stance;
    type Cmd = Cmd<ChartCmd>;
    type View = Desk;
    type History = History;
    type NodeId = Node;

    fn init(_: ()) -> Boot<Self> {
        Boot::new(
            Desk::default(),
            Model::default(),
            History::default(),
            Cmd::none(),
        )
    }

    fn update(&mut self, model: &mut Model, hist: &mut History, msg: Stance) -> Cmd<ChartCmd> {
        model.ts = msg.ts;
        model.close = msg.close;
        model.bar_i = msg.bar_i;
        model.bars_in_quad = model.bars_in_quad.saturating_add(1);
        let want = Desk {
            quad: msg.quad,
            ema: msg.ema,
        };
        let mut cmd = Cmd::none();
        if want.quad != self.quad {
            let dest = Desk {
                quad: want.quad,
                ema: self.ema,
            };
            cmd = cmd.and(perform(
                self,
                self.quad.node(),
                dest,
                want.quad.node(),
                model,
                hist,
            ));
        }
        if want.ema != self.ema {
            let dest = Desk {
                quad: self.quad,
                ema: want.ema,
            };
            cmd = cmd.and(perform(
                self,
                self.ema.node(),
                dest,
                want.ema.node(),
                model,
                hist,
            ));
        }
        cmd
    }

    fn view(&self, _: &Model) -> Desk {
        *self
    }

    fn in_state(&self, id: Node) -> bool {
        id == Node::Root
            || id == Node::Quad
            || id == Node::Ema
            || self.quad.node() == id
            || self.ema.node() == id
    }

    fn project(&self) -> Bits {
        let mut b = Bits::EMPTY;
        match self.quad {
            QuadStoch::QuadOversold => b.insert(X_Q_OVERSOLD),
            QuadStoch::QuadOverbought => b.insert(X_Q_OVERBOUGHT),
            QuadStoch::Neutral => b.insert(X_Q_NEUTRAL),
            QuadStoch::Warmup => {}
        }
        match self.ema {
            EmaStance::Split => b.insert(X_E_SPLIT),
            EmaStance::AboveTwoOrMore => b.insert(X_E_ABOVE),
            EmaStance::BelowTwoOrMore => b.insert(X_E_BELOW),
            EmaStance::Warmup | EmaStance::Neutral => {}
        }
        b
    }
}

// =============================================================================
// HOST beliefs + lift (ticks stay silent)
// =============================================================================

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum FactKey {
    Snap,
}

#[derive(Clone, Debug, PartialEq)]
enum Fact {
    Snap(Snap),
}

type Store = MemoryStore<FactKey, Fact, &'static str>;

#[derive(Clone, Copy, Debug)]
struct Policy {
    vol_hot: f64,
    max_paper: u32,
}

impl Mandate for Policy {
    type Id = u32;
    fn id(&self) -> u32 {
        self.max_paper
    }
}

struct StanceLift;

impl Lift for StanceLift {
    type Beliefs = Store;
    type Mandate = Policy;
    type Config = Desk;
    type Msg = Stance;
    type Key = FactKey;
    type Fact = Fact;

    fn lift(
        &self,
        now: &Store,
        _rev: &Revision<FactKey, Fact>,
        _mandate: &Policy,
        config: &Desk,
    ) -> Lifted<Stance> {
        let Some(Fact::Snap(snap)) = now.get(&FactKey::Snap).map(|b| &b.fact) else {
            return Lifted::Silence;
        };
        let quad = classify_quad(snap);
        let ema = classify_ema(snap);
        if quad == config.quad && ema == config.ema {
            Lifted::Silence
        } else {
            Lifted::Msg(Stance {
                quad,
                ema,
                ts: snap.ts.clone(),
                close: snap.close,
                bar_i: snap.bar_i,
            })
        }
    }
}

// =============================================================================
// KERNEL — scores from tape, extra from Newton, exact ROM
// =============================================================================

const IMPULSE_UP: ScoreId = ScoreId::new(0);
const IMPULSE_DOWN: ScoreId = ScoreId::new(1);
const VOLUME_HOT: ScoreId = ScoreId::new(2);

const X_Q_OVERSOLD: u32 = SCORE_WIDTH;
const X_Q_OVERBOUGHT: u32 = SCORE_WIDTH + 1;
const X_Q_NEUTRAL: u32 = SCORE_WIDTH + 2;
const X_E_SPLIT: u32 = SCORE_WIDTH + 3;
const X_E_ABOVE: u32 = SCORE_WIDTH + 4;
const X_E_BELOW: u32 = SCORE_WIDTH + 5;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Play {
    LongLift,
    LongSuper,
    LongSuperHot,
    ShortFade,
}

fn raw_scores(snap: &Snap, mandate: &Policy) -> Key {
    let mut raw = Key::EMPTY;
    if let Some(e9) = snap.ema9 {
        if snap.close > e9 {
            raw.set(IMPULSE_UP);
        } else if snap.close < e9 {
            raw.set(IMPULSE_DOWN);
        }
    }
    if let Some(ma) = snap.vol_ma {
        if ma > 0.0 && snap.volume > mandate.vol_hot * ma {
            raw.set(VOLUME_HOT);
        }
    }
    raw
}

fn extra_from(desk: &Desk) -> Key {
    Key::from_raw(desk.project().raw())
}

fn chord(scores: &[ScoreId], extras: &[u32]) -> Key {
    let mut k = Key::from_ids(scores.iter().copied());
    for e in extras {
        k = k.union(Key::from_bit(*e));
    }
    k
}

fn intern_rows() -> Vec<(Key, Play)> {
    // Exact product includes every in-play score bit. Impulse up/down are
    // mutually exclusive; intern both overlays plus volume-hot.
    let overlays: &[&[ScoreId]] = &[
        &[],
        &[IMPULSE_UP],
        &[IMPULSE_DOWN],
        &[VOLUME_HOT],
        &[IMPULSE_UP, VOLUME_HOT],
        &[IMPULSE_DOWN, VOLUME_HOT],
    ];
    let mut rows = Vec::new();
    let longs = [
        ([X_Q_OVERSOLD, X_E_SPLIT], Play::LongLift),
        ([X_Q_OVERSOLD, X_E_ABOVE], Play::LongSuper),
        ([X_Q_OVERSOLD, X_E_BELOW], Play::LongLift),
        ([X_Q_OVERBOUGHT, X_E_BELOW], Play::ShortFade),
        ([X_Q_OVERBOUGHT, X_E_ABOVE], Play::ShortFade),
        ([X_Q_OVERBOUGHT, X_E_SPLIT], Play::ShortFade),
    ];
    for scores in overlays {
        for (extras, play) in longs {
            let play = if play == Play::LongSuper
                && scores.contains(&VOLUME_HOT)
                && scores.contains(&IMPULSE_UP)
            {
                Play::LongSuperHot
            } else {
                play
            };
            rows.push((chord(scores, &extras), play));
        }
    }
    rows.sort_by_key(|r| r.0);
    rows.dedup_by_key(|r| r.0);
    rows
}

fn score_specs() -> [ScoreSpec; 3] {
    [
        ScoreSpec::new(IMPULSE_UP),
        ScoreSpec::new(IMPULSE_DOWN),
        ScoreSpec::new(VOLUME_HOT).with_sticky(2),
    ]
}

// =============================================================================
// HOST gateway — paper authority, not a broker
// =============================================================================

#[derive(Clone, Copy, Debug)]
enum Wire {
    Chart(ChartCmd),
    Enter(Play),
    Exit(Play),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Reason {
    PaperCap,
}

struct PaperWorld {
    fires: u32,
}

struct PaperGw;

impl Gateway for PaperGw {
    type Cmd = Wire;
    type Reason = Reason;
    type Mandate = Policy;
    type World = PaperWorld;

    fn admit(&self, cmd: Wire, mandate: &Policy, world: &PaperWorld) -> Admission<Wire, Reason> {
        match cmd {
            Wire::Enter(play) if world.fires >= mandate.max_paper => {
                let _ = play;
                Admission::Refuse {
                    reason: Reason::PaperCap,
                }
            }
            Wire::Enter(play) => Admission::Admit(Wire::Enter(play)),
            Wire::Exit(play) => Admission::Admit(Wire::Exit(play)),
            Wire::Chart(c) => Admission::Admit(Wire::Chart(c)),
        }
    }
}

// =============================================================================
// HOST — lake + pulse
// =============================================================================

fn csv_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/aapl_1m_2026-04.csv")
}

fn load_bars() -> Vec<Bar> {
    let f = File::open(csv_path()).unwrap_or_else(|e| {
        panic!(
            "missing {}: {e} (GitHub tree; DuckDB-extracted April 2026 AAPL 1m)",
            csv_path().display()
        )
    });
    let mut bars = Vec::new();
    for (i, line) in BufReader::new(f).lines().enumerate() {
        let line = line.expect("read");
        if i == 0 {
            continue;
        }
        let c: Vec<&str> = line.split(',').collect();
        if c.len() < 7 {
            continue;
        }
        bars.push(Bar {
            ts: c[1].to_string(),
            high: c[3].parse().expect("high"),
            low: c[4].parse().expect("low"),
            close: c[5].parse().expect("close"),
            volume: c[6].parse().expect("volume"),
        });
    }
    bars
}

fn main() {
    println!("# AAPL 1m  April 2026  newton-machine + newtonian-core\n");
    println!("truth = Desk (quad × ema)   policy = exact sleeve ROM   clock = 1m bar\n");

    let bars = load_bars();
    let mut ta = Ta::new();
    let (mut rt, _) = Runtime::<Desk>::boot(());
    let mut beliefs = Store::new();
    let lift = StanceLift;
    let mandate = Policy {
        vol_hot: 1.5,
        max_paper: 8,
    };
    let gateway = PaperGw;
    let mut world = PaperWorld { fires: 0 };

    let specs = score_specs();
    let rows = intern_rows();
    let table = Table::from_sorted(&rows);
    let folded = Folded::new(&specs, table);
    let mut entity = EntityState::new();

    let mut lift_msgs = 0u32;
    let mut chart_changes = 0u32;
    let mut kernel_enter = [0u32; 4];
    let mut kernel_miss = 0u32;
    let mut refused = 0u32;
    let mut restore_at: Option<(u32, Snapshot<Desk, Model, History>, EntityState)> = None;
    let mut restored = false;
    let mut live_play: Option<Play> = None;
    let mut chart_prints = 0u32;
    let mut kernel_prints = 0u32;

    for (i, bar) in bars.iter().enumerate() {
        let snap = ta.push(bar);
        let tick = i as u64;

        let previous = beliefs.get(&FactKey::Snap).map(|b| b.fact.clone());
        beliefs.revise(
            FactKey::Snap,
            Fact::Snap(snap.clone()),
            Justification {
                source: "lake",
                tick,
            },
        );
        let rev = Revision::new(FactKey::Snap, previous, Some(Fact::Snap(snap.clone())));

        let desk_before = *rt.machine();
        match lift.lift(&beliefs, &rev, &mandate, &desk_before) {
            Lifted::Msg(stance) => {
                lift_msgs += 1;
                let step = IntentionMachine::apply(&mut rt, stance);
                if *rt.machine() != desk_before {
                    chart_changes += 1;
                    if chart_prints < 12 {
                        print_chart(&desk_before, &rt, &step.cmd);
                        chart_prints += 1;
                    } else if chart_prints == 12 {
                        println!("  … further chart edges omitted (see footer counts)\n");
                        chart_prints += 1;
                    }
                }
                for atom in step.cmd.iter().copied() {
                    let _ = gateway.admit(Wire::Chart(atom), &mandate, &world);
                }
            }
            Lifted::Silence => {}
        }

        let kstep = folded.step(
            &mut entity,
            raw_scores(&snap, &mandate),
            extra_from(rt.machine()),
            Pulse::new(tick),
        );
        if kstep.sleeve.is_none() && kstep.transition != Transition::Hold {
            kernel_miss += 1;
        }
        if kstep.transition == Transition::Leave {
            if let Some(play) = live_play.take() {
                let _ = gateway.admit(Wire::Exit(play), &mandate, &world);
            }
        }
        if matches!(kstep.transition, Transition::Enter | Transition::Replace) {
            if let Some(play) = kstep.sleeve.copied() {
                match play {
                    Play::LongLift => kernel_enter[0] += 1,
                    Play::LongSuper => kernel_enter[1] += 1,
                    Play::LongSuperHot => kernel_enter[2] += 1,
                    Play::ShortFade => kernel_enter[3] += 1,
                }
                match gateway.admit(Wire::Enter(play), &mandate, &world) {
                    Admission::Admit(Wire::Enter(_)) => {
                        world.fires += 1;
                        live_play = Some(play);
                        if kernel_prints < 16 {
                            print_kernel(&snap, rt.machine(), &kstep, "admit");
                            kernel_prints += 1;
                        }
                        if restore_at.is_none() && play == Play::LongLift {
                            restore_at = Some((i as u32, rt.snapshot(), entity.clone()));
                        }
                    }
                    Admission::Refuse { reason } => {
                        refused += 1;
                        if kernel_prints < 16 {
                            print_kernel(
                                &snap,
                                rt.machine(),
                                &kstep,
                                &format!("refuse {reason:?}"),
                            );
                            kernel_prints += 1;
                        }
                    }
                    _ => {}
                }
            }
        }

        if !restored {
            if let Some((at, snap_rt, snap_ent)) = restore_at.clone() {
                if i as u32 == at + 40 {
                    println!("── restore demo (bar {i}; snapshot was bar {at}) ──");
                    println!("  beliefs still at {}  (restore is not the lake)", snap.ts);
                    println!("  chart before restore  {:?}", rt.machine());
                    rt.restore(snap_rt);
                    entity = snap_ent;
                    println!(
                        "  chart after restore   {:?}  last_quad={:?}",
                        rt.machine(),
                        rt.history().last_quad
                    );
                    println!("  next bars keep writing beliefs; lift may Resync.\n");
                    restored = true;
                }
            }
        }
    }

    let journal = rt.snapshot();
    println!("── footer ──");
    println!("  bars                 {}", bars.len());
    println!("  lift Msg             {lift_msgs}  (silence is the rest — ticks are not messages)");
    println!("  chart region changes {chart_changes}");
    println!("  kernel LongLift      {}", kernel_enter[0]);
    println!("  kernel LongSuper     {}", kernel_enter[1]);
    println!("  kernel LongSuperHot  {}", kernel_enter[2]);
    println!("  kernel ShortFade     {}", kernel_enter[3]);
    println!("  kernel leave→miss    {kernel_miss}");
    println!(
        "  gateway paper cap    {refused}  (mandate.max_paper={})",
        mandate.max_paper
    );
    println!("  snapshot.config      {:?}", journal.config);
    println!(
        "  snapshot.history     last_quad={:?} last_ema={:?} last_extreme_ts={}",
        journal.history.last_quad,
        journal.history.last_ema,
        journal.history.last_extreme_ts.as_deref().unwrap_or("—")
    );
}

fn print_chart(prev: &Desk, rt: &Runtime<Desk>, cmd: &Cmd<ChartCmd>) {
    let m = rt.model();
    let now = rt.machine();
    let sidecar = rt.history();
    println!(
        "── {}  close={:.2}  bar#{}  CHART ──",
        m.ts, m.close, m.bar_i
    );
    if prev.quad != now.quad {
        println!(
            "  stoch  {:?} → {:?}    sidecar last_quad={:?} bars_in_last_extreme={}",
            prev.quad, now.quad, sidecar.last_quad, sidecar.bars_in_last_quad
        );
    }
    if prev.ema != now.ema {
        println!(
            "  ema    {:?} → {:?}    last_ema={:?}",
            prev.ema, now.ema, sidecar.last_ema
        );
    }
    if !cmd.is_none() {
        print!("  cmd    ");
        let mut first = true;
        for c in cmd.iter() {
            if !first {
                print!(", ");
            }
            print!("{c:?}");
            first = false;
        }
        println!();
    }
    println!();
}

fn print_kernel(snap: &Snap, desk: &Desk, step: &KernelStep<'_, Play>, gate: &str) {
    println!(
        "── {}  close={:.2}  KERNEL {:?} {:?}  extra={:?}  {gate} ──",
        snap.ts, snap.close, step.transition, step.sleeve, desk
    );
    println!();
}
