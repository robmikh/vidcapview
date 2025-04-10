use std::sync::Once;

use windows::{
    Graphics::SizeInt32,
    UI::Composition::{Compositor, Desktop::DesktopWindowTarget},
    Win32::{
        Foundation::{HWND, LPARAM, LRESULT, RECT, WPARAM},
        Graphics::Gdi::{
            GetMonitorInfoW, MONITOR_DEFAULTTONEAREST, MONITORINFO, MonitorFromWindow,
        },
        System::{LibraryLoader::GetModuleHandleW, WinRT::Composition::ICompositorDesktopInterop},
        UI::WindowsAndMessaging::{
            AdjustWindowRectEx, CREATESTRUCTW, CW_USEDEFAULT, CreateWindowExW, DefWindowProcW,
            GWL_STYLE, GWLP_USERDATA, GetClientRect, GetWindowLongPtrW, GetWindowRect,
            HWND_NOTOPMOST, HWND_TOPMOST, IDC_ARROW, LoadCursorW, PostQuitMessage, RegisterClassW,
            SW_SHOW, SWP_SHOWWINDOW, SetWindowLongPtrW, SetWindowPos, ShowWindow, WM_DESTROY,
            WM_HOTKEY, WM_LBUTTONDOWN, WM_MOUSEMOVE, WM_NCCREATE, WM_RBUTTONDOWN, WM_SIZE,
            WM_SIZING, WNDCLASSW, WS_EX_NOREDIRECTIONBITMAP, WS_OVERLAPPEDWINDOW, WS_POPUP,
        },
    },
    core::{HSTRING, Interface, PCWSTR, Result, w},
};
use windows_numerics::Vector2;

use crate::app::App;

static REGISTER_WINDOW_CLASS: Once = Once::new();
const WINDOW_CLASS_NAME: PCWSTR = w!("vidcapview.Window");

