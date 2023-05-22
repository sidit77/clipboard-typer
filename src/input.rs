use std::mem::size_of;

use windows_sys::Win32::UI::Input::KeyboardAndMouse::*;

use crate::clipboard::U16String;
use crate::ensure;
use crate::error::{WinError, WinResult};

const CR: u16 = b'\r' as u16;
const LF: u16 = b'\n' as u16;

pub fn type_string(string: &U16String) -> WinResult<()> {
    for character in string {
        match character {
            CR => {}
            LF => send_input(&enter())?,
            c => send_input(&convert_char(c))?
        }
    }
    Ok(())
}

fn send_input(inputs: &[INPUT]) -> WinResult<()> {
    let sent = unsafe { SendInput(inputs.len() as _, inputs.as_ptr(), size_of::<INPUT>() as _) as usize };
    ensure!(sent == inputs.len(), WinError::last_error());
    Ok(())
}

const fn enter() -> [INPUT; 2] {
    [make_vk_input(VK_RETURN, true), make_vk_input(VK_RETURN, false)]
}

fn convert_char(ch: u16) -> [INPUT; 2] {
    [make_char_input(ch, true), make_char_input(ch, false)]
}

const fn make_char_input(ch: u16, pressed: bool) -> INPUT {
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: 0,
                wScan: ch,
                dwFlags: match pressed {
                    true => KEYEVENTF_UNICODE,
                    false => KEYEVENTF_UNICODE | KEYEVENTF_KEYUP
                },
                time: 0,
                dwExtraInfo: 0
            }
        }
    }
}

const fn make_vk_input(vk: VIRTUAL_KEY, pressed: bool) -> INPUT {
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: vk,
                wScan: 0,
                dwFlags: match pressed {
                    true => 0,
                    false => KEYEVENTF_KEYUP
                },
                time: 0,
                dwExtraInfo: 0
            }
        }
    }
}
