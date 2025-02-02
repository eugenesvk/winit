#![allow(
    unused_imports,
    unused_mut,
    unused_variables,
    dead_code,
    unused_assignments,
    unused_macros,
    non_snake_case,
)]
use std::mem;
use std::io;
use std::error::Error;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

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
      + resize border@↓←→ (WS_SIZEBOX) via with_resizable        (true ) ≝
      − resize border@↑                via with_top_resize_border(false)
        ├ not a separate WS_ window style, 'manual' removal on NonClientArea events
        └ only implemented for windows without a title bar, eg, with a custom title bar handling \
         resizing from the top
    ——————————————————————————————
    Press a key for (un)setting/querying a specific parameter (modifiers are ignored):
                         on  off  toggle  query
    title bar            q    w     e       r
    resize border@↓←→    a    s     d       f
    resize border@↑      z    x     c       v
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

use windows_sys::Win32::UI::WindowsAndMessaging::{
    AdjustWindowRectEx, EnableMenuItem, GetMenu, GetSystemMenu, GetWindowLongW, SendMessageW,
    SetWindowLongW, SetWindowPos, ShowWindow, GWL_EXSTYLE, GWL_STYLE, HWND_BOTTOM, HWND_NOTOPMOST,
    HWND_TOPMOST, MF_BYCOMMAND, MF_DISABLED, MF_ENABLED, SC_CLOSE, SWP_ASYNCWINDOWPOS,
    SWP_FRAMECHANGED, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOREPOSITION, SWP_NOSIZE, SWP_NOZORDER,
    SW_HIDE, SW_MAXIMIZE, SW_MINIMIZE, SW_RESTORE, SW_SHOW, SW_SHOWNOACTIVATE, WINDOWPLACEMENT,
    WINDOW_EX_STYLE, WINDOW_STYLE, WS_BORDER, WS_CAPTION, WS_CHILD, WS_CLIPCHILDREN,
    WS_CLIPSIBLINGS, WS_EX_ACCEPTFILES, WS_EX_APPWINDOW, WS_EX_LAYERED, WS_EX_NOREDIRECTIONBITMAP,
    WS_EX_TOPMOST, WS_EX_TRANSPARENT, WS_EX_WINDOWEDGE, WS_MAXIMIZE, WS_MAXIMIZEBOX, WS_MINIMIZE,
    WS_MINIMIZEBOX, WS_OVERLAPPEDWINDOW, WS_POPUP, WS_SIZEBOX, WS_SYSMENU, WS_VISIBLE,
    WINDOWINFO,GetWindowInfo,
};
use windows_sys::Win32::Foundation::{POINT, RECT};
use windows_sys::Win32::Foundation::{BOOL, HWND, NTSTATUS, S_OK};
pub fn win_to_err(result:BOOL) -> Result<(), io::Error> {
    if result != false.into() {Ok(())
    } else                    {Err(io::Error::last_os_error())}
}
// pub fn get_win_info(win_id:HWND, mut win_info:PWINDOWINFO) -> Result<PWINDOWINFO , io::Error> {unsafe {
pub fn get_border_resize_size() -> Result<i32, io::Error> {
    // if unsafe{IsZoomed(win_id) != 0} {Ok(0) // when maximized: resize borders are hidden outside screen
    // } else { // border size = win_rect(with style) - win_rect(with no style)  for an empty client to work with unititialized and minimized windows
    // border size = win_rect(with style) - win_rect(with no style)  for an empty client to work with unititialized and minimized windows
    let style    = WS_CAPTION | WS_BORDER | WS_CLIPSIBLINGS | WS_SYSMENU; // Required styles to properly support common window functionality like aero snap
    let style_no =              WS_BORDER | WS_CLIPSIBLINGS | WS_SYSMENU;
    let style_ex = WS_EX_WINDOWEDGE | WS_EX_ACCEPTFILES;
    let rect_style : RECT = {
       let mut rect: RECT = unsafe{mem::zeroed()};
       if unsafe{AdjustWindowRectEx(&mut rect, style   , FALSE, style_ex) == false.into()} {return Err(io::Error::last_os_error())}
       rect};
    let rect_style_no : RECT = {
       let mut rect   : RECT = unsafe{mem::zeroed()};
       if unsafe{AdjustWindowRectEx(&mut rect, style_no, FALSE, style_ex) == false.into()} {return Err(io::Error::last_os_error())}
       rect};
    // let lbr:BdLbr = BdLbr(rect_style_no.left - rect_style.left);
    let lbr = rect_style_no.left - rect_style.left;
    println!("← style={} nostyle={}",rect_style.left,rect_style_no.left);
    // println!("↑ style={} nostyle={}",rect_style.top ,rect_style_no.top);
    // let top:BdTop = if style & WS_CAPTION == WS_CAPTION {println!("✓caption");BdTop(0)} else {println!("✗caption");BdTop(rect_style_no.top  - rect_style.top)};
    // windows with a title bar don't have external resize border, it's part of the title bar
    Ok(lbr)
}
pub fn get_win_info(win_id:HWND) -> Result<WINDOWINFO , io::Error> {
    // let rect: RECT = unsafe {
    //     let mut rect: RECT = mem::zeroed();
    //     if GetClientRect(window, &mut rect) == false.into() {
    //         return PointerMoveKind::None; // exit early if GetClientRect failed
    //     }
    //     rect
    // };
    let mut win_info: WINDOWINFO = unsafe{mem::zeroed()};
    win_info.cbSize = mem::size_of::<WINDOWINFO>() as u32; // must set cbSize member to sizeof(WINDOWINFO) before calling GetWindowInfo
    win_to_err(unsafe{GetWindowInfo(win_id, &mut win_info)})?;
    Ok(win_info)
}

