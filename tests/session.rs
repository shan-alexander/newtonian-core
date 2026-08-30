//! HOST example: a session protocol, not a desk.
//!
//! Shows the five-way split on a connection machine. LIB is newtonian-core
//! ports. HOST is this file. The test *is* the Executive: hosts that already
//! have a pulse should look like this, and ignore `newtonian_core::exec`.

use newtonian_core::{
    Admission, Belief, BeliefStore, Freshness, Gateway, IntentionMachine, Justification, Lift,
    Lifted, Mandate, ProgramParts, Revision, Step,
};

// --- LIB-shaped HOST types (would live in a domain crate, not newtonian-core) ---

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Conn {
    Offline,
    Connecting { attempt: u8 },
    Online,
}

struct Session {
    conn: Conn,
}

impl IntentionMachine for Session {
    type Msg = Msg;
    type Cmd = Cmd;
    type View = Conn;
    type Snapshot = Conn;
    type NodeId = str;

    fn apply(&mut self, msg: Msg) -> Step<Cmd> {
        let cmd = match (self.conn, msg) {
            (Conn::Offline, Msg::Dial) => {
                self.conn = Conn::Connecting { attempt: 1 };
                Cmd::OpenSocket
            }
            (Conn::Connecting { attempt }, Msg::PeerHello) => {
                self.conn = Conn::Online;
                let _ = attempt;
                Cmd::None
            }
            (_, Msg::FeedStale) => {
                self.conn = Conn::Offline;
                Cmd::CloseSocket
            }
            (Conn::Connecting { attempt }, Msg::Dial) if attempt < 3 => {
                self.conn = Conn::Connecting {
                    attempt: attempt + 1,
                };
                Cmd::OpenSocket
            }
            _ => Cmd::None,
        };
        Step { cmd }
    }

    fn view(&self) -> Conn {
        self.conn
    }

    fn snapshot(&self) -> Conn {
        self.conn
    }

    fn in_state(&self, node: &str) -> bool {
        matches!(
            (node, self.conn),
            ("offline", Conn::Offline)
                | ("connecting", Conn::Connecting { .. })
                | ("online", Conn::Online)
        )
    }

    fn restore(&mut self, snapshot: Conn) {
        self.conn = snapshot;
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Msg {
    Dial,
    PeerHello,
    FeedStale,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Cmd {
    None,
    OpenSocket,
    CloseSocket,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Key {
    LastHello,
    Now,
}

struct Store {
    last_hello: Option<Belief<u64, &'static str>>,
    now: Option<Belief<u64, &'static str>>,
}

impl Store {
    fn empty() -> Self {
        Self {
            last_hello: None,
            now: None,
        }
    }

    fn slot(&self, key: Key) -> Option<&Belief<u64, &'static str>> {
        match key {
            Key::LastHello => self.last_hello.as_ref(),
            Key::Now => self.now.as_ref(),
        }
    }

    fn slot_mut(&mut self, key: Key) -> &mut Option<Belief<u64, &'static str>> {
        match key {
            Key::LastHello => &mut self.last_hello,
            Key::Now => &mut self.now,
        }
    }
}

impl BeliefStore for Store {
    type Key = Key;
    type Fact = u64;
    type Source = &'static str;

    fn revise(&mut self, key: Key, fact: u64, because: Justification<&'static str>) {
        *self.slot_mut(key) = Some(Belief {
            fact,
            because,
            freshness: Freshness::Fresh { tick: because.tick },
        });
    }

    fn withdraw(&mut self, key: &Key, _source: &&'static str) {
        if let Some(b) = self.slot_mut(*key).as_mut() {
            b.freshness = Freshness::Stale {
                last_fresh_tick: Some(b.because.tick),
            };
        }
    }

    fn get(&self, key: &Key) -> Option<&Belief<u64, &'static str>> {
        self.slot(*key)
    }

    fn get_mut(&mut self, key: &Key) -> Option<&mut Belief<u64, &'static str>> {
        self.slot_mut(*key).as_mut()
    }
}

struct Policy {
    stale_after: u64,
}

impl Mandate for Policy {
    type Id = u64;
    fn id(&self) -> u64 {
        self.stale_after
    }
    fn freshness_bound(&self) -> Option<u64> {
        Some(self.stale_after)
    }
}

