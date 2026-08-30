//! Tiny reflexes the Executive may run. Not chart children.
//!
//! 3T skill layer / ATLANTIS controller / Brooks subsumption at L0/L1.
//! Heartbeat, cancel-on-disconnect enable, estop poll, watchdog pulse, UI
//! cursor blink — things that must run even if `update` is not on the path.
//!
//! Skills **override** the Executive only when the world demands a reflex
//! (subsumption). They do not subsume *inside* the chart. They do not call
//! `apply`. They may *post a belief* or *enqueue a Msg* for the Executive to
//! lift / apply on the next pulse.

/// A named reflex.
///
/// `pulse` is the skill's opportunity to look at the world. It returns data
/// (a belief patch, a command request, nothing). The Executive decides what
/// to do with that data. The skill does not own the machine.
pub trait Skill {
    /// Input the Executive passes in: clock, a belief slice, mandate.
    type Input;

    /// Output: "please revise this belief" / "please consider this Cmd" /
    /// silence. Domain crates specialize. Core keeps it associated.
    type Output;

    /// Stable name for logs and for the skill manager.
    fn name(&self) -> &'static str;

    /// One pulse. Pure with respect to the machine. May be called from a
    /// higher-priority loop than the Executive (L0/L1).
    fn pulse(&mut self, input: Self::Input) -> Self::Output;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Heartbeat {
        beats: u32,
    }

    impl Skill for Heartbeat {
        type Input = ();
        type Output = u32;

        fn name(&self) -> &'static str {
            "heartbeat"
        }

        fn pulse(&mut self, (): ()) -> u32 {
            self.beats += 1;
            self.beats
        }
    }

    #[test]
    fn skill_is_not_a_xor_child() {
        let mut s = Heartbeat { beats: 0 };
        assert_eq!(s.name(), "heartbeat");
        assert_eq!(s.pulse(()), 1);
    }
}
