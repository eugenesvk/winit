//! Simple winit window example.

use std::error::Error;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
#[cfg(web_platform)]
use winit::platform::web::WindowAttributesWeb;
use winit::window::{Window, WindowAttributes, WindowId};

#[path = "util/fill.rs"]
mod fill;
#[path = "util/tracing.rs"]
mod tracing;

#[derive(Default, Debug)]
struct App {
    window: Option<Box<dyn Window>>,
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
  if state.contains(ModifiersState::CONTROL){s.push('⎈')}else{s.push(' ')};
  if state.contains(ModifiersState::META   ){s.push('◆')}else{s.push(' ')};
  if state.contains(ModifiersState::ALT    ){s.push('⎇')}else{s.push(' ')};
  s
}
pub fn mod_state_phys_s(mods:&Modifiers) -> String {
  let mut s = String::new();
  if let ModifiersKeyState::Pressed = mods.lshift_state()     {s.push_str("‹⇧ ")}else{s.push_str("  ")};
  if let ModifiersKeyState::Pressed = mods.rshift_state()     {s.push_str(" ⇧›")}else{s.push_str("  ")};
  if let ModifiersKeyState::Pressed = mods.lcontrol_state()   {s.push_str("‹⎈ ")}else{s.push_str("  ")};
  if let ModifiersKeyState::Pressed = mods.rcontrol_state()   {s.push_str(" ⎈›")}else{s.push_str("  ")};
  if let ModifiersKeyState::Pressed = mods.lsuper_state()     {s.push_str("‹◆ ")}else{s.push_str("  ")};
  if let ModifiersKeyState::Pressed = mods.rsuper_state()     {s.push_str(" ◆›")}else{s.push_str("  ")};
  if let ModifiersKeyState::Pressed = mods.lalt_state()       {s.push_str("‹⎇ ")}else{s.push_str("  ")};
  if let ModifiersKeyState::Pressed = mods.ralt_state()       {s.push_str(" ⎇›")}else{s.push_str("  ")};
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
    ElementState::Pressed   => {s.push('↓')},
    ElementState::Released  => {s.push('↑')},
  }
  if key.repeat {s.push('🔁')}else{s.push(' ')}; //𜱣⚛
  if let PhysicalKey    ::Code        (key_code         ) = &key.physical_key   {s.push_str(&format!("{:?} "    ,key_code           ))};
  if let PhysicalKey    ::Unidentified(key_code_native  ) = &key.physical_key   {s.push_str(&format!("�{:?} "   ,key_code_native    ))};
  if let Key            ::Named       (key_named        ) = &key.logical_key    {s.push_str(&format!("{:?} "    ,key_named          ))};
  if let Key            ::Character   (key_char         ) = &key.logical_key    {s.push_str(&format!("{} "      ,key_char           ))};
  if let Key            ::Unidentified(key_native       ) = &key.logical_key    {s.push_str(&format!("�{:?} "   ,key_native         ))};
  if let Key            ::Dead        (maybe_char       ) = &key.logical_key    {s.push_str(&format!("🕱{:?} "   ,maybe_char         ))};
  match &key.location {
    KeyLocation::Standard   => {s.push('≝')},
    KeyLocation::Left       => {s.push('←')},
    KeyLocation::Right      => {s.push('→')},
    KeyLocation::Numpad     => {s.push('🔢')},
  }
  s
}

impl ApplicationHandler for App {
    fn can_create_surfaces(&mut self, event_loop: &dyn ActiveEventLoop) {
        #[cfg(not(web_platform))]
        let window_attributes = WindowAttributes::default();
        #[cfg(web_platform)]
        let window_attributes = WindowAttributes::default()
            .with_platform_attributes(Box::new(WindowAttributesWeb::default().with_append(true)));
        self.window = match event_loop.create_window(window_attributes) {
            Ok(window) => Some(window),
            Err(err) => {
                eprintln!("error creating window: {err}");
                event_loop.exit();
                return;
            },
        }
    }

    fn window_event(&mut self, event_loop: &dyn ActiveEventLoop, _: WindowId, event: WindowEvent) {
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
            },
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::SurfaceResized(_) => {
                self.window.as_ref().expect("resize event without a window").request_redraw();
            },
            WindowEvent::RedrawRequested => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                let window = self.window.as_ref().expect("redraw request without a window");

                // Notify that you're about to draw.
                window.pre_present_notify();

                // Draw.
                fill::fill_window(window.as_ref());

                // For contiguous redraw loop you can request a redraw from here.
                // window.request_redraw();
            },
            _ => (),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(web_platform)]
    console_error_panic_hook::set_once();

    tracing::init();

    let event_loop = EventLoop::new()?;

    // For alternative loop run options see `pump_events` and `run_on_demand` examples.
    event_loop.run_app(App::default())?;

    Ok(())
}
