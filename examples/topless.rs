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
    WS_MINIMIZEBOX, WS_OVERLAPPEDWINDOW, WS_POPUP, WS_SIZEBOX, WS_SYSMENU, WS_VISIBLE,WS_DLGFRAME,
    WS_DISABLED,WS_GROUP,WS_HSCROLL,WS_VSCROLL,WS_OVERLAPPED,WS_TABSTOP,WS_THICKFRAME,WS_POPUPWINDOW,
    WINDOWINFO,GetWindowInfo,IsZoomed,
};
use windows_sys::Win32::Foundation::{POINT, RECT};
use windows_sys::Win32::Foundation::{BOOL, HWND, NTSTATUS, S_OK};
pub fn win_to_err(result:BOOL) -> Result<(), io::Error> {
    if result != false.into() {Ok(())
    } else                    {Err(io::Error::last_os_error())}
}
use indexmap::IndexMap;
pub fn get_ws_style_s(style_in:u32) -> String {
    let mut ws_prime = IndexMap::new();
    ws_prime.insert(WS_SIZEBOX     	, (0x___40000u32	,"BorderSize"  	.to_string(),"↔"     	.to_string()," "     	.to_string()));
    ws_prime.insert(WS_BORDER      	, (0x__800000u32	,"Border"      	.to_string(),"─"     	.to_string()," "     	.to_string()));
    ws_prime.insert(WS_DLGFRAME    	, (0x__400000u32	,"Dialog"      	.to_string(),"Dlg"   	.to_string(),"   "   	.to_string()));
    ws_prime.insert(WS_CHILD       	, (0x40000000u32	,"Child"       	.to_string(),"👶"     	.to_string()," "     	.to_string()));
    ws_prime.insert(WS_TABSTOP     	, (0x___10000u32	,"Tabstop"     	.to_string(),"⭾"     	.to_string()," "     	.to_string()));
    ws_prime.insert(WS_GROUP       	, (0x___20000u32	,"Group"       	.to_string(),"G1"    	.to_string(),"  "    	.to_string()));
    ws_prime.insert(WS_SYSMENU     	, (0x___80000u32	,"Sysmenu"     	.to_string(),"Sys"   	.to_string(),"  "    	.to_string()));
    ws_prime.insert(WS_HSCROLL     	, (0x__100000u32	,"HScroll"     	.to_string(),"←📜→"   	.to_string(),"   "   	.to_string()));
    ws_prime.insert(WS_VSCROLL     	, (0x__200000u32	,"VScroll"     	.to_string(),"↓📜↑"   	.to_string(),"   "   	.to_string()));
    ws_prime.insert(WS_MAXIMIZE    	, (0x_1000000u32	,"Maximize"    	.to_string(),"Max"   	.to_string(),"   "   	.to_string()));
    ws_prime.insert(WS_MAXIMIZEBOX 	, (0x___10000u32	,"Maximizebox" 	.to_string(),"🗖"     	.to_string()," "     	.to_string()));
    ws_prime.insert(WS_MINIMIZE    	, (0x20000000u32	,"Minimize"    	.to_string(),"Min"   	.to_string(),"   "   	.to_string()));
    ws_prime.insert(WS_MINIMIZEBOX 	, (0x___20000u32	,"Minimizebox" 	.to_string(),"🗕"     	.to_string()," "     	.to_string()));
    ws_prime.insert(WS_OVERLAPPED  	, (0x_______0u32	,"Overlapped"  	.to_string(),"Over"  	.to_string(),"    "  	.to_string()));
    ws_prime.insert(WS_POPUP       	, (0x80000000u32	,"Popup"       	.to_string(),"Popup" 	.to_string(),"     " 	.to_string()));
    ws_prime.insert(WS_CLIPCHILDREN	, (0x_2000000u32	,"ClipChildren"	.to_string(),"ClCh"  	.to_string(),"    "  	.to_string()));
    ws_prime.insert(WS_CLIPSIBLINGS	, (0x_4000000u32	,"ClipSibling" 	.to_string(),"ClSibl"	.to_string(),"      "	.to_string()));
    ws_prime.insert(WS_DISABLED    	, (0x_8000000u32	,"Disabled"    	.to_string(),"✗"     	.to_string()," "     	.to_string()));
    ws_prime.insert(WS_VISIBLE     	, (0x10000000u32	,"Visible"     	.to_string(),"👁"     	.to_string()," "     	.to_string()));
    let mut ws_combo = IndexMap::new(); //
    ws_combo.insert(WS_OVERLAPPEDWINDOW	, ((WS_OVERLAPPED|WS_CAPTION|WS_SYSMENU|WS_THICKFRAME|WS_MINIMIZEBOX|WS_MAXIMIZEBOX)	,"OverlappedW (O+T+Sys+BdSz+🗖🗕)"	.to_string()," "   	.to_string(),"    "	.to_string()));
    ws_combo.insert(WS_CAPTION         	, ((         WS_BORDER|WS_DLGFRAME)                                                 	,"Title (Bd+Dlg)"               	.to_string(),"−"   	.to_string()," "   	.to_string())); //0x__C00000
    ws_combo.insert(WS_POPUPWINDOW     	, ((WS_POPUP|WS_BORDER      |WS_SYSMENU)                                            	,"PopupWin"                     	.to_string(),"PopW"	.to_string(),"    "	.to_string()));

    let mut style = style_in;
    let mut out:String = String::new();
    for (ws, s) in ws_prime {out += " ";
        if style & ws == ws {style &= !ws; out += &s.2} else {out += &s.3}
    }
    if style != 0 {out += &format!(" ❓{:#x}",style).to_string()};
    out += &format!(" ({:#x})",style_in).to_string();
    out
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Hash)]
pub struct BdLbr(i32);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Hash)]
pub struct BdTop(i32);
pub fn get_border_resize_size(win_id:HWND) -> Result<(BdLbr,BdTop), io::Error> {
    if unsafe{IsZoomed(win_id) != 0} { // when maximized: borders are hidden, but the title bar is not
        //  // Border size = win_rect - client_rect
        //  let rect_ext   : RECT = unsafe {
        //     let mut rect: RECT = mem::zeroed();
        //     if GetWindowRect(win_id, &mut rect) == false.into() {Err(io::Error::last_os_error())}
        //     rect};
        //  let rect_int   : RECT = unsafe {
        //     let mut rect: RECT = mem::zeroed();
        //     if GetClientRect(win_id, &mut rect) == false.into() {Err(io::Error::last_os_error())}
        //     rect};
        // wndRect.Size() - clientRect.Size();
        Ok((BdLbr(0),BdTop(0)))
    } else { // border size = win_rect(with style) - win_rect(with no style)  for an empty client to work with unititialized and minimized windows
        let style    = unsafe{GetWindowLongW(win_id, GWL_STYLE  ) as u32};
        let style_ex = unsafe{GetWindowLongW(win_id, GWL_EXSTYLE) as u32};
        let style_no = style & !WS_SIZEBOX;
        let style_s    = get_ws_style_s(style);
        let style_no_s = get_ws_style_s(style_no);
        let b_menu = unsafe{GetMenu(win_id) != 0};
        let rect_style : RECT = {
           let mut rect: RECT = unsafe{mem::zeroed()};
           if unsafe{AdjustWindowRectEx(&mut rect, style   , b_menu.into(), style_ex) == false.into()} {return Err(io::Error::last_os_error())}
           rect};
        let rect_style_no : RECT = {
           let mut rect   : RECT = unsafe{mem::zeroed()};
           if unsafe{AdjustWindowRectEx(&mut rect, style_no, b_menu.into(), style_ex) == false.into()} {return Err(io::Error::last_os_error())}
           rect};
        let lbr:BdLbr = BdLbr(rect_style_no.left - rect_style.left);
        println!("  style={style_s}\nnostyle={style_no_s}");
        println!("← style={} nostyle={}",rect_style.left,rect_style_no.left);
        println!("↑ style={} nostyle={}",rect_style.top ,rect_style_no.top);
        let top:BdTop = if style & WS_CAPTION == WS_CAPTION {println!("✓caption");BdTop(0)} else {println!("✗caption");BdTop(rect_style_no.top  - rect_style.top)};
        // windows with a title bar don't have external resize border, it's part of the title bar
        Ok((lbr,top))
    }
}
pub fn get_win_info(win_id:HWND) -> Result<WINDOWINFO , io::Error> {
    // doesn't separate resize borders from others! use GetWindowRect with/out WS_THICKFRAME style
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
                            let win_id = _window_id.into_raw().try_into().unwrap();
                            if let Ok((lbr,top)) = get_border_resize_size(win_id) {
                                println!("↓←→w{} ↑h{} px resize border",lbr.0,top.0);}
                            let win_info = get_win_info(win_id).unwrap();
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
// ✗title bar   ✓resize
// cbSize=60b is_active=1 style=0x160f0000 style_ex=0x20040910 atomWindowType=50061 wCreatorVersion=1280
// ↔10 ↕10 border px
// ←0 ↑0 →820 ↓620 window
// ←10 ↑10 →810 ↓610 client

// ✗title bar   ✓resize  ✓border (WS_BORDER, thin one, added via spy)
// cbSize=60b is_active=1 style=0x168f0000 style_ex=0x20040910 atomWindowType=50051 wCreatorVersion=1280
// ↔11 ↕11 border px
// ←0 ↑0 →820 ↓620 window
// ←11 ↑11 →809 ↓609 client

// ✓title bar   ✓resize
// cbSize=60b is_active=1 style=0x16cf0000 style_ex=0x20040910 atomWindowType=49989 wCreatorVersion=1280
// style diff = 0xC00000
// ↔11¦11 border px
// ←0 ↑0 →820 ↓620 window
// ←11 ↑45 →809 ↓609 client

// ✓title bar   ✗resize
// cbSize=60b is_active=1 style=0x16cb0000 style_ex=0x20040910 atomWindowType=50062 wCreatorVersion=1280
// ↔11 ↕11 border px
// ←0 ↑0 →820 ↓620 window
// ←11 ↑45 →809 ↓609 client

// ✗title bar   ✗resize
// cbSize=60b is_active=1 style=0x160b0000 style_ex=0x20040810 atomWindowType=50062 wCreatorVersion=1280
// ↔0 ↕0 border px
// ←0 ↑0 →820 ↓620 window
// ←0 ↑0 →820 ↓620 client

                            println!(
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
