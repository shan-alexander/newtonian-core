//! `IntentionMachine` for [`newton_machine::Runtime`].
//!
//! Feature `machine`. The trait is local, so this blanket impl is legal here
//! and illegal in a downstream example crate (orphan rules: uncovered
//! `Runtime<LocalChart>`). See [[docs/adr/0014-intention-machine-is-a-port]].
//!
//! Does not fork `Msg` / `Cmd`. Associated types *are* the chart's.

use newton_machine::machine::Machine;
use newton_machine::runtime::Runtime;
use newton_machine::snapshot::Snapshot;

use crate::intention::{IntentionMachine, Step};

#[cfg_attr(docsrs, doc(cfg(feature = "machine")))]
impl<M> IntentionMachine for Runtime<M>
where
    M: Machine + Clone,
    M::Model: Clone,
    M::History: Clone,
    M::NodeId: Clone,
{
    type Msg = M::Msg;
    type Cmd = M::Cmd;
    type View = M::View;
    type Snapshot = Snapshot<M, M::Model, M::History>;
    type NodeId = M::NodeId;

    fn apply(&mut self, msg: M::Msg) -> Step<M::Cmd> {
        Step {
            cmd: Runtime::apply(self, msg),
        }
    }

    fn view(&self) -> M::View {
        Runtime::view(self)
    }

    fn snapshot(&self) -> Self::Snapshot {
        Runtime::snapshot(self)
    }

    fn in_state(&self, node: &M::NodeId) -> bool {
        Runtime::in_state(self, node.clone())
    }

    fn restore(&mut self, snapshot: Self::Snapshot) {
        Runtime::restore(self, snapshot);
    }
}

#[cfg(test)]
mod tests {
    use newton_machine::cmd::Cmd;
    use newton_machine::machine::{Boot, Machine};
    use newton_machine::runtime::Runtime;

    use super::*;

    #[derive(Clone, Debug)]
    struct Chart {
        n: u8,
    }

    #[derive(Clone, Debug, Default)]
    struct Model {
        n: u8,
    }

    impl Machine for Chart {
        type Flags = ();
        type Model = Model;
        type Msg = u8;
        type Cmd = Cmd<()>;
        type View = u8;
        type History = ();
        type NodeId = &'static str;

        fn init(_: ()) -> Boot<Self> {
            Boot::new(Chart { n: 0 }, Model::default(), (), Cmd::none())
        }

        fn update(&mut self, model: &mut Model, _: &mut (), msg: u8) -> Cmd<()> {
            self.n = msg;
            model.n = msg;
            Cmd::none()
        }

        fn view(&self, model: &Model) -> u8 {
            model.n
        }

        fn in_state(&self, id: &'static str) -> bool {
            id == "armed" && self.n > 0
        }
    }

    #[test]
    fn runtime_is_an_intention_machine() {
        let (mut rt, _) = Runtime::<Chart>::boot(());
        assert!(!IntentionMachine::in_state(&rt, &"armed"));
        let _ = IntentionMachine::apply(&mut rt, 1);
        assert!(IntentionMachine::in_state(&rt, &"armed"));
        assert_eq!(IntentionMachine::view(&rt), 1);
        let snap = IntentionMachine::snapshot(&rt);
        let _ = IntentionMachine::apply(&mut rt, 0);
        assert!(!IntentionMachine::in_state(&rt, &"armed"));
        IntentionMachine::restore(&mut rt, snap);
        assert!(IntentionMachine::in_state(&rt, &"armed"));
    }
}
