//! # Mosquito trap — 500 ms camera pulse as a Newtonian program
//!
//! Run: `cargo run --example mosquito --release --features machine,fold`
//!
//! GitHub-only (not crates.io). No sockets, no GPIO, no laser. `Cmd` is printed.
//!
//! Hypothetical turret: a camera sample every **500 ms** is the Mealy clock
//! (same kernel pulse as a 1 m bar or a game frame — the host names it).
//! Each frame is reduced to a handful of features. The chart names *stance*.
//! The kernel names *whether to fire / lure*. The gateway is the interlock.
//!
//! ## Scene
//!
//! A fake sensor synthesizes blobs. Mosquitoes cruise slowly (~0.5–1.5 m/s)
//! and are small (~3–8 mm). Moths are larger. Dust is tiny and jittery.
//! Speed and size are **gates** (raw classifiers). Confidence is a
//! **hysteretic score**. Image coordinates are beliefs, not XOR children.
//!
//! ## Chart (newton-machine)
//!
//! ```text
//! Patrol → Attract → Track → Engage → Cooldown ↺ Patrol
//!                    ↘ Lost ↗
//! Halt ← interlock
//! ```
//!
//! ## Kernel (newtonian-core)
//!
//! Scores: `SizeOk`, `SpeedOk`, `InView`, `HighConf` (sticky 2 s), `OnTarget`
//! (enabled by `HighConf`). Extra bits = `project()` of the live mode.
//! Exact ROM: fire only on `{Engage, HighConf, OnTarget, InView}`. A confident
//! track that is not yet Engage is Aim, not Fire. Unauthored keys miss.
//!
//! See [[docs/adr/0020-kernel-sits-beside-the-chart]].

use newton_machine::bits::Bits;
use newton_machine::cmd::Cmd;
use newton_machine::machine::{Boot, Machine};
use newton_machine::runtime::Runtime;
use newton_machine::topology::Topology;
use newton_machine::transition::{perform, Transitional};

use newtonian_core::prelude::*;

const TICK_S: f64 = 0.5;
const WIDTH: f64 = 640.0;
const HEIGHT: f64 = 480.0;
/// ~1.5 mm per pixel at a 2 m trap plane (HOST geometry, not the crate).
const MM_PER_PX: f64 = 1.5;

// =============================================================================
// HOST — fake camera (deterministic script, not a learned detector)
// =============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    None,
    Dust,
    Moth,
    Mosquito,
}

#[derive(Clone, Copy, Debug)]
struct Blob {
    kind: Kind,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    vx: f64,
    vy: f64,
}

#[derive(Clone, Copy, Debug)]
struct Frame {
    interlock: bool,
    blob: Option<Blob>,
}

/// Scripted 30 s of 500 ms frames. Interpolation is HOST, not Newton.
fn camera(tick: u64) -> Frame {
    let t = tick as f64 * TICK_S;
    let interlock = (28.0..31.0).contains(&t);
    let blob = if (6.5..8.5).contains(&t) {
        Some(dust(tick, t))
    } else if (9.0..12.0).contains(&t) {
        Some(moth(t))
    } else if (13.0..19.0).contains(&t) {
        Some(mosquito(t, 13.0, true))
    } else if (22.0..26.0).contains(&t) {
        Some(mosquito(t, 22.0, false))
    } else {
        None
    };
    Frame { interlock, blob }
}

fn dust(tick: u64, t: f64) -> Blob {
    let jitter = ((tick as f64 * 17.0).sin()) * 40.0;
    Blob {
        kind: Kind::Dust,
        x: 320.0 + jitter,
        y: 200.0 + (t * 30.0).sin() * 80.0,
        w: 2.0,
        h: 2.0,
        vx: 90.0,
        vy: 40.0,
    }
}

fn moth(t: f64) -> Blob {
    let u = (t - 9.0) / 3.0;
    Blob {
        kind: Kind::Moth,
        x: 80.0 + u * 400.0,
        y: 240.0,
        w: 28.0,
        h: 18.0,
        vx: 130.0,
        vy: 5.0,
    }
}

