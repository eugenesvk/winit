#![allow(
    unused_imports,
    unused_mut,
    unused_variables,
    dead_code,
    unused_assignments,
    unused_macros
)]
use std::error::Error;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};
use winit_core::keyboard::NamedKey;

#[path = "util/fill.rs"]
mod fill;
#[path = "util/tracing.rs"]
mod tracing;

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
struct Application {
    window: Option<Box<dyn Window>>,
}

impl Application {
    fn new() -> Self {
        Self { window: None }
    }
}

use winit::event::{Modifiers, KeyEvent};
// pub struct Modifiers { // CORE: Describes keyboard modifiers event.
  //   state       : ModifiersState,
  //   pressed_mods: ModifiersKeys , // NOTE: Currently pressed modifiers keys. The field providing a metadata, it shouldn't be used as a source of truth.
  // impl Modifiers {
  //   fn lshift_state(&self) -> ModifiersKeyState { /// The state of the left shift key
  //     self.mod_state(ModifiersKeys::LSHIFT)}
pub fn mod_state_logic_s(state:&ModifiersState) -> String {
  let mut s = String::new();
  if state.contains(ModifiersState::SHIFT  ){s.push('⇧')}else{s.push(' ')};
  if state.contains(ModifiersState::SUPER  ){s.push('◆')}else{s.push(' ')};
  if state.contains(ModifiersState::CONTROL){s.push('⎈')}else{s.push(' ')};
  if state.contains(ModifiersState::ALT    ){s.push('⎇')}else{s.push(' ')};
  s
}
pub fn mod_state_phys_s(mods:&Modifiers) -> String {
  let mut s = String::new();
  if let ModifiersKeyState::Pressed = mods.lshift_state()     {s.push_str("‹⇧ ")}else{s.push_str("  ")};
  if let ModifiersKeyState::Pressed = mods.rshift_state()     {s.push_str(" ⇧›")}else{s.push_str("  ")};
  if let ModifiersKeyState::Pressed = mods.lalt_state()       {s.push_str("‹◆ ")}else{s.push_str("  ")};
  if let ModifiersKeyState::Pressed = mods.ralt_state()       {s.push_str(" ◆›")}else{s.push_str("  ")};
  if let ModifiersKeyState::Pressed = mods.lcontrol_state()   {s.push_str("‹⎇ ")}else{s.push_str("  ")};
  if let ModifiersKeyState::Pressed = mods.rcontrol_state()   {s.push_str(" ⎇›")}else{s.push_str("  ")};
  if let ModifiersKeyState::Pressed = mods.lsuper_state()     {s.push_str("‹⎈ ")}else{s.push_str("  ")};
  if let ModifiersKeyState::Pressed = mods.rsuper_state()     {s.push_str(" ⎈›")}else{s.push_str("  ")};
  s
}
  // pub struct KeyEvent {
    // pub physical_key: PhysicalKey, enum PhysicalKey
      // Code(KeyCode)
      // �Unidentified(NativeKeyCode),
    // pub logical_key: Key,// pub enum Key<Str = SmolStr> {
      // Named(NamedKey),
      // Character(Str),
      // �Unidentified(NativeKey),
      // 🕱Dead(Option<char>),
    // pub text: Option<SmolStr>,
    // pub location: KeyLocation, pub enum KeyLocation { Standard,Left,Right,Numpad,
    // pub state: ElementState, pressed/released
    // pub repeat: bool, 🔁
use winit::keyboard::{PhysicalKey, Key, ModifiersState, ModifiersKeyState, KeyLocation};
use winit::event::ElementState;
pub fn ev_key_s(key:&KeyEvent) -> String {
  let mut s = String::new();
  match &key.state {
    ElementState::Pressed 	=> {s.push('↓')},
    ElementState::Released	=> {s.push('↑')},
  }
  if key.repeat {s.push('🔁')}else{s.push(' ')}; //𜱣⚛
  if let PhysicalKey	::Code        (key_code       	) = &key.physical_key	{s.push_str(&format!("{:?} " 	,key_code       	))};
  if let PhysicalKey	::Unidentified(key_code_native	) = &key.physical_key	{s.push_str(&format!("�{:?} "	,key_code_native	))};
  if let Key        	::Named       (key_named      	) = &key.logical_key 	{s.push_str(&format!("{:?} "  	,key_named      	))};
  if let Key        	::Character   (key_char       	) = &key.logical_key 	{s.push_str(&format!("{} "    	,key_char       	))};
  if let Key        	::Unidentified(key_native     	) = &key.logical_key 	{s.push_str(&format!("�{:?} " 	,key_native     	))};
  if let Key        	::Dead        (maybe_char     	) = &key.logical_key 	{s.push_str(&format!("🕱{:?} " 	,maybe_char     	))};
  match &key.location {
    KeyLocation::Standard	=> {s.push('≝')},
    KeyLocation::Left    	=> {s.push('←')},
    KeyLocation::Right   	=> {s.push('→')},
    KeyLocation::Numpad  	=> {s.push('🔢')},
  }
  s
}

#[cfg(windows_platform)]
use winit::platform::windows::WindowAttributesWindows;
#[cfg(windows_platform)]
use winit::platform::windows::WindowExtWindows;
#[cfg(windows_platform)]
impl ApplicationHandler for Application {
    fn can_create_surfaces(&mut self, event_loop: &dyn ActiveEventLoop) {
        let window_attributes_win = Box::new(
            WindowAttributesWindows::default()
                .with_titlebar(false) //         titlebar          ≝true
                .with_top_resize_border(false), // top_resize_border ≝true
        );
        let window_attributes = WindowAttributes::default()
            .with_title("Topless (unless you see this)!")
            .with_decorations(true) //       decorations       ≝true
            .with_resizable(true) //         resizable         ≝true
            .with_platform_attributes(window_attributes_win) //
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
            WindowEvent::ModifiersChanged(mods) => {
                let state       = mods.state() ;//ModifiersState;
                // let pressed_mods= mods.ModifiersKeys ; // NOTE: Currently pressed modifiers keys. The field providing a metadata, it shouldn't be used as a source of truth.
                let state_s = mod_state_logic_s(&state);
                let pressed_mods_s = mod_state_phys_s(&mods);
                println!("Δ\t{}\n\t{} phys↓",state_s, pressed_mods_s);
                // ModifiersChanged(Modifiers { state: ModifiersState(ALT), pressed_mods: ModifiersKeys(0x0) })
                // window.modifiers = modifiers.state();
                // info!("Modifiers changed to {:?}", window.modifiers);
            },
            WindowEvent::KeyboardInput { event, is_synthetic, .. } => {
                let key_event_s = ev_key_s(&event);
                let is_synthetic_s = if is_synthetic{"⚗"}else{" "};
                println!("{}{}",is_synthetic_s,key_event_s);
                if event.state == ElementState::Pressed && !event.repeat {
                    match event.key_without_modifiers.as_ref() {
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
                        Key::Named(NamedKey::Escape) => {
                            event_loop.exit();
                        },
                        _ => (),
                    }
                }
            },
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
