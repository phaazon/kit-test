use std::sync::mpsc::{Receiver, Sender};

use kit::loop_ctrl::LoopControl;

pub trait View {
  type Cmd: 'static + Send;
  type Msg: 'static + Send;

  fn run(
    &mut self,
    cmds: &Receiver<Self::Cmd>,
    msgs: &Sender<Self::Msg>,
  ) -> LoopControl;
}