/// `center`: fly across the kill box. `!center`: graze the frame edge.
fn mosquito(t: f64, t0: f64, center: bool) -> Blob {
    let u = (t - t0) / 6.0;
    let y = if center {
        240.0 + (u * std::f64::consts::PI).sin() * 20.0
    } else {
        30.0 + u * 40.0
    };
    Blob {
        kind: Kind::Mosquito,
        x: 40.0 + u * 560.0,
        y,
        w: 6.0,
        h: 4.0,
        vx: 22.0,
        vy: if center { 6.0 } else { 10.0 },
    }
}

#[derive(Clone, Copy, Debug)]
struct Features {
    kind: Kind,
    x: f64,
    y: f64,
    size_mm: f64,
    speed_mps: f64,
    in_view: bool,
    size_ok: bool,
    speed_ok: bool,
    conf: f64,
    on_target: bool,
}

fn features(blob: Option<Blob>) -> Features {
    let Some(b) = blob else {
        return Features {
            kind: Kind::None,
            x: 0.0,
            y: 0.0,
            size_mm: 0.0,
            speed_mps: 0.0,
            in_view: false,
            size_ok: false,
            speed_ok: false,
            conf: 0.0,
            on_target: false,
        };
    };
    let size_px = (b.w + b.h) * 0.5;
    let size_mm = size_px * MM_PER_PX;
    let speed_px_s = (b.vx * b.vx + b.vy * b.vy).sqrt();
    let speed_mps = (speed_px_s * MM_PER_PX) / 1000.0;
    // Gates are in *image* units: a circling mosquito is small and slow on the
    // sensor. Moths are large; dust is tiny/jittery. mm/s is display only.
    let size_ok = (4.0..11.0).contains(&size_px);
    let speed_ok = (12.0..70.0).contains(&speed_px_s);
    let margin = 48.0;
    let in_view = b.x > margin && b.x < WIDTH - margin && b.y > margin && b.y < HEIGHT - margin;
    let cx = (b.x - WIDTH / 2.0).abs();
    let cy = (b.y - HEIGHT / 2.0).abs();
    let on_target = in_view && cx < 90.0 && cy < 70.0;
    let mut conf = 0.0;
    if size_ok {
        conf += 0.35;
    }
    if speed_ok {
        conf += 0.35;
    }
    if in_view {
        conf += 0.15;
    }
    if b.kind == Kind::Mosquito {
        conf += 0.15;
    }
    Features {
        kind: b.kind,
        x: b.x,
        y: b.y,
        size_mm,
        speed_mps,
        in_view,
        size_ok,
        speed_ok,
        conf,
        on_target,
    }
}

// =============================================================================
// LIB — stance chart
// =============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mode {
    Patrol,
    Attract,
    Track,
    Engage,
    Cooldown,
    Halt,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Node {
    Root,
    Patrol,
    Attract,
    Track,
    Engage,
    Cooldown,
    Halt,
}

impl Mode {
    fn node(self) -> Node {
        match self {
            Mode::Patrol => Node::Patrol,
            Mode::Attract => Node::Attract,
            Mode::Track => Node::Track,
            Mode::Engage => Node::Engage,
            Mode::Cooldown => Node::Cooldown,
            Mode::Halt => Node::Halt,
        }
    }
}

