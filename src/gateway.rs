//! Authority. What we are allowed to do to the world.
//!
//! The machine emits **intent** (`Cmd`). The gateway **admits or refuses**.
//! Actuate out of band. Reconcile in band (a later `Msg` from the Executive).
//! Never invert that. Never give the machine a `Broker` / `Socket` / `Client`
//! trait to call.
//!
//! This trait still has **no I/O**. Admission is a pure decision. The
//! Executive (or a process next to it) performs the admitted effect. That
//! split is how the gateway can panic, restart, or lock the wire without
//! taking the intention snapshot down with it — OTP supervision, stolen on
//! purpose.

use core::marker::PhantomData;

/// Result of asking the gateway whether a command may touch the world.
#[must_use]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Admission<Cmd, Reason> {
    /// The Executive may execute this (or hand it to an effector).
    Admit(Cmd),
    /// Refused. The Executive should typically feed a reconcile `Msg` so the
    /// machine learns what the world did *not* do.
    Refuse {
        /// Why. Domain-defined. Core does not know about PDT, estop, or 403.
        reason: Reason,
    },
    /// Already locked; drop silently or log. Still not a machine call.
    Drop,
}

/// Pure authority check.
///
/// Implementations may read mandate and a *slice* of beliefs (account state,
/// interlock, quota). They must not call `apply`. They must not perform I/O.
pub trait Gateway {
    /// Command type, usually the machine's `Cmd` or a host mapping of it.
    type Cmd;

    /// Why a command was refused.
    type Reason;

    /// Standing aims, if the gateway cares (limits, calendars).
    type Mandate;

    /// Belief slice the check needs (not the whole store).
    type World;

    /// Admit, refuse, or drop.
    fn admit(
        &self,
        cmd: Self::Cmd,
        mandate: &Self::Mandate,
        world: &Self::World,
    ) -> Admission<Self::Cmd, Self::Reason>;
}

/// A gateway that admits everything. Tests and dry-run hosts. Not a production
/// default — a named hole.
///
/// Generic over `Cmd` so a host can admit machine or sleeve payloads without
/// inventing a wrapper.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OpenGateway<Cmd = ()> {
    _cmd: PhantomData<fn() -> Cmd>,
}

impl<Cmd> Default for OpenGateway<Cmd> {
    fn default() -> Self {
        Self { _cmd: PhantomData }
    }
}

impl<Cmd> Gateway for OpenGateway<Cmd> {
    type Cmd = Cmd;
    type Reason = ();
    type Mandate = ();
    type World = ();

    fn admit(&self, cmd: Cmd, (): &(), (): &()) -> Admission<Cmd, ()> {
        Admission::Admit(cmd)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Locked;

    impl Gateway for Locked {
        type Cmd = u8;
        type Reason = &'static str;
        type Mandate = ();
        type World = bool;

        fn admit(&self, cmd: u8, (): &(), world: &bool) -> Admission<u8, &'static str> {
            if *world {
                Admission::Drop
            } else {
                Admission::Admit(cmd)
            }
        }
    }

    #[test]
    fn lock_is_not_a_chart_child() {
        let g = Locked;
        assert!(matches!(g.admit(1, &(), &true), Admission::Drop));
        assert!(matches!(g.admit(1, &(), &false), Admission::Admit(1)));
    }
}
