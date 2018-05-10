use std::sync::mpsc;
use std::thread;

/// An event loop is responsible in serving events to the rest of the application.
pub trait EventLoop: 'static + Send {
  type Event: Send;

  /// Poll an event.
  ///
  /// It is recommended that this function, when it has nothing to do, block the execution so that
  /// the event thread doesn’t spin / busywait.
  fn poll(&mut self) -> Self::Event;
}


pub(crate) fn start_event_loop<EL>(
  mut el: EL,
) -> Events<EL::Event>
where EL: EventLoop {
  let (sx, rx) = mpsc::channel();

  thread::spawn(move || {
    loop {
      let event = el.poll();
      let _ = sx.send(event);
    }
  });

  Events(rx)
}

