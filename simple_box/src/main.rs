#[cfg(windows)] extern crate winapi;
#[cfg(windows)] extern crate kernel32;
#[cfg(windows)] extern crate user32;

use std::io::Error;

use winapi::shared::minwindef::{UINT, WPARAM, LPARAM, LRESULT, LPVOID, HINSTANCE};
use winapi::shared::windef::{HWND, HMENU, HICON, HCURSOR, HBRUSH};
use winapi::um::wingdi::{GetStockObject, WHITE_BRUSH};
use winapi::um::winnt::{LPCSTR, LPCWSTR};
use winapi::um::winuser::{WNDPROC, MSG, TranslateMessage, WNDCLASSA, RegisterClassA, LoadCursorA, LoadIconA, GetMessageA, DispatchMessageA}; // functions
use winapi::um::winuser::{IDI_APPLICATION, IDC_ARROW, CS_HREDRAW, CS_VREDRAW, WS_EX_TOPMOST, WS_OVERLAPPEDWINDOW, CW_USEDEFAULT}; // const variable
use kernel32::{GetModuleHandleA};
use user32::{CreateWindowExA, ShowWindow}; // WinMain
use user32::{DefWindowProcA, PostQuitMessage, BeginPaint}; // WndProc

fn to_wstring(str : &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    let v : Vec<u16> =
            OsStr::new(str).encode_wide().chain(once(0).into_iter()).collect();
    v
}

#[cfg(windows)]
unsafe extern "system"
fn wnd_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_DESTROY => {
            PostQuitMessage(0); 0
        },
        _ => {
            DefWindowProcA(hwnd, msg, wparam, lparam) // A쓰지마
        }
    }
}

#[cfg(windows)]
fn print_message(msg: &str) -> Result<i32, Error> {
    let wide: Vec<u16> = to_wstring(msg);

    let ret = unsafe {
        // use winapi::um::winuser::{MB_OK, MessageBoxW};
        // use std::ptr::null_mut;
        // MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), MB_OK)

        let wndclass = WNDCLASSA {
            cbClsExtra: 0,
            cbWndExtra: 0,
            hbrBackground: 16 as HBRUSH,
            hCursor: LoadCursorA(0 as HINSTANCE, IDC_ARROW), // IDC_ARROW A가 없다, 무조건 W임
            hIcon: LoadIconA(0 as HINSTANCE, winapi::winuser::IDI_APPLICATION),
            hInstance: 0 as HINSTANCE,
            lpfnWndProc: Some(window_proc), 
            lpszClassName: class_name.as_ptr(),
            lpszMenuName: 0 as LPCSTR,
            style: CS_HREDRAW | CS_VREDRAW,
        };
    };

    if ret == 0 { Err(Error::last_os_error()) }
    else { Ok(ret) }
}

#[cfg(not(windows))]
fn print_message(msg: &str) -> Result<(), Error> {
    println!("{}", msg);
    Ok(())
}

fn main() {
    // https://docs.rs/winapi/0.3.0/winapi/um/winuser/index.html
    // https://github.com/retep998/winapi-rs/issues/122
    // https://www.codeproject.com/Tips/1053658/Win-GUI-Programming-In-Rust-Language
    print_message("Hello, world with rust winapi crate!").unwrap();
}