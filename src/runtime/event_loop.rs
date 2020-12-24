// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! An event loop for platform and window events.

use super::window;

use crate::{
  prelude::*,
  sync::{blocking::Mutex, OnceCell, Request},
};

use winit::{
  event::Event as WinitEvent,
  event_loop::{EventLoop, EventLoopProxy},
  window::{Window, WindowBuilder},
};

/// One of the possible commands that can be sent to the event loop with
/// [`send()`].
pub enum Command {
  /// Creates a window.
  #[cfg(feature = "window")]
  CreateWindow(window::Options, Request<Result<Window>>),
  /// Stops the event loop.
  Stop(Result),
}

/// A proxy for sending events to the loop.
static PROXY: OnceCell<Mutex<EventLoopProxy<Command>>> = OnceCell::new();

/// Send a [`Command`] to the event loop.
pub fn send(event: Command) {
  if PROXY.get().expect("event loop is not started").lock().send_event(event).is_err() {
    panic!("event loop has stopped");
  }
}

/// Spawns the given future as a task and then runs an event loop on the current
/// thread until the future completes.
pub fn run(future: impl Future<Output = Result> + Send + 'static) -> ! {
  let event_loop = EventLoop::with_user_event();

  PROXY.set(Mutex::new(event_loop.create_proxy())).ok().expect("event loop is already running");

  // Spawn the main task and send its result back to the event loop when it
  // completes.

  task::start_detached(async move {
    let result = future.await;

    trace!("Main task finished. Stopping event loop…");

    send(Command::Stop(result));
  });

  // Run the event loop until the task completes.

  let mut result = Err(fail::err!("The event loop terminated early."));

  event_loop.run(move |event, event_loop, flow| {
    use winit::event_loop::ControlFlow;

    *flow = ControlFlow::Wait;

    match event {
      // Occurs when a request comes in to create a new window.
      #[cfg(feature = "window")]
      WinitEvent::UserEvent(Command::CreateWindow(options, req)) => {
        let window = WindowBuilder::new()
          .with_resizable(options.is_resizable)
          .with_inner_size(winit::dpi::PhysicalSize::new(options.size.x, options.size.y))
          .with_title(options.title)
          .build(&event_loop)
          .map_err(fail::Error::from);

        req.resolve(window);
      }

      // Occurs when the spawned task completes.
      WinitEvent::UserEvent(Command::Stop(output)) => {
        *flow = ControlFlow::Exit;
        result = output;
      }

      // Occurs just before the loop (and thus the whole process) exits.
      WinitEvent::LoopDestroyed => {
        if let Err(err) = result.clone() {
          let _ = writeln!(console::Term::stderr(), "{:#}", err);

          std::process::exit(1);
        }
      }

      _ => {}
    }
  })
}