struct HelloLift;

impl Lift for HelloLift {
    type Beliefs = Store;
    type Mandate = Policy;
    type Config = Conn;
    type Msg = Msg;
    type Key = Key;
    type Fact = u64;

    fn lift(
        &self,
        now: &Store,
        rev: &Revision<Key, u64>,
        mandate: &Policy,
        _config: &Conn,
    ) -> Lifted<Msg> {
        let hello_now = now.get(&Key::LastHello).map(|b| b.fact).unwrap_or(0);
        let hello_was = match rev.key {
            Key::LastHello => rev.previous.unwrap_or(hello_now),
            Key::Now => hello_now,
        };
        let t_now = now.get(&Key::Now).map(|b| b.fact).unwrap_or(0);
        let t_was = match rev.key {
            Key::Now => rev.previous.unwrap_or(t_now),
            Key::LastHello => t_now,
        };
        let was = t_was.saturating_sub(hello_was) >= mandate.stale_after;
        let is = t_now.saturating_sub(hello_now) >= mandate.stale_after;
        if !was && is {
            Lifted::Msg(Msg::FeedStale)
        } else {
            Lifted::Silence
        }
    }
}

struct Wire {
    locked: bool,
}

impl Gateway for Wire {
    type Cmd = Cmd;
    type Reason = &'static str;
    type Mandate = Policy;
    type World = bool;

    fn admit(&self, cmd: Cmd, _m: &Policy, locked: &bool) -> Admission<Cmd, &'static str> {
        if *locked || self.locked {
            Admission::Drop
        } else {
            Admission::Admit(cmd)
        }
    }
}

// --- HOST loop (this test *is* the Executive, written out) ---

#[test]
fn ticks_do_not_become_messages_until_stale() {
    let mut machine = Session {
        conn: Conn::Offline,
    };
    let mut beliefs = Store::empty();
    let mandate = Policy { stale_after: 5 };
    let lift = HelloLift;
    let gateway = Wire { locked: false };

    let step = machine.apply(Msg::Dial);
    assert_eq!(step.cmd, Cmd::OpenSocket);
    assert!(matches!(
        gateway.admit(step.cmd, &mandate, &false),
        Admission::Admit(Cmd::OpenSocket)
    ));
    let _ = machine.apply(Msg::PeerHello);
    assert!(machine.in_state("online"));

    fn clock(store: &mut Store, tick: u64) -> Revision<Key, u64> {
        let previous = store.get(&Key::Now).map(|b| b.fact);
        store.revise(
            Key::Now,
            tick,
            Justification {
                source: "clock",
                tick,
            },
        );
        Revision::new(Key::Now, previous, Some(tick))
    }

    // Hello at t=4, then clock-only pulses. Age 1..=4 is silence.
    beliefs.revise(
        Key::LastHello,
        4,
        Justification {
            source: "peer",
            tick: 4,
        },
    );
    let _ = clock(&mut beliefs, 4);
    for tick in 5..=8 {
        let rev = clock(&mut beliefs, tick);
        assert!(
            matches!(
                lift.lift(&beliefs, &rev, &mandate, &machine.view()),
                Lifted::Silence
            ),
            "tick {tick} is still inside the bound"
        );
    }

    // t=9: age = 9-4 = 5, crosses the bound. That is the Msg.
    let rev = clock(&mut beliefs, 9);
    assert!(matches!(
        lift.lift(&beliefs, &rev, &mandate, &machine.view()),
        Lifted::Msg(Msg::FeedStale)
    ));
    let step = machine.apply(Msg::FeedStale);
    assert!(machine.in_state("offline"));
    assert_eq!(step.cmd, Cmd::CloseSocket);

    let snap = machine.snapshot();
    machine.restore(Conn::Online);
    assert!(machine.in_state("online"));
    machine.restore(snap);
    assert!(machine.in_state("offline"));
}

#[test]
fn program_parts_bind_the_ports() {
    let parts = ProgramParts::new(
        Session {
            conn: Conn::Offline,
        },
        Store::empty(),
        Policy { stale_after: 5 },
        Wire { locked: false },
        HelloLift,
        newtonian_core::NoSkill,
    );
    assert!(parts.machine.in_state("offline"));
    assert_eq!(parts.mandate.id(), 5);
}
