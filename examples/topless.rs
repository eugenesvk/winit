#[allow(
    unused_imports,
    unused_mut,
    unused_variables,
    dead_code,
    unused_assignments,
    unused_macros
)]
use std::error::Error;

#[cfg(windows_platform)]
use winit::application::ApplicationHandler;
#[cfg(windows_platform)]
use winit::event::WindowEvent;
#[cfg(windows_platform)]
use winit::event_loop::{ActiveEventLoop, EventLoop};
#[cfg(windows_platform)]
use winit::window::{Window, WindowAttributes, WindowId};

#[cfg(windows_platform)]
#[path = "util/fill.rs"]
mod fill;
#[cfg(windows_platform)]
#[path = "util/tracing.rs"]
mod tracing;

#[cfg(windows_platform)]
use ::tracing::info;
#[cfg(windows_platform)]
fn main() -> Result<(), Box<dyn Error>> {
    tracing::init();

    println!(
        "Topless mode (Windows only):
      − title bar         (WS_CAPTION) via with_titlebar         (false)
      + resize border     (WS_SIZEBOX) via with_resizable        (true ) ≝
      − top resize border              via with_top_resize_border(false)
        ├ not a separate WS_ window style, 'manual' removal on NonClientArea events
        └ only implemented for windows without a title bar, eg, with a custom title bar handling \
         resizing from the top
    ——————————————————————————————
    Press a key for (un)setting/querying a specific parameter (modifiers are ignored):
                         on  off  toggle  status
    title bar            q    w     e        r
    resize border        a    s     d        f
    ─ top resize border  z    x     c        v
    "
    );

    let event_loop = EventLoop::new()?;

    let app = Application::new();
    Ok(event_loop.run_app(app)?)
}

/// Application state and event handling.
#[cfg(windows_platform)]
struct Application {
    window: Option<Box<dyn Window>>,
}

#[cfg(windows_platform)]
impl Application {
    fn new() -> Self {
        Self { window: None }
    }
}

#[cfg(windows_platform)]
use winit::event::ElementState;
#[cfg(windows_platform)]
use winit::keyboard::Key;
#[cfg(windows_platform)]
use winit::keyboard::ModifiersState;
#[cfg(windows_platform)]
use winit::platform::modifier_supplement::KeyEventExtModifierSupplement;
#[cfg(windows_platform)]
use winit::platform::windows::WindowAttributesExtWindows;
#[cfg(windows_platform)]
use winit::platform::windows::WindowExtWindows;
#[cfg(windows_platform)]
impl ApplicationHandler for Application {
    fn can_create_surfaces(&mut self, event_loop: &dyn ActiveEventLoop) {
        let window_attributes = WindowAttributes::default()
            .with_title("Topless (unless you see this)!")
            .with_decorations(true) //       decorations       ≝true
            .with_titlebar(false) //         titlebar          ≝true
            .with_resizable(true) //         resizable         ≝true
            .with_top_resize_border(false) // top_resize_border ≝true
            .with_position(dpi::Position::Logical(dpi::LogicalPosition::new(0.0, 0.0)));
        self.window = Some(event_loop.create_window(window_attributes).unwrap());
    }

    fn window_event(
        &mut self,
        event_loop: &dyn ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let win = match self.window.as_ref() {
            Some(win) => win,
            None => return,
        };
        let _modi = ModifiersState::default();
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == ElementState::Pressed && !event.repeat {
                    match event.key_without_modifiers().as_ref() {
                        Key::Character("q") => {
                            win.set_titlebar(true);
                            info!("set_titlebar         → true")
                        },
                        Key::Character("w") => {
                            win.set_titlebar(false);
                            info!("set_titlebar         → false")
                        },
                        Key::Character("e") => {
                            let flip = !win.is_titlebar();
                            win.set_titlebar(flip);
                            info!("set_titlebar         → {flip}")
                        },
                        Key::Character("r") => {
                            let is = win.is_titlebar();
                            info!("is_titlebar          = {is}")
                        },
                        Key::Character("a") => {
                            win.set_resizable(true);
                            info!("set_resizable        → true")
                        },
                        Key::Character("s") => {
                            win.set_resizable(false);
                            info!("set_resizable        → false")
                        },
                        Key::Character("d") => {
                            let flip = !win.is_resizable();
                            win.set_resizable(flip);
                            info!("set_resizable        → {flip}")
                        },
                        Key::Character("f") => {
                            let is = win.is_resizable();
                            info!("is_resizable         = {is}")
                        },
                        Key::Character("z") => {
                            win.set_top_resize_border(true);
                            info!("set_top_resize_border→ true")
                        },
                        Key::Character("x") => {
                            win.set_top_resize_border(false);
                            info!("set_top_resize_border→ false")
                        },
                        Key::Character("c") => {
                            let flip = !win.is_top_resize_border();
                            win.set_top_resize_border(flip);
                            info!("set_top_resize_border→ {flip}")
                        },
                        Key::Character("v") => {
                            let is = win.is_top_resize_border();
                            info!("is_top_resize_border = {is}")
                        },
                        _ => (),
                    }
                }
            },

            // WindowEvent::KeyboardInput {
            //     event: KeyEvent { logical_key: key, state: ElementState::Pressed, .. },
            //     ..
            // } => match key.as_ref() {
            //     // WARNING: Consider using `key_without_modifiers()` if available on your
            // platform.     // See the `key_binding` example
            //     Key::Character("1") => {
            //         self.mode = Mode::Wait;
            //         warn!("mode: {:?}", self.mode);
            //     },
            //     Key::Character("2") => {
            //         self.mode = Mode::WaitUntil;
            //         warn!("mode: {:?}", self.mode);
            //     },
            //     Key::Character("3") => {
            //         self.mode = Mode::Poll;
            //         warn!("mode: {:?}", self.mode);
            //     },
            //     Key::Character("r") => {
            //         self.request_redraw = !self.request_redraw;
            //         warn!("request_redraw: {}", self.request_redraw);
            //     },
            //     Key::Named(NamedKey::Escape) => {
            //         self.close_requested = true;
            //     },
            //     _ => (),
            // },
            WindowEvent::RedrawRequested => {
                let window = self.window.as_ref().unwrap();
                window.pre_present_notify();
                fill::fill_window(window.as_ref());
            },
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            _ => {},
        }
    }
}

#[cfg(not(windows))]
fn main() -> Result<(), Box<dyn Error>> {
    println!("This example is only supported on Windows.");
    Ok(())
}
