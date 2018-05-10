use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;

use kit::loop_ctrl::LoopControl;
use kit::view::View;

pub fn run<V, F>(mut view: V, mut f: F)
where
  V: View,
  F: 'static + Send + FnMut(&Sender<V::Cmd>, &Receiver<V::Msg>) -> LoopControl {
  let (cmds_sdr, cmds_rcvr) = channel();
  let (msgs_sdr, msgs_rcvr) = channel();

  thread::spawn(move || {
    'lo: loop {
      let ctrl = f(&cmds_sdr, &msgs_rcvr);

      if ctrl == LoopControl::Break {
        break 'lo;
      }
    }
  });

  'lo: loop {
    let ctrl = view.run(&cmds_rcvr, &msgs_sdr);

    if ctrl == LoopControl::Break {
      break 'lo;
    }
  }
}