use winit::event::ElementState;
use winit::keyboard::{Key, ModifiersState};
#[cfg(windows_platform)]
use winit::platform::modifier_supplement::KeyEventExtModifierSupplement;
#[cfg(windows_platform)]
use winit::platform::windows::WindowAttributesExtWindows;
#[cfg(windows_platform)]
use winit::platform::windows::WindowExtWindows;
#[cfg(windows_platform)]
impl ApplicationHandler for Application {
    fn can_create_surfaces(&mut self, event_loop: &dyn ActiveEventLoop) {
        let x = 0; let y = 0;
        println!("initial position (physical): {} {}",x,y);
        let window_attributes = WindowAttributes::default()
            .with_title("Topless (unless you see this)!")
            // .with_decorations(true) //       decorations       ≝true
            .with_titlebar(false) //         titlebar          ≝true
            .with_resizable(true) //         resizable         ≝true
            // .with_top_resize_border(false) // top_resize_border ≝true
            // .with_position(dpi::Position::Logical(dpi::LogicalPosition::new(0.0, -7.0)));
            .with_position(dpi::Position::Physical(dpi::PhysicalPosition::new(x, y)))
            ;
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
                        Key::Character("5") => {
                            let win_info = get_win_info(_window_id.into_raw().try_into().unwrap()).unwrap();
                            let cbSize         :u32             = win_info.cbSize; //size of the structure, in bytes
                            let rcWindow       :RECT            = win_info.rcWindow; //coordinates of the window
                            let rcClient       :RECT            = win_info.rcClient; //coordinates of the client area (left top right bottom)
                            let dwStyle        :WINDOW_STYLE    = win_info.dwStyle; //
                            let dwExStyle      :WINDOW_EX_STYLE = win_info.dwExStyle; //
                            let is_active :u32             = win_info.dwWindowStatus; //window status. If this member is WS_ACTIVECAPTION (0x0001), the window is active. Otherwise, this member is zero
                            let cxWindowBorders:u32             = win_info.cxWindowBorders; //width of the window border, in pixels
                            let cyWindowBorders:u32             = win_info.cyWindowBorders; //height of the window border, in pixels
                            let atomWindowType :u16             = win_info.atomWindowType; //window class atom
                            let wCreatorVersion:u16             = win_info.wCreatorVersion; //Windows version of the application that created the window
                            println!("cbSize={cbSize}b is_active={is_active} style={dwStyle:#x} style_ex={dwExStyle:#x} atomWindowType={atomWindowType} wCreatorVersion={wCreatorVersion}");
                            println!("↔{cxWindowBorders} ↕{cyWindowBorders} border px");
                            println!("←{} ↑{} →{} ↓{} window",rcWindow.left,rcWindow.top,rcWindow.right,rcWindow.bottom);
                            println!("←{} ↑{} →{} ↓{} client",rcClient.left,rcClient.top,rcClient.right,rcClient.bottom);
                            "win pos outer{:?}\nwin pos surf {:?}",win.outer_position().unwrap(),win.surface_position(),);},
                        // Key::Character("i") => {println!("win pos \ninner{:?}\nouter{:?}\nsurf {:?}",win.inner_position().unwrap(),win.outer_position().unwrap(),win.surface_position(),);},
                        // Key::Character("1") => {win.set_inner_position(dpi::Position::Physical(dpi::PhysicalPosition::new( 0,0),));info!("set inner position to  0,0")},
                        // Key::Character("2") => {win.set_inner_position(dpi::Position::Physical(dpi::PhysicalPosition::new(50,0),));info!("set inner position to 50,0")},
                        Key::Character("1") => {win.set_outer_position(dpi::Position::Physical(dpi::PhysicalPosition::new( 0,0),));info!("set outer position to  0,0")},
                        Key::Character("2") => {win.set_outer_position(dpi::Position::Physical(dpi::PhysicalPosition::new( -9, -9),));info!("set outer position to - 9,- 9")},
                        Key::Character("3") => {win.set_outer_position(dpi::Position::Physical(dpi::PhysicalPosition::new(-10,-10),));info!("set outer position to -10,-10")},
                        Key::Character("4") => {win.set_outer_position(dpi::Position::Physical(dpi::PhysicalPosition::new(50,0),));info!("set outer position to 50,0")},

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
