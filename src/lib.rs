extern crate winapi;

use std::ffi::CStr;
use std::io::{ self, Write };
use std::ptr;
use std::slice;
use winapi::um::consoleapi::AllocConsole;
use winapi::um::consoleapi::ReadConsoleW;
use winapi::um::consoleapi::WriteConsoleW;
use winapi::um::processenv::GetStdHandle;
use winapi::um::winbase::{ STD_INPUT_HANDLE, STD_OUTPUT_HANDLE };
use winapi::um::wincon::{
    SetConsoleOutputCP,
    SetConsoleScreenBufferSize,
    SetConsoleTitleW,
    SetConsoleWindowInfo,
    COORD,
};
use winapi::um::winnls::{ GetLocaleInfoA, GetUserDefaultLCID };

#[no_mangle]
pub extern "C" fn vyvolatKonzoli() {
    unsafe {
        if AllocConsole() == 0 {
            return;
        }

        let title = "[crashfex]";
        let wide_title: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();
        SetConsoleTitleW(wide_title.as_ptr());

        let lcid = GetUserDefaultLCID();
        let mut cp = [0i8; 7];

        if GetLocaleInfoA(lcid, 0x00001004, cp.as_mut_ptr(), cp.len() as i32) != 0 {
            let cp_str = CStr::from_ptr(cp.as_ptr()).to_string_lossy();

            if let Ok(code_page) = cp_str.parse::<u32>() {
                SetConsoleOutputCP(code_page);
            }
        }

        let console_handle = GetStdHandle(STD_OUTPUT_HANDLE);

        let buffer_size = COORD { X: 85, Y: 15 };
        SetConsoleScreenBufferSize(console_handle, buffer_size);

        // velikost okna w:80, h:15
        let window_size = winapi::um::wincon::SMALL_RECT {
            Left: 0,
            Top: 0,
            Right: 84,
            Bottom: 14,
        };
        SetConsoleWindowInfo(console_handle, 1, &window_size);

        let text =
            "[1]   access out-of-bounds array index\n[2]   terminate the program\n[3]   exit the program\n>_ ";

        let text_wide: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
        let mut num_chars_written: u32 = 0;
        WriteConsoleW(
            GetStdHandle(STD_OUTPUT_HANDLE),
            text_wide.as_ptr() as *const _,
            (text_wide.len() as u32) - 1,
            &mut num_chars_written,
            ptr::null_mut()
        );

        // precist a zpracovat input
        let mut buffer = [0u16; 256];
        loop {
            let mut num_chars_read: u32 = 0;
            let success = ReadConsoleW(
                GetStdHandle(STD_INPUT_HANDLE),
                buffer.as_mut_ptr() as *mut _,
                buffer.len() as u32,
                &mut num_chars_read,
                ptr::null_mut()
            );
            if success == 0 {
                return;
            }

            // prevadeni bufferu to stringu
            let input = slice
                ::from_raw_parts(buffer.as_ptr() as *const u8, (num_chars_read as usize) * 2)
                .iter()
                .map(|&b| b as char)
                .take_while(|&c| c != '\0')
                .collect::<String>()
                .trim()
                .to_string();

            match input.as_str() {
                "1" => function1(),
                "2" => function2(),
                "3" => function3(),
                _ => {
                    let msg = "invalid input.\n>_ ";
                    let msg_wide: Vec<u16> = msg.encode_utf16().chain(std::iter::once(0)).collect();
                    WriteConsoleW(
                        GetStdHandle(STD_OUTPUT_HANDLE),
                        msg_wide.as_ptr() as *const _,
                        (msg_wide.len() as u32) - 1,
                        &mut num_chars_written,
                        ptr::null_mut()
                    );
                }
            }

            io::stdout().flush().unwrap();
        }
    }
}

#[allow(unconditional_panic)]
fn function1() {
    let arr = [1, 2, 3];
    unsafe {
        println!("{}", *arr.get_unchecked(10));
    }
}

fn function2() {
    print!("terminating...");
    std::process::abort();
}

fn function3() {
    print!("exiting...");
    std::process::exit(0);
}

#[no_mangle]
pub extern "C" fn DllMain(
    _: *mut winapi::ctypes::c_void,
    reason: u32,
    _: *mut winapi::ctypes::c_void
) -> i32 {
    const DLL_PROCESS_ATTACH: u32 = 1;

    if reason == DLL_PROCESS_ATTACH {
        //  vola funkci vyvolatKonzoli() po injectu
        vyvolatKonzoli();
    }

    1 // oznacuje uspesny inject
}
