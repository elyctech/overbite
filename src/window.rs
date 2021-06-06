use winit::{
  event::{Event, WindowEvent},
  event_loop::{ControlFlow},
  window::{WindowBuilder},
};

pub struct Window {
  event_loop: winit::event_loop::EventLoop<()>,
  _window: winit::window::Window,
}

impl Window {
  // Associated functions

  pub fn new(title: &str, width: u32, height: u32, resizeable: bool) -> Window {
    let event_loop = winit::event_loop::EventLoop::new();
    let window = WindowBuilder::new()
      .with_inner_size(winit::dpi::LogicalSize::new(width, height))
      .with_resizable(resizeable)
      .with_title(title)
      .build(&event_loop)
      .expect("failed to create window!");

    Window {
      event_loop,
      _window: window
    }
  }

  // Methods

  pub fn open(self) {
    self.event_loop.run(move |event, _, control_flow| {
      *control_flow = ControlFlow::Wait;

      match event {
          Event::WindowEvent {
              event: WindowEvent::CloseRequested,
              ..
          } => *control_flow = ControlFlow::Exit,
          _ => (),
      }
    });
  }
}