impl Topology for Mode {
    type Node = Node;
    fn parent(node: Node) -> Option<Node> {
        match node {
            Node::Root => None,
            _ => Some(Node::Root),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct History {
    last: Option<Mode>,
    shots: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ChartCmd {
    StartCo2,
    StartViolet,
    StopLure,
    Arm,
    Disarm,
    HaltMotors,
}

#[derive(Clone, Copy, Debug, Default)]
struct Model;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Msg {
    Lure,
    Acquired,
    Lost,
    Locked,
    CycleDone,
    Trip,
}

impl Transitional for Mode {
    type Ctx = Model;
    type Hist = History;
    type Cmd = Cmd<ChartCmd>;

    fn exit(&mut self, node: Node, _: &mut Model, hist: &mut History) -> Cmd<ChartCmd> {
        if matches!(
            node,
            Node::Patrol | Node::Attract | Node::Track | Node::Engage | Node::Cooldown
        ) {
            hist.last = Some(*self);
        }
        match node {
            Node::Attract => Cmd::single(ChartCmd::StopLure),
            Node::Engage => Cmd::single(ChartCmd::Disarm),
            _ => Cmd::none(),
        }
    }

    fn enter(&mut self, node: Node, _ctx: &mut Model, hist: &mut History) -> Cmd<ChartCmd> {
        match node {
            Node::Attract => {
                Cmd::single(ChartCmd::StartCo2).and(Cmd::single(ChartCmd::StartViolet))
            }
            Node::Engage => {
                hist.shots = hist.shots.saturating_add(1);
                Cmd::single(ChartCmd::Arm)
            }
            Node::Halt => Cmd::single(ChartCmd::HaltMotors),
            _ => Cmd::none(),
        }
    }
}

impl Machine for Mode {
    type Flags = ();
    type Model = Model;
    type Msg = Msg;
    type Cmd = Cmd<ChartCmd>;
    type View = Mode;
    type History = History;
    type NodeId = Node;

    fn init(_: ()) -> Boot<Self> {
        Boot::new(Mode::Patrol, Model, History::default(), Cmd::none())
    }

    fn update(&mut self, model: &mut Model, hist: &mut History, msg: Msg) -> Cmd<ChartCmd> {
        let dest = match (*self, msg) {
            (_, Msg::Trip) => Mode::Halt,
            (Mode::Halt, _) => Mode::Halt,
            (Mode::Patrol, Msg::Lure) => Mode::Attract,
            (Mode::Patrol | Mode::Attract, Msg::Acquired) => Mode::Track,
            (Mode::Track, Msg::Locked) => Mode::Engage,
            (Mode::Track | Mode::Engage, Msg::Lost) => Mode::Attract,
            (Mode::Engage, Msg::CycleDone) => Mode::Cooldown,
            (Mode::Cooldown, Msg::CycleDone) => Mode::Patrol,
            (mode, _) => mode,
        };
        if dest == *self {
            return Cmd::none();
        }
        perform(self, self.node(), dest, dest.node(), model, hist)
    }

    fn view(&self, _: &Model) -> Mode {
        *self
    }

    fn in_state(&self, id: Node) -> bool {
        id == Node::Root || self.node() == id
    }

    fn project(&self) -> Bits {
        let mut b = Bits::EMPTY;
        b.insert(match self {
            Mode::Patrol => X_PATROL,
            Mode::Attract => X_ATTRACT,
            Mode::Track => X_TRACK,
            Mode::Engage => X_ENGAGE,
            Mode::Cooldown => X_COOL,
            Mode::Halt => X_HALT,
        });
        b
    }
}

// =============================================================================
// HOST beliefs + lift
// =============================================================================

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum FactKey {
    Feat,
    Empty,
    Interlock,
    ShotAt,
}

#[derive(Clone, Copy, Debug)]
enum Fact {
    Feat(Features),
    Count(u32),
    Flag(bool),
    Tick(u64),
}

impl Fact {
    fn as_tick(self) -> Option<u64> {
        match self {
            Fact::Tick(t) => Some(t),
            _ => None,
        }
    }
}

type Store = MemoryStore<FactKey, Fact, &'static str>;

#[derive(Clone, Copy, Debug)]
struct Standing {
    lure_after: u32,
    max_shots: u8,
    cool_ticks: u32,
}

impl Mandate for Standing {
    type Id = u8;
    fn id(&self) -> u8 {
        self.max_shots
    }
}

struct BugLift;

impl Lift for BugLift {
    type Beliefs = Store;
    type Mandate = Standing;
    type Config = Mode;
    type Msg = Msg;
    type Key = FactKey;
    type Fact = Fact;

    fn lift(
        &self,
        now: &Store,
        rev: &Revision<FactKey, Fact>,
        mandate: &Standing,
        config: &Mode,
    ) -> Lifted<Msg> {
        if matches!(
            now.get(&FactKey::Interlock).map(|b| b.fact),
            Some(Fact::Flag(true))
        ) && *config != Mode::Halt
        {
            return Lifted::Msg(Msg::Trip);
        }
        let feat = match now.get(&FactKey::Feat).map(|b| b.fact) {
            Some(Fact::Feat(f)) => f,
            _ => return Lifted::Silence,
        };
        let empty = match now.get(&FactKey::Empty).map(|b| b.fact) {
            Some(Fact::Count(n)) => n,
            _ => 0,
        };
        let prev_conf = match (&rev.key, &rev.previous) {
            (FactKey::Feat, Some(Fact::Feat(p))) => p.conf,
            _ => feat.conf,
        };

        if *config == Mode::Patrol && empty >= mandate.lure_after {
            return Lifted::Msg(Msg::Lure);
        }
        if matches!(config, Mode::Patrol | Mode::Attract) && prev_conf < 0.7 && feat.conf >= 0.7 {
            return Lifted::Msg(Msg::Acquired);
        }
        if matches!(config, Mode::Track | Mode::Engage) && prev_conf >= 0.7 && feat.conf < 0.55 {
            return Lifted::Msg(Msg::Lost);
        }
        if *config == Mode::Track && feat.on_target && feat.conf >= 0.7 {
            return Lifted::Msg(Msg::Locked);
        }
        Lifted::Silence
    }
}

// Cooldown ticks are an Executive clock, not a chart child. `main` injects
// `Msg::CycleDone` when the mandate cool-down elapses.

// =============================================================================
// KERNEL
// =============================================================================

const HIGH_CONF: ScoreId = ScoreId::new(0);
const ON_TARGET: ScoreId = ScoreId::new(1);

const X_PATROL: u32 = SCORE_WIDTH;
const X_ATTRACT: u32 = SCORE_WIDTH + 1;
const X_TRACK: u32 = SCORE_WIDTH + 2;
const X_ENGAGE: u32 = SCORE_WIDTH + 3;
const X_COOL: u32 = SCORE_WIDTH + 4;
const X_HALT: u32 = SCORE_WIDTH + 5;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Play {
    Scan,
    LureCo2,
    LureViolet,
    Aim,
    Fire,
}

fn raw_from(f: &Features) -> Key {
    let mut k = Key::EMPTY;
    // Size and speed are HOST gates into HighConf, not extra ROM bits.
    if f.size_ok && f.speed_ok && f.conf >= 0.7 {
        k.set(HIGH_CONF);
    }
    if f.on_target {
        k.set(ON_TARGET);
    }
    k
}

fn trap_catalog() -> Catalog<Play> {
    Catalog::new()
        .score("HighConf", HIGH_CONF)
        .score("OnTarget", ON_TARGET)
        .extra("Patrol", X_PATROL)
        .extra("Attract", X_ATTRACT)
        .extra("Track", X_TRACK)
        .extra("Engage", X_ENGAGE)
        .extra("Cooldown", X_COOL)
        .extra("Halt", X_HALT)
        .play("Scan", Play::Scan)
        .play("LureCo2", Play::LureCo2)
        .play("LureViolet", Play::LureViolet)
        .play("Aim", Play::Aim)
        .play("Fire", Play::Fire)
        .knob("power")
}

// =============================================================================
// Gateway
// =============================================================================

#[derive(Clone, Copy, Debug)]
enum Wire {
    Chart(ChartCmd),
    Sleeve(Play),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Reason {
    Interlock,
    ShotBudget,
    NotArmed,
}

struct World {
    interlock: bool,
    shots: u8,
    armed: bool,
}

struct Safety;

impl Gateway for Safety {
    type Cmd = Wire;
    type Reason = Reason;
    type Mandate = Standing;
    type World = World;

    fn admit(&self, cmd: Wire, mandate: &Standing, world: &World) -> Admission<Wire, Reason> {
        if world.interlock && matches!(cmd, Wire::Sleeve(Play::Fire) | Wire::Chart(ChartCmd::Arm)) {
            return Admission::Refuse {
                reason: Reason::Interlock,
            };
        }
        match cmd {
            Wire::Sleeve(Play::Fire) if !world.armed => Admission::Refuse {
                reason: Reason::NotArmed,
            },
            Wire::Sleeve(Play::Fire) if world.shots >= mandate.max_shots => Admission::Refuse {
                reason: Reason::ShotBudget,
            },
            other => Admission::Admit(other),
        }
    }
}

fn main() {
    println!("# mosquito trap   clock = 500 ms camera   newton-machine + newtonian-core\n");
    println!("gates = size ∩ speed   scores = HighConf / OnTarget (sticky)   fire = exact ROM\n");

    let (mut rt, _) = Runtime::<Mode>::boot(());
    let mut beliefs = Store::new();
    let lift = BugLift;
    let mandate = Standing {
        lure_after: 5,
        max_shots: 2,
        cool_ticks: 4,
    };
    let gateway = Safety;
    let mut world = World {
        interlock: false,
        shots: 0,
        armed: false,
    };

    let catalog = trap_catalog();
    let power = catalog.knob_id("power").expect("power knob");
    let policy = catalog
        .fold_toml(include_str!("mosquito_policy.toml"))
        .unwrap_or_else(|e| panic!("fold mosquito_policy.toml: {e}"));
    let mut entity = EntityState::new();
    let mut empty = 0u32;
    let mut cool_left: Option<u32> = None;
    let mut fires = 0u32;
    let mut misses = 0u32;

    for tick in 0..70 {
        let frame = camera(tick);
        let feat = features(frame.blob);
        let t = tick as f64 * TICK_S;
        world.interlock = frame.interlock;

        if feat.kind == Kind::None {
            empty += 1;
        } else {
            empty = 0;
            if !feat.size_ok || !feat.speed_ok {
                println!(
                    "  t={t:4.1}s  HOST gate {:?}  size_ok={} speed_ok={} (not a mosquito classifier)",
                    feat.kind, feat.size_ok, feat.speed_ok
                );
            }
        }

        let prev_feat = beliefs.get(&FactKey::Feat).map(|b| b.fact);
        revise(
            &mut beliefs,
            FactKey::Feat,
            Fact::Feat(feat),
            tick,
            "camera",
        );
        revise(
            &mut beliefs,
            FactKey::Empty,
            Fact::Count(empty),
            tick,
            "camera",
        );
        revise(
            &mut beliefs,
            FactKey::Interlock,
            Fact::Flag(frame.interlock),
            tick,
            "estop",
        );

        let mode_before = *rt.machine();
        let rev = Revision::new(FactKey::Feat, prev_feat, Some(Fact::Feat(feat)));
        let mut msg = match lift.lift(&beliefs, &rev, &mandate, &mode_before) {
            Lifted::Msg(m) => Some(m),
            Lifted::Silence => None,
        };
        if mode_before == Mode::Cooldown {
            if let Some(left) = cool_left.as_mut() {
                *left = left.saturating_sub(1);
                if *left == 0 {
                    msg = Some(Msg::CycleDone);
                    cool_left = None;
                }
            }
        }

        if let Some(m) = msg {
            let step = IntentionMachine::apply(&mut rt, m);
            if *rt.machine() != mode_before {
                print_mode(t, mode_before, *rt.machine(), feat, &step.cmd);
            }
            for atom in step.cmd.iter().copied() {
                match gateway.admit(Wire::Chart(atom), &mandate, &world) {
                    Admission::Admit(Wire::Chart(ChartCmd::Arm)) => world.armed = true,
                    Admission::Admit(Wire::Chart(ChartCmd::Disarm)) => world.armed = false,
                    Admission::Refuse { reason } => {
                        println!("  t={t:4.1}s  chart {:?} refused {reason:?}", atom);
                    }
                    _ => {}
                }
            }
        }

        let kstep = policy.step(
            &mut entity,
            raw_from(&feat),
            Key::from_raw(rt.machine().project().raw()),
            Pulse::new(tick),
        );
        if kstep.sleeve.is_none() && feat.kind != Kind::None && kstep.transition != Transition::Hold
        {
            misses += 1;
            println!(
                "  t={t:4.1}s  KERNEL miss  {:?}  size={:.1}mm speed={:.2}m/s conf={:.2} in_view={}  (unauthored)",
                feat.kind, feat.size_mm, feat.speed_mps, feat.conf, feat.in_view
            );
        }
        if matches!(kstep.transition, Transition::Enter | Transition::Replace) {
            if let Some(sleeve) = kstep.sleeve.copied() {
                if sleeve.observe_only {
                    continue;
                }
                let play = sleeve.play;
                match gateway.admit(Wire::Sleeve(play), &mandate, &world) {
                    Admission::Admit(Wire::Sleeve(Play::Fire)) => {
                        fires += 1;
                        world.shots += 1;
                        world.armed = false;
                        cool_left = Some(mandate.cool_ticks);
                        revise(
                            &mut beliefs,
                            FactKey::ShotAt,
                            Fact::Tick(tick),
                            tick,
                            "laser",
                        );
                        let _ = IntentionMachine::apply(&mut rt, Msg::CycleDone);
                        println!(
                            "  t={t:4.1}s  FIRE  at ({:.0},{:.0})  shots={}  power={:?}  {:?}",
                            feat.x,
                            feat.y,
                            world.shots,
                            sleeve.knobs.get(power),
                            rt.machine()
                        );
                    }
                    Admission::Admit(Wire::Sleeve(p)) => {
                        println!(
                            "  t={t:4.1}s  KERNEL {:?} {:?}  {:?}/conf={:.2}",
                            kstep.transition, p, feat.kind, feat.conf
                        );
                    }
                    Admission::Refuse { reason } => {
                        println!("  t={t:4.1}s  KERNEL {:?} refused {reason:?}", play);
                    }
                    _ => {}
                }
            }
        }
    }

    println!("\n── footer ──");
    println!("  pulses     70  (35 s @ 500 ms)");
    println!("  mode now   {:?}", rt.machine());
    println!("  fires      {fires}");
    println!("  ROM misses {misses}  (dust/moth/edge — exact key, no guess)");
    println!(
        "  sidecar    last={:?} shots={}",
        rt.history().last,
        rt.history().shots
    );
    println!("  snapshot   {:?}", rt.snapshot().config);
    if let Some(t) = beliefs.get(&FactKey::ShotAt).and_then(|b| b.fact.as_tick()) {
        println!("  last shot  t={:.1}s", t as f64 * TICK_S);
    }
}

fn revise(store: &mut Store, key: FactKey, fact: Fact, tick: u64, source: &'static str) {
    store.revise(key, fact, Justification { source, tick });
}

fn print_mode(t: f64, prev: Mode, now: Mode, feat: Features, cmd: &Cmd<ChartCmd>) {
    print!("  t={t:4.1}s  CHART {prev:?} → {now:?}");
    if feat.kind != Kind::None {
        print!(
            "  {:?} {:.1}mm conf={:.2} view={}",
            feat.kind, feat.size_mm, feat.conf, feat.in_view
        );
    }
    if !cmd.is_none() {
        print!("  cmd=");
        for c in cmd.iter() {
            print!("{c:?} ");
        }
    }
    println!();
}
