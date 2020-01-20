#[cfg(windows)] extern crate winapi;

use std::io::Error;
use std::mem::{size_of, zeroed};

#[cfg(windows)]
use winapi::shared::minwindef::{UINT, WPARAM, LPARAM, LRESULT, LPVOID, HINSTANCE};
use winapi::shared::windef::{HWND, HMENU, HBRUSH};
use winapi::um::winnt::{LPCSTR, LPCWSTR};
use winapi::um::winuser::{WNDCLASSEXW, LoadCursorW, LoadIconW, GetMessageW, DispatchMessageW, RegisterClassExW, CreateWindowExW, ShowWindow, MessageBoxA, TranslateMessage, DefWindowProcW, PostQuitMessage}; // functions
use winapi::um::winuser::{IDI_APPLICATION, IDC_ARROW, CS_HREDRAW, CS_VREDRAW, WS_EX_TOPMOST, WS_OVERLAPPEDWINDOW, CW_USEDEFAULT, WM_DESTROY, SW_SHOWDEFAULT}; // const variable
use winapi::um::libloaderapi::GetModuleHandleA;

#[cfg(windows)]
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
            DefWindowProcW(hwnd, msg, wparam, lparam)
        }
    }
}

#[cfg(windows)]
fn print_message(msg: &str) -> Result<i32, Error> {
    let wide: Vec<u16> = to_wstring(msg);

    let ret = unsafe {
        let h_instance = GetModuleHandleA(0 as LPCSTR);

        let wndclass = WNDCLASSEXW {
            cbSize: size_of::<WNDCLASSEXW>() as u32,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hbrBackground: 16 as HBRUSH,
            hCursor: LoadCursorW(0 as HINSTANCE, IDC_ARROW), // IDC_ARROW A가 없다, 무조건 W임
            hIcon: LoadIconW(0 as HINSTANCE, IDI_APPLICATION),
            hIconSm: LoadIconW(0 as HINSTANCE, IDI_APPLICATION),
            hInstance: h_instance,
            lpfnWndProc: Some(wnd_proc), 
            lpszClassName: wide.as_ptr(),
            lpszMenuName: 0 as LPCWSTR,
            style: CS_HREDRAW | CS_VREDRAW,
        };

        let window = CreateWindowExW(
            WS_EX_TOPMOST,
            wide.as_ptr(),
            wide.as_ptr(),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT,
            0 as HWND, 0 as HMENU,
            h_instance,
            0 as *mut winapi::ctypes::c_void
        );
        
        match RegisterClassExW(&wndclass) {
            0 => {
                MessageBoxA(
                    0 as HWND,
                    b"Call to RegisterClassEx failed!\0".as_ptr() as *const i8,
                    b"Win32 Guided Tour\0".as_ptr() as *const i8,
                    0 as UINT
                );
            },
            _atom => {
                let window = CreateWindowExW(
                    WS_EX_TOPMOST,
                    wide.as_ptr(),
                    wide.as_ptr(),
                    WS_OVERLAPPEDWINDOW,
                    CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT,
                    0 as HWND, 0 as HMENU,
                    h_instance,
                    0 as LPVOID
                );
                if window.is_null() {
                    MessageBoxA(
                        0 as HWND,
                        b"Call to CreateWindow failed!\0".as_ptr() as *const i8,
                        b"Win32 Guided Tour\0".as_ptr() as *const i8,
                        0 as UINT
                    );
                } else {
                    ShowWindow(window, SW_SHOWDEFAULT);
                    let mut msg = zeroed();
                    while GetMessageW(&mut msg, 0 as HWND, 0, 0) != 0 {
                        TranslateMessage(&msg);
                        DispatchMessageW(&msg);
                    }
                }
            }
        }
    };

    // if ret == 0 { Err(Error::last_os_error()) }
    // else { Ok(ret) }

    return Ok(0);
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