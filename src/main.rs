#[macro_use]
extern crate spectra;

mod kit;

use spectra::render::framebuffer::Framebuffer2D;
use spectra::render::pipeline::{RenderState, pipeline};
use spectra::render::shader::program::{Program, ProgramKey};
use spectra::luminance::tess::{Mode, Tess};
use spectra::sys::bootstrap::{Device, WindowOpt};
use spectra::sys::event::{Action, Key, MouseButton, WindowEvent};
use spectra::sys::res::{Res, Store, StoreOpt};
use std::sync::mpsc::{Receiver, Sender};

use kit::loop_ctrl::LoopControl;
use kit::runner;
use kit::view::View;

struct V {
  store: Store<()>,
  dev: Device,
  screen: Framebuffer2D<(), ()>,
  triangle: Tess<()>,
  triangle_offset: f32
}

enum Cmd {
  Quit,
  RenderTriangle(f32)
}

enum Msg {
  Event(WindowEvent),
}

uniform_interface! {
  struct Uniforms {
    #[unbound]
    offset: f32
  }
}

impl View for V {
  type Cmd = Cmd; 
  type Msg = Msg;

  fn run(&mut self, cmds: &Receiver<Self::Cmd>, com: &Sender<Self::Msg>) -> LoopControl {
    for cmd in cmds.try_iter() {
      match cmd {
        Cmd::Quit => return LoopControl::Break,
        Cmd::RenderTriangle(offset) => self.triangle_offset = offset,
      }
    }

    for event in self.dev.events() {
      let _ = com.send(Msg::Event(event));
    }

    let ctx = &mut ();
    self.store.sync(ctx);

    let program: Res<Program<(), (), Uniforms>> = self.store.get(&ProgramKey::new("shaders.main"), ctx).expect("main shader");

    pipeline(&self.screen, [0., 0., 0., 1.], |shd_gt| {
      shd_gt.shade(&program.borrow(), |rdr_gt, uniforms| {
        uniforms.offset.update(self.triangle_offset);

        rdr_gt.render(RenderState::default(), |tess_gt| {
          tess_gt.render((&self.triangle).into());
        });
      });
    });

    self.dev.step(|_| {
    });

    LoopControl::Continue
  }
}

fn main() {
  let view_store = Store::new(StoreOpt::default().set_root("data")).expect("view store");
  let dev = bootstrap!(800, 600, WindowOpt::default()).expect("device");
  let screen = Framebuffer2D::default([800, 600]);
  let view = V {
    store: view_store,
    dev,
    screen,
    triangle: Tess::attributeless(Mode::Triangle, 3),
    triangle_offset: 0.
  };

  let mut mouse = [0., 0.];
  let mut viewport: [u32; 2] = [800, 600];

  runner::run(view, move |cmds, msgs| {
    for msg in msgs.iter() {
      match msg {
        Msg::Event(WindowEvent::Close) | Msg::Event(WindowEvent::Key(Key::Escape, _, Action::Press, _)) => {
          let _ = cmds.send(Cmd::Quit);
          return LoopControl::Break;
        }

        Msg::Event(WindowEvent::FramebufferSize(w, h)) => {
          viewport = [w as u32, h as u32];
        }

        Msg::Event(WindowEvent::CursorPos(x, y)) => {
          mouse = [x, y];
        }

        Msg::Event(WindowEvent::MouseButton(MouseButton::Button1, Action::Release, _)) => {
          let offset = mouse[0] as f32 / viewport[0] as f32;
          println!("send offset={}", offset);
          cmds.send(Cmd::RenderTriangle(offset));
        }

        _ => ()
      }
    }

    LoopControl::Continue
  });
}