pub struct Window {
    handle: HWND,
    app: App,
    is_closed: bool,
    resized: bool,
    is_fullscreen: bool,
    last_rect: RECT,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32, app: App) -> Result<Box<Self>> {
        let instance = unsafe { GetModuleHandleW(None)? };
        REGISTER_WINDOW_CLASS.call_once(|| {
            let class = WNDCLASSW {
                hCursor: unsafe { LoadCursorW(None, IDC_ARROW).ok().unwrap() },
                hInstance: instance.into(),
                lpszClassName: WINDOW_CLASS_NAME,
                lpfnWndProc: Some(Self::wnd_proc),
                ..Default::default()
            };
            assert_ne!(unsafe { RegisterClassW(&class) }, 0);
        });

        let window_ex_style = WS_EX_NOREDIRECTIONBITMAP;
        let window_style = WS_OVERLAPPEDWINDOW;

        let (adjusted_width, adjusted_height) = {
            let mut rect = RECT {
                left: 0,
                top: 0,
                right: width as i32,
                bottom: height as i32,
            };
            unsafe {
                AdjustWindowRectEx(&mut rect, window_style, false, window_ex_style)?;
            }
            (rect.right - rect.left, rect.bottom - rect.top)
        };

        let mut result = Box::new(Self {
            handle: HWND(std::ptr::null_mut()),
            app,
            is_closed: false,
            resized: false,
            is_fullscreen: false,
            last_rect: RECT::default(),
        });

        let window = unsafe {
            CreateWindowExW(
                window_ex_style,
                WINDOW_CLASS_NAME,
                &HSTRING::from(title),
                window_style,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                adjusted_width,
                adjusted_height,
                None,
                None,
                Some(instance.into()),
                Some(result.as_mut() as *mut _ as _),
            )?
        };
        let _ = unsafe { ShowWindow(window, SW_SHOW) };

        Ok(result)
    }

    pub fn size(&self) -> Result<SizeInt32> {
        get_window_size(self.handle)
    }

    pub fn handle(&self) -> HWND {
        self.handle
    }

    pub fn create_window_target(
        &self,
        compositor: &Compositor,
        is_topmost: bool,
    ) -> Result<DesktopWindowTarget> {
        let compositor_desktop: ICompositorDesktopInterop = compositor.cast()?;
        unsafe { compositor_desktop.CreateDesktopWindowTarget(self.handle(), is_topmost) }
    }

    fn message_handler(&mut self, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        match message {
            WM_DESTROY => {
                unsafe { PostQuitMessage(0) };
                self.is_closed = true;
                return LRESULT(0);
            }
            WM_MOUSEMOVE => {
                let (x, y) = get_mouse_position(lparam);
                let point = Vector2 {
                    X: x as f32,
                    Y: y as f32,
                };
                self.app.on_pointer_moved(&point).unwrap();
            }
            WM_SIZE | WM_SIZING => {
                let new_size = self.size().unwrap();
                let new_size = Vector2 {
                    X: new_size.Width as f32,
                    Y: new_size.Height as f32,
                };
                self.app.on_parent_size_changed(&new_size).unwrap();
                self.resized = true;
            }
            WM_LBUTTONDOWN => {
                self.app.on_pointer_pressed(false, false).unwrap();
            }
            WM_RBUTTONDOWN => {
                self.app.on_pointer_pressed(true, false).unwrap();
            }
            WM_HOTKEY => {
                self.is_fullscreen = !self.is_fullscreen;
                if self.is_fullscreen {
                    unsafe { GetWindowRect(self.handle, &mut self.last_rect).unwrap() }

                    unsafe {
                        SetWindowLongPtrW(self.handle, GWL_STYLE, WS_POPUP.0 as isize);
                    }

                    let monitor =
                        unsafe { MonitorFromWindow(self.handle, MONITOR_DEFAULTTONEAREST) };
                    let mut monitor_info = MONITORINFO::default();
                    monitor_info.cbSize = std::mem::size_of_val(&monitor_info) as u32;
                    unsafe { GetMonitorInfoW(monitor, &mut monitor_info).ok().unwrap() }

                    let x = monitor_info.rcMonitor.left;
                    let y = monitor_info.rcMonitor.top;
                    let cx = monitor_info.rcMonitor.right - x;
                    let cy = monitor_info.rcMonitor.bottom - y;

                    unsafe {
                        SetWindowPos(
                            self.handle,
                            Some(HWND_TOPMOST),
                            x,
                            y,
                            cx,
                            cy,
                            SWP_SHOWWINDOW,
                        )
                        .ok()
                        .unwrap()
                    }
                } else {
                    unsafe {
                        SetWindowLongPtrW(self.handle, GWL_STYLE, WS_OVERLAPPEDWINDOW.0 as isize);
                    }

                    let x = self.last_rect.left;
                    let y = self.last_rect.top;
                    let cx = self.last_rect.right - x;
                    let cy = self.last_rect.bottom - y;

                    unsafe {
                        SetWindowPos(
                            self.handle,
                            Some(HWND_NOTOPMOST),
                            x,
                            y,
                            cx,
                            cy,
                            SWP_SHOWWINDOW,
                        )
                        .ok()
                        .unwrap()
                    }
                }
            }
            _ => {}
        }
        unsafe { DefWindowProcW(self.handle, message, wparam, lparam) }
    }

    unsafe extern "system" fn wnd_proc(
        window: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        unsafe {
            if message == WM_NCCREATE {
                let cs = lparam.0 as *const CREATESTRUCTW;
                let this = (*cs).lpCreateParams as *mut Self;
                (*this).handle = window;

                SetWindowLongPtrW(window, GWLP_USERDATA, this as _);
            } else {
                let this = GetWindowLongPtrW(window, GWLP_USERDATA) as *mut Self;

                if let Some(this) = this.as_mut() {
                    return this.message_handler(message, wparam, lparam);
                }
            }
            DefWindowProcW(window, message, wparam, lparam)
        }
    }
}

fn get_window_size(window_handle: HWND) -> Result<SizeInt32> {
    unsafe {
        let mut rect = RECT::default();
        GetClientRect(window_handle, &mut rect)?;
        let width = rect.right - rect.left;
        let height = rect.bottom - rect.top;
        Ok(SizeInt32 {
            Width: width,
            Height: height,
        })
    }
}

fn get_mouse_position(lparam: LPARAM) -> (isize, isize) {
    let x = lparam.0 & 0xffff;
    let y = (lparam.0 >> 16) & 0xffff;
    (x, y)
}
