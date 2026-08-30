//! A Newtonian program is the *composition*, not the loop.
//!
//! ```text
//! Program = Executive
//!         + one or more IntentionMachines
//!         + one BeliefStore
//!         + one Mandate
//!         + one Gateway
//!         + zero or more Skills
//!         + optional policy kernel (beside the chart, not a field here)
//! ```
//!
//! [`ProgramParts`] is a bag of handles so a host pulse and [`crate::exec`]
//! agree on the grain. It is **not** the Executive. Putting a `run()` here
//! would drag sockets and clocks into the ports. That is the soup we refused.
//!
//! The kernel ([`crate::step_entity`]) is **not** a seventh field. See
//! [[docs/adr/0020-kernel-sits-beside-the-chart]].

use crate::belief::BeliefStore;
use crate::gateway::Gateway;
use crate::intention::IntentionMachine;
use crate::lift::Lift;
use crate::mandate::Mandate;
use crate::skill::Skill;

/// Named composition of the four kinds of state plus lift and skills.
///
/// Associated types are constrained so lift `Msg` is the machine's `Msg`,
/// lift/gateway mandate is this mandate, lift beliefs/facts match the store,
/// and gateway `Cmd` is the machine's `Cmd`. The kernel sits beside this bag.
///
/// An Executive ([`crate::exec`], or a host loop) borrows these for one pulse:
/// ingest → revise → lift → step → admit → present.
pub struct ProgramParts<M, B, P, G, L, S = NoSkill>
where
    M: IntentionMachine,
    B: BeliefStore,
    P: Mandate,
    G: Gateway<Cmd = M::Cmd, Mandate = P>,
    L: Lift<Beliefs = B, Mandate = P, Config = M::View, Msg = M::Msg, Key = B::Key, Fact = B::Fact>,
    S: Skill,
{
    /// Committed stance.
    pub machine: M,
    /// Justified world facts.
    pub beliefs: B,
    /// Standing desires.
    pub mandate: P,
    /// Authority.
    pub gateway: G,
    /// Belief × mandate × config → Msg or silence.
    pub lift: L,
    /// Optional reflex bundle. [`NoSkill`] until a host has skills; use a
    /// tuple or a small vec in `alloc` hosts.
    pub skills: S,
}

impl<M, B, P, G, L, S> ProgramParts<M, B, P, G, L, S>
where
    M: IntentionMachine,
    B: BeliefStore,
    P: Mandate,
    G: Gateway<Cmd = M::Cmd, Mandate = P>,
    L: Lift<Beliefs = B, Mandate = P, Config = M::View, Msg = M::Msg, Key = B::Key, Fact = B::Fact>,
    S: Skill,
{
    /// Construct the bag. Does not start an Executive. Does not intern a ROM.
    pub fn new(machine: M, beliefs: B, mandate: P, gateway: G, lift: L, skills: S) -> Self {
        Self {
            machine,
            beliefs,
            mandate,
            gateway,
            lift,
            skills,
        }
    }
}

/// A skill that never pulses. Placeholder so [`ProgramParts`] can be named
/// without a skill bundle.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NoSkill;

impl Skill for NoSkill {
    type Input = ();
    type Output = ();

    fn name(&self) -> &'static str {
        "none"
    }

    fn pulse(&mut self, (): ()) {}
}
